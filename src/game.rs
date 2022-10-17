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
}

const TOP_LEFT_CELL: Position = Position{column: 0_usize, row: 0_usize};

impl Game {
    pub fn create(size: usize) -> Self {
        let grid = Grid::generate(size);
        let graph = Graph::create(&grid);
        Self {
            state: GameState::Solving,
            graph,
            number_of_clicks: 0,
        }
    }

    pub fn fill_component_of_top_left_cell_with(&mut self, colour: Colour) {
        self.number_of_clicks += 1;
        self.graph.change_colour_of_component_at(&TOP_LEFT_CELL, colour);
    }
}
