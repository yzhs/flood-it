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

        let rendering_size = screen_width().min(screen_height());

        draw_rectangle((screen_width() - rendering_size) / 2.0, (screen_height() - rendering_size) / 2.0, rendering_size, rendering_size, GREEN);

        next_frame().await
    }
}
