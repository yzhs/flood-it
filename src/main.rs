use macroquad::prelude::*;

use grid::Grid;
use graph::Graph;

mod colour;
mod grid;
mod graph;

#[macroquad::main("BasicShapes")]
async fn main() {
    let grid = Grid::generate(8);
    let mut graph = Graph::create(&grid);

    loop {
        clear_background(BLACK);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
