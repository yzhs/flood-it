#![feature(rand)]

#[macro_use]
extern crate glium;
extern crate rand;

use std::collections::{HashSet, VecDeque};

use glium::Surface;
use glium::glutin;


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

impl rand::Rand for Color {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        *rng.choose(COLORS).unwrap()
    }
}

struct Grid {
    width: u8,
    height: u8,

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
}


#[derive(Copy, Clone)]
struct Vertex {
    color: f32,
    position: [f32; 2],
}

implement_vertex!(Vertex, color, position);

const TITLE: &'static str = "Flood-It";

fn main() {
    use glium::glutin::{ContextBuilder, EventsLoop, WindowBuilder};
    use glutin::{Event, WindowEvent};

    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new().with_title(TITLE);
    let context = ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        in float color;
        uniform mat4 matrix;
        out float c;
        void main() {
            c = color;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in float c;
        out vec4 color;
        void main() {
            if (c <= 0.0)
                color = vec4(1.0, 0.0, 0.0, 1.0);
            else if (c <= 1.0)
                color = vec4(1.0, 1.0, 0.0, 1.0);
            else if (c <= 2.0)
                color = vec4(0.0, 0.69, 0.0, 1.0);
            else if (c <= 3.0)
                color = vec4(1.0, 0.8, 0.4, 1.0);
            else if (c <= 4.0)
                color = vec4(0.5, 0.0, 0.5, 1.0);
            else if (c <= 5.0)
                color = vec4(0.0, 1.0, 1.0, 1.0);
            else if (c <= 6.0)
                color = vec4(0.0, 0.0, 1.0, 1.0);
            else if (c <= 7.0)
                color = vec4(1.0, 0.0, 1.0, 1.0);
            else
                color = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let params = glium::DrawParameters {
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
        ..Default::default()
    };

    let mut grid = Grid::new(6, 14);

    let mut cursor_position = (0.0, 0.0);
    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        let (width, height) = target.get_dimensions();
        let screen_aspect_ratio = height as f32 / width as f32;
        let grid_aspect_ratio = grid.height as f32 / grid.width as f32;

        let (ratio_x, ratio_y) = if screen_aspect_ratio < grid_aspect_ratio {
            (screen_aspect_ratio, 1.0)
        } else {
            (1.0, 1.0 / screen_aspect_ratio)
        };

        let uniforms =
            uniform! {
            matrix: [
                [ratio_x, 0.0, 0.0, 0.0],
                [0.0, -ratio_y, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0_f32],
            ]
        };

        let mut triangles = vec![];

        for &color in COLORS.iter() {
            for i in 0..grid.cells.len() {
                let x = (i % grid.width as usize) as f32;
                let y = (i / grid.width as usize) as f32;
                if color != grid.cells[i] {
                    continue;
                }

                let cell_size = 2.0 / grid.height.max(grid.width) as f32;

                let mut quad = generate_rectangle_vertices(
                    x * cell_size - 1.0,
                    y * cell_size - 1.0,
                    (x + 1.0) * cell_size - 1.0,
                    (y + 1.0) * cell_size - 1.0,
                    color,
                );
                triangles.append(&mut quad);
            }
            let vertex_buffer = glium::VertexBuffer::new(&display, &triangles).unwrap();
            target
                .draw(&vertex_buffer, &indices, &program, &uniforms, &params)
                .unwrap();
        }

        target.finish().unwrap();

        events_loop.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::Closed => closed = true,
                    WindowEvent::MouseMoved { position, .. } => cursor_position = position,
                    WindowEvent::MouseInput {
                        state: glium::glutin::ElementState::Released,
                        button: glium::glutin::MouseButton::Left,
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
                                        println!("You win! You used {} out of {} available moves.", grid.num_clicks, grid.max_clicks);
                                    } else {
                                        println!("You lose. You took {} moves but should have finished in {}.", grid.num_clicks, grid.max_clicks);
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        });
    }
}

fn generate_rectangle_vertices(
    left: f32,
    bottom: f32,
    right: f32,
    top: f32,
    color: Color,
) -> Vec<Vertex> {
    let color = color as u32 as f32;
    vec![
        Vertex {
            color,
            position: [left, bottom],
        },
        Vertex {
            color,
            position: [right, bottom],
        },
        Vertex {
            color,
            position: [left, top],
        },
        Vertex {
            color,
            position: [right, bottom],
        },
        Vertex {
            color,
            position: [right, top],
        },
        Vertex {
            color,
            position: [left, top],
        },
    ]
}
