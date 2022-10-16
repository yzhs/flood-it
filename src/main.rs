use macroquad::prelude::*;

use colour::Colour;
use graph::{Graph, Position};
use game::{Game, GameState};

mod colour;
mod graph;
mod grid;
mod game;

fn macroquad_colour(colour: Colour) -> Color {
    match colour {
        Colour::Red => color_u8!(255, 0, 0, 255),
        Colour::Yellow => color_u8!(255, 255, 0, 255),
        Colour::Green => color_u8!(0, 176, 0, 255),
        Colour::LightBrown => color_u8!(255, 204, 102, 255),
        Colour::Purple => color_u8!(128, 0, 128, 255),
        Colour::Cyan => color_u8!(0, 255, 255, 255),
        Colour::Blue => color_u8!(0, 0, 255, 255),
        Colour::Fuchsia => color_u8!(255, 0, 255, 255),
    }
}

fn draw_cell(grid_x: f32, grid_y: f32, cell_size: f32, position: &Position, colour: Colour) {
    let color = macroquad_colour(colour);
    draw_rectangle(
        grid_x + cell_size * position.column as f32,
        grid_y + cell_size * position.row as f32,
        cell_size,
        cell_size,
        color,
    );
}

struct Ui {
    /// Size of the grid in cells.
    size: usize,

    /// Size of the grid in pixels.  Assumes the grid is always square.
    grid_size: f32,

    /// Offset of the grid relative to the whole window along the horizontal axis.
    grid_x: f32,

    /// Offset of the grid relative to the whole window along the vertical axis.
    grid_y: f32,
}

impl Ui {
    fn create(size: usize) -> Ui {
        let mut ui = Ui {
            size,
            grid_size: 0.0,
            grid_x: 0.0,
            grid_y: 0.0,
        };

        ui.resize();

        ui
    }

    fn resize(&mut self) {
        let screen_height = screen_height();
        let screen_width = screen_width();

        let grid_size = screen_height.min(screen_width);
        self.grid_size = grid_size;
        self.grid_x = (screen_width - grid_size) / 2.0;
        self.grid_y = (screen_height - grid_size) / 2.0;
    }

    fn render(&self, game: &Game) {
        clear_background(BLACK);

        let grid_x = self.grid_x;
        let grid_y = self.grid_y;
        let cell_size = self.cell_size();

        let graph = &game.graph;
        for component in graph.neighbours.keys() {
            for position in &graph.components[component].cells {
                draw_cell(grid_x, grid_y, cell_size, position, graph.components[component].colour);
            }
        }
    }

    fn cell_size(&self) -> f32 {
        self.grid_size / self.size as f32
    }

    fn cell_position(&self, raw_position: (f32, f32)) -> Option<Position> {
        let (raw_x, raw_y) = raw_position;

        let cell_size = self.cell_size();
        let x = (raw_x - self.grid_x) / cell_size;
        let y = (raw_y - self.grid_y) / cell_size;

        let size = self.size as f32;
        if x < 0.0 || y < 0.0 || x > size || y > size {
            // Out of bounds, nothing to do
            None
        } else {
            Some(Position{column: x.floor() as usize, row: y.floor() as usize})
        }
    }

    fn handle_click(&self, game: &mut Game) {
        if !is_mouse_button_pressed(MouseButton::Left) {
            return;
        }

        if let Some(position) = self.cell_position(mouse_position()) {
            let clicked_component = game.graph.find_component(&position);
            let colour = clicked_component.colour;

            println!("Detected click at cell ({}, {}) with colour {:#?}", position.column, position.row, colour);

            game.fill_component_of_top_left_cell_with(colour);

            match game.state {
                GameState::Solving =>
                if game.graph.components.len() == 1 {
                    game.state = GameState::Solved;
                    println!("Done");
                }
                GameState::Solved => (),
            }
        }
    }
}

#[macroquad::main("Flood-It")]
async fn main() {
    let size = 8;

    let mut game = Game::create(size);
    let mut ui = Ui::create(size);

    loop {
        if let Some(KeyCode::Q) = get_last_key_pressed() {
            break;
        }

        ui.handle_click(&mut game);

        ui.resize();
        ui.render(&game);

        next_frame().await
    }
}
