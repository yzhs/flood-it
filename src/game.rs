use crate::graph::Graph;
use crate::grid::Grid;

pub enum GameState {
    Solving,
    Solved,
}

pub struct Game {
    pub state: GameState,
    pub graph: Graph,
}

impl Game {
    pub fn create(size: usize) -> Self {
        let grid = Grid::generate(size);
        let graph = Graph::create(&grid);
        Self {
            state: GameState::Solving,
            graph,
        }
    }
}
