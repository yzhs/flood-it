use crate::colour::Colour;
use crate::graph::{Graph, Position};
use crate::grid::Grid;

pub enum GameState {
    Solving,
    Solved,
}

pub struct Game {
    pub state: GameState,
    pub graph: Graph,
    number_of_clicks: u32,
    allowed_clicks: u32,
}

const TOP_LEFT_CELL: Position = Position{column: 0_usize, row: 0_usize};

impl Game {
    pub fn create(size: u32) -> Self {
        let num_colours = 4;
        let grid = Grid::generate(size as usize);
        let graph = Graph::create(&grid);

        let allowed_clicks = 25 * 2 * size * num_colours / (2 * 14 * 6);

        Self {
            state: GameState::Solving,
            graph,
            number_of_clicks: 0,
            allowed_clicks,
        }
    }

    pub fn fill_component_of_top_left_cell_with(&mut self, colour: Colour) {
        self.number_of_clicks += 1;
        self.graph.change_colour_of_component_at(&TOP_LEFT_CELL, colour);
    }
}
