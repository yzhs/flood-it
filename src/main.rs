extern crate clap;
#[macro_use]
extern crate glium;
extern crate rand;

use std::borrow::Cow;
use std::collections::{HashSet, VecDeque};

use glium::Surface;
use glium::glutin::{ContextBuilder, WindowBuilder, EventsLoop};
use glium::glutin::{Event, WindowEvent, ElementState, KeyboardInput, MouseButton, VirtualKeyCode};
use glium::texture::{RawImage2d, Texture2d, Texture2dDataSink};


/// Maximum frames per second.
const FPS: u32 = 30;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    Red,
    Yellow,
    Green,
    LightBrown,
    Purple,
    Cyan,
    Blue,
    Fuchsia,
}

const COLORS: &[Color] = &[
    Color::Red,
    Color::Yellow,
    Color::Green,
    Color::LightBrown,
    Color::Purple,
    Color::Cyan,
    Color::Blue,
    Color::Fuchsia,
];

impl Color {
    fn to_rgb(self) -> (u8, u8, u8) {
        match self {
            Color::Red => (255, 0, 0),
            Color::Yellow => (255, 255, 0),
            Color::Green => (0, 176, 0),
            Color::LightBrown => (255, 204, 102),
            Color::Purple => (128, 0, 128),
            Color::Cyan => (0, 255, 255),
            Color::Blue => (0, 0, 255),
            Color::Fuchsia => (255, 0, 255),
        }
    }
}

impl rand::Rand for Color {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        *rng.choose(COLORS).unwrap()
    }
}

struct Grid {
    width: u8,
    height: u8,
    num_colors: u8,

    cells: Vec<Color>,
    population: Vec<u16>,

    max_clicks: u16,
    num_clicks: u16,
}

impl Grid {
    pub fn new(num_colors: u8, size: u8) -> Self {
        use rand::distributions::{IndependentSample, Range};
        let between = Range::new(0, num_colors as usize);
        let mut rng = rand::thread_rng();

        let cells: Vec<Color> = (0..size as u64 * size as u64)
            .map(|_| COLORS[between.ind_sample(&mut rng)])
            .collect();
        let max_clicks = 25 * 2 * size as u16 * num_colors as u16 / (2 * 14 * 6);
        let mut population = vec![0; COLORS.len()];
        for &c in &cells {
            population[c as usize] += 1;
        }

        Self {
            width: size,
            height: size,
            num_colors,

            cells,
            population,

            max_clicks,
            num_clicks: 0,
        }
    }

    fn index(&self, row: u8, column: u8) -> usize {
        self.width as usize * row as usize + column as usize
    }

    fn current_color(&self) -> Color {
        self.cells[0]
    }

    fn solved(&self) -> bool {
        let mut colors_present = 0;
        for &x in &self.population {
            if x > 0 {
                colors_present += 1;
            }
        }
        colors_present == 1
    }

    pub fn click(&mut self, row: u8, column: u8) -> bool {
        if row >= self.height || column >= self.width {
            return false;
        }

        let i = self.index(row, column);
        let new_color = self.cells[i];
        if self.current_color() == new_color {
            return false;
        }

        self.num_clicks += 1;
        self.flood(new_color);
        self.solved()
    }

    /// Flood the grid from the top left cell and return the number of cell which are connected to
    /// the top left cell by cells of indentical color.
    fn flood(&mut self, new_color: Color) {
        let current_color = self.current_color();

        let rows = self.height as usize;
        let columns = self.width as usize;

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(0);

        while let Some(i) = queue.pop_front() {
            if self.cells[i] == new_color {
                continue;
            }
            let c = self.cells[i];
            self.population[c as usize] -= 1;
            self.cells[i] = new_color;
            self.population[new_color as usize] += 1;
            visited.insert(i);

            if i % columns > 0 && self.cells[i - 1] == current_color &&
                !visited.contains(&(i - 1))
            {
                queue.push_back(i - 1);
            }
            if i % columns < columns - 1 && self.cells[i + 1] == current_color &&
                !visited.contains(&(i + 1))
            {
                queue.push_back(i + 1);
            }
            if i >= columns && self.cells[i - columns] == current_color &&
                !visited.contains(&(i - columns))
            {
                queue.push_back(i - columns);
            }
            if i < (rows - 1) * columns && self.cells[i + columns] == current_color &&
                !visited.contains(&(i + columns))
            {
                queue.push_back(i + columns);
            }
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.height as f32 / self.width as f32
    }

    pub fn reset(&mut self) {
        use rand::distributions::{IndependentSample, Range};
        let between = Range::new(0, self.num_colors as usize);
        let mut rng = rand::thread_rng();

        self.cells.iter_mut().for_each(|x| {
            *x = COLORS[between.ind_sample(&mut rng)]
        });

        self.population = vec![0; COLORS.len()];
        for &c in &self.cells {
            self.population[c as usize] += 1;
        }
        self.num_clicks = 0;
    }
}


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

    uniform vec2 size;
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
        let tmp: u8 = matches
            .value_of("colors")
            .and_then(|x| x.parse().ok())
            .unwrap_or(6);
        if tmp < 3 || tmp > 8 {
            panic!("Flood-It only supports 3 through 8 (inclusive) colors.");
        } else {
            tmp
        }
    };
    let size: u8 = {
        let tmp: u8 = matches
            .value_of("size")
            .and_then(|x| x.parse().ok())
            .unwrap_or(14);
        if tmp < 2 {
            panic!("Flood-It needs a grid of at least 2x2 cells.");
        } else {
            tmp
        }
    };

    (colors, size)
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

    let rect = generate_rectangle_vertices(-1.0, -1.0, 1.0, 1.0);
    let vertex_buffer = glium::VertexBuffer::new(&display, &rect).unwrap();

    main_loop(|| {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        let (width, height) = target.get_dimensions();
        let screen_aspect_ratio = height as f32 / width as f32;

        let (ratio_x, ratio_y) = if screen_aspect_ratio < grid_aspect_ratio {
            (screen_aspect_ratio, 1.0)
        } else {
            (1.0, screen_aspect_ratio.recip())
        };

        // Create a texture representation of the colored grid
        let cell_colors: Vec<_> = grid.cells.iter().map(|x| x.to_rgb()).collect();
        let cell_image = RawImage2d::from_raw(
            Cow::from(cell_colors),
            grid.width as u32,
            grid.height as u32,
        );
        let cell_texture = Texture2d::new(&display, cell_image).unwrap();

        let uniforms =
            uniform! {
                size: (grid.width as f32, grid.height as f32),
                colors: cell_texture.sampled()
                    //.wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                matrix: [
                    [ratio_x, 0.0, 0.0, 0.0],
                    [0.0, -ratio_y, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0_f32],
                ]
        };
        target
            .draw(&vertex_buffer, &indices, &program, &uniforms, &params)
            .unwrap();

        target.finish().unwrap();

        let mut closed = false;
        events_loop.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::Closed => closed = true,

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
                        let offsets = {
                            let offset = (width as f64 - height as f64).abs() / 2.0;
                            if screen_aspect_ratio < grid_aspect_ratio {
                                (offset, 0.0)
                            } else {
                                (0.0, offset)
                            }
                        };
                        if cursor_position.0 - offsets.0 >= 0.0 &&
                            cursor_position.1 - offsets.1 >= 0.0
                        {
                            let column = ((cursor_position.0 - offsets.0) * grid.width as f64 /
                                              (width as f64 - 2.0 * offsets.0))
                                .floor() as u8;
                            let row = ((cursor_position.1 - offsets.1) * grid.height as f64 /
                                           (height as f64 - 2.0 * offsets.1))
                                .floor() as u8;
                            if grid.click(row, column) {
                                if grid.num_clicks <= grid.max_clicks {
                                    println!(
                                        "You win! You used {} out of {} available moves.",
                                        grid.num_clicks,
                                        grid.max_clicks
                                    );
                                } else {
                                    println!(
                                        "You lose. You took {} moves but should have \
                                                 finished in {}.",
                                        grid.num_clicks,
                                        grid.max_clicks
                                    );
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        });
        !closed
    });
}

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

    let one_frame = Duration::new(0, 10 ^ 9 / FPS + 1);
    let mut last_frame = Instant::now();

    while callback() {
        let now = Instant::now();
        let remaining = one_frame.checked_sub(now - last_frame).unwrap_or_default();
        last_frame = now;
        thread::sleep(remaining);
    }
}
