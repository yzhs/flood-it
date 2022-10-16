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
}

const top_left_cell: Position = Position{column: 0_usize, row: 0_usize};

impl Game {
    pub fn create(size: usize) -> Self {
        let grid = Grid::generate(size);
        let graph = Graph::create(&grid);
        Self {
            state: GameState::Solving,
            graph,
        }
    }

    pub fn fill_component_of_top_left_cell_with(&mut self, colour: Colour) {
        self.graph.change_colour_of_component_at(&top_left_cell, colour);
    }
}
