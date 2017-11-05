extern crate clap;
#[macro_use]
extern crate glium;
extern crate rand;

mod grid;
mod screen;
mod types;

use glium::Surface;
use glium::glutin::{ContextBuilder, WindowBuilder, EventsLoop};
use glium::glutin::{Event, WindowEvent, ElementState, KeyboardInput, MouseButton, VirtualKeyCode};

use grid::*;
use screen::*;

/// Maximum frames per second.
const FPS: u32 = 30;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

const TITLE: &str = "Flood-It";

const VERTEX_SHADER: &str = r#"
    #version 140

    in  vec2 position;
    out vec2 v_tex_coords;

    uniform mat4 matrix;

    void main() {
        v_tex_coords = 0.5 * (position + vec2(1.0, 1.0));
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
"#;

/// Map Color::* to RGB values.
const FRAGMENT_SHADER: &str = r#"
    #version 140

    in  vec2 v_tex_coords;
    out vec4 color;

    uniform sampler2D colors;

    void main() {
        color = texture(colors, v_tex_coords);
    }
"#;

fn generate_rectangle_vertices(left: f32, bottom: f32, right: f32, top: f32) -> Vec<Vertex> {
    vec![
        Vertex { position: [left, bottom] },
        Vertex { position: [right, bottom] },
        Vertex { position: [left, top] },
        Vertex { position: [right, bottom] },
        Vertex { position: [right, top] },
        Vertex { position: [left, top] },
    ]
}

fn main_loop<F: FnMut() -> bool>(mut callback: F) {
    use std::time::{Duration, Instant};
    use std::thread;

    let one_frame = Duration::new(0, 1_000_000_000 / FPS + 1);
    let mut last_frame = Instant::now();

    while callback() {
        let now = Instant::now();
        let remaining = one_frame.checked_sub(now - last_frame).unwrap_or_default();
        last_frame = now;
        thread::sleep(remaining);
    }
}

fn main() {
    let (colors, size) = parse_args();

    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new().with_title(TITLE);
    let context = ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let program = glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None)
        .unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let params = glium::DrawParameters {
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
        ..Default::default()
    };

    let mut grid = Grid::new(colors, size);
    let grid_aspect_ratio = grid.aspect_ratio();

    let mut cursor_position = (0.0, 0.0);

    let rect = {
        let tmp = generate_rectangle_vertices(-1.0, -1.0, 1.0, 1.0);
        glium::VertexBuffer::new(&display, &tmp).unwrap()
    };

    let mut screen = ScreenInfo::dummy();
    let mut cell_texture = grid.render(&display);
    let mut init = true;

    main_loop(|| {
        /*
         * Rendering
         */
        {
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            let (width, height) = target.get_dimensions();
            if init {
                screen.resize(width, height, grid_aspect_ratio);
                init = false;
            }

            let uniforms =
                uniform! {
                    colors: cell_texture.sampled()
                        //.wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                        matrix: screen.matrix,
                };
            target
                .draw(&rect, &indices, &program, &uniforms, &params)
                .unwrap();

            target.finish().unwrap();
        }

        /*
         * Handle events
         */
        let mut closed = false;
        events_loop.poll_events(|event| {
            let mut changed = false;
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::Closed => closed = true,
                    WindowEvent::Resized(width, height) => {
                        screen.resize(width, height, grid_aspect_ratio);
                    }

                    WindowEvent::MouseMoved { position, .. } => cursor_position = position,

                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            virtual_keycode: Some(key),
                            state: ElementState::Released,
                            ..
                        },
                        ..
                    } => {
                        match key {
                            VirtualKeyCode::Space | VirtualKeyCode::N | VirtualKeyCode::R => {
                                if grid.solved() {
                                    grid.reset();
                                    changed = true;
                                }
                            }
                            VirtualKeyCode::Q => closed = true,
                            _ => (),
                        }
                    }

                    WindowEvent::MouseInput {
                        state: ElementState::Released,
                        button: MouseButton::Left,
                        ..
                    } => {
                        screen
                            .cursor_to_grid_coords(cursor_position.into(), grid.size())
                            .map(|types::Position(column, row)| {
                                changed = true;
                                if grid.click(row, column) {
                                    let num = grid.num_clicks().0;
                                    let max = grid.max_clicks().0;
                                    if grid.won() {
                                        println!(
                                            "You win! You used {} out of {} available moves.",
                                            num,
                                            max
                                        );
                                    } else {
                                        println!(
                                            "You lose. You took {} moves but should have \
                                                 finished in {}.",
                                            num,
                                            max
                                        );
                                    }
                                }
                            });
                    }
                    _ => (),
                }
            }
            if changed {
                cell_texture = grid.render(&display);
            }
        });
        !closed
    });
}

/// Handle command line arguments
fn parse_args() -> (u8, u8) {
    use clap::{App, Arg};

    let matches = App::new(TITLE)
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("colors")
                .takes_value(true)
                .help("The number of different colors")
                .default_value("6"),
        )
        .arg(
            Arg::with_name("size")
                .takes_value(true)
                .help("The height and width of the grid")
                .default_value("14"),
        )
        .get_matches();

    let colors = {
        let tmp: usize = matches.value_of("colors").unwrap().parse().expect(
            "Invalid number of colors",
        );
        if tmp < 3 || tmp > COLORS.len() {
            panic!(
                "Flood-It only supports 3 through {} (inclusive) colors.",
                COLORS.len()
            );
        } else {
            tmp as u8
        }
    };

    let size: u8 = {
        let tmp: u8 = matches.value_of("size").unwrap().parse().expect(
            "Invalid grid size",
        );
        if tmp < 2 {
            panic!("Flood-It needs a grid of at least 2x2 cells.");
        } else {
            tmp
        }
    };

    (colors, size)
}
