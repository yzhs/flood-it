use macroquad::prelude::*;

use colour::Colour;
use graph::{Graph, Position};
use grid::Grid;

mod colour;
mod graph;
mod grid;

fn macroquad_colour(colour: Colour) -> Color {
    match colour {
        Colour::Red => RED,
        Colour::Yellow => YELLOW,
        Colour::Green => GREEN,
        Colour::LightBrown => LIGHTGRAY,
        Colour::Purple => PURPLE,
        Colour::Cyan => PINK,
        Colour::Blue => BLUE,
        Colour::Fuchsia => MAGENTA,
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

#[macroquad::main("BasicShapes")]
async fn main() {
    let size = 8;
    let grid = Grid::generate(size);
    let mut graph = Graph::create(&grid);

    loop {
        clear_background(BLACK);

        let grid_size = screen_width().min(screen_height());
        let grid_x = (screen_width() - grid_size) / 2.0;
        let grid_y = (screen_height() - grid_size) / 2.0;
        let cell_size = grid_size / size as f32;

        for component in graph.neighbours.keys() {
            for position in &component.cells {
                draw_cell(grid_x, grid_y, cell_size, position, component.colour);
            }
        }

        next_frame().await
    }
}
