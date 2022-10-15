use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

use crate::colour::Colour;
use crate::grid::Grid;

// Represent the state of the game as a graph.  One node for each cell.  There is an edge between
// two cells if and only if they are the same colour and have a distance of 1 according to the
// Manhattan metric.
//
// The game has only one real operation (i.e. not counting level generation): Switching the colour
// of the connected component containing the top left cell to whichever colour the clicked cell
// has.  This results in said component growing by every component it neighbours which has its new
// colour.  In essence, the aim of the gaim is to join connected components until only one is
// left.  Fewer clicks is better.
//
// So, we represent the game state as a collection of connected components, each of which has a
// colour, a list of coordinates of the constituent cells, and a list of it s neighbours.  As we
// do all our operations on connected components, we actually work not with the graph of
// neighbouring cells but with a graph of connected components.  The graph of connected components
// is made up of one node representing each connected component and an edge between any two
// connected components that have cells which share an edge, i.e. two cells which would have been
// connected by an edge in the original graph if they had had the same colour (which they did
// not).

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct ConnectedComponent {
    id: usize,
    pub colour: Colour,
    pub cells: HashSet<Position>,
}

impl PartialEq for ConnectedComponent {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ConnectedComponent {}

impl Hash for ConnectedComponent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug)]
pub struct Graph {
    pub components: HashMap<usize, ConnectedComponent>,
    pub neighbours: HashMap<usize, HashSet<usize>>,
}

fn find_connected_components(grid: &Grid) -> Vec<(ConnectedComponent, HashSet<usize>)> {
    let rows = grid.number_of_rows;
    let columns = grid.number_of_columns;

    let mut remaining_cells: HashSet<usize> = (0..grid.cells.len()).into_iter().collect();

    let mut components = vec![];
    let mut counter = 0;

    // Take any element to land in any component.
    while let Some(&start_cell_index) = remaining_cells.iter().next() {
        let mut visited = HashSet::new();
        let mut neighbours = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(start_cell_index);

        // Gather all cells in the connected component of start_cell_index.
        while let Some(i) = queue.pop_front() {
            visited.insert(i);
            remaining_cells.remove(&i);

            let mut neighbouring_cells = vec![];
            if i % columns > 0 {
                neighbouring_cells.push(i - 1);
            }
            if i % columns < columns - 1 {
                neighbouring_cells.push(i + 1);
            }
            if i >= columns {
                neighbouring_cells.push(i - columns)
            }
            if i < (rows - 1) * columns {
                neighbouring_cells.push(i + columns)
            }

            for neighbour in neighbouring_cells {
                if visited.contains(&neighbour) {
                    continue;
                }

                if grid.cells[i] == grid.cells[neighbour] {
                    queue.push_back(neighbour);
                } else {
                    neighbours.insert(neighbour);
                }
            }
        }

        let cells = visited
            .iter()
            .map(|&i| Position {
                column: i % columns,
                row: i / columns,
            })
            .collect();

        let component = ConnectedComponent {
            id: counter,
            colour: grid.cells[*visited.iter().next().unwrap()],
            cells,
        };
        counter += 1;

        components.push((component, neighbours));
    }

    return components;
}

impl Graph {
    pub fn create(grid: &Grid) -> Self {
        let components_and_neighbours = find_connected_components(&grid);

        let mut components = HashMap::with_capacity(components_and_neighbours.len());
        let mut neighbours: HashMap<usize, HashSet<usize>> = HashMap::new();

        for (c, n) in components_and_neighbours.into_iter() {
            let id = c.id;
            components.insert(id, c);
            neighbours.insert(id, n);
        }

        Self {
            components,
            neighbours
        }
    }

    fn to_grid(&self, rows: usize, columns: usize) -> Grid {
        let mut cells = vec![Colour::Red; rows * columns];

        for component in self.neighbours.keys() {
            for position in &self.components[component].cells {
                cells[position.column + position.row * columns] = self.components[component].colour;
            }
        }

        Grid {
            number_of_rows: rows,
            number_of_columns: columns,
            cells,
        }
    }

    pub fn find_component(&self, position: &Position) -> &ConnectedComponent {
        for component in self.neighbours.keys() {
            if self.components[component].cells.contains(&position) {
                return &self.components[component];
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_same_grid_as_input() {
        let size = 4;
        let grid = Grid::generate(size);

        let graph = Graph::create(&grid);
        let reconstituted_grid = graph.to_grid(size, size);

        assert_eq!(reconstituted_grid, grid);
    }

    #[test]
    fn should_have_one_component() {
        let grid = Grid {
            number_of_columns: 2,
            number_of_rows: 2,
            cells: vec![Colour::Red; 4],
        };

        let graph = Graph::create(&grid);

        assert_eq!(graph.neighbours.len(), 1);
    }

    #[test]
    fn should_have_four_component() {
        let grid = Grid {
            number_of_columns: 2,
            number_of_rows: 2,
            cells: vec![Colour::Red, Colour::Yellow, Colour::Yellow, Colour::Red],
        };

        let graph = Graph::create(&grid);

        assert_eq!(graph.neighbours.len(), 4);
    }

    #[test]
    fn should_have_two_component() {
        let grid = Grid {
            number_of_columns: 2,
            number_of_rows: 2,
            cells: vec![Colour::Red, Colour::Red, Colour::Yellow, Colour::Yellow],
        };

        let graph = Graph::create(&grid);

        assert_eq!(graph.neighbours.len(), 2);
    }

    #[test]
    fn should_have_same_keys_for_components_and_neighbours_maps() {
        let size = 4;
        let grid = Grid::generate(size);

        let graph = Graph::create(&grid);

        let components_keys: HashSet<_> = graph.components.keys().collect();
        let neighbours_keys: HashSet<_> = graph.neighbours.keys().collect();
        assert_eq!(components_keys, neighbours_keys);
    }

    #[test]
    fn should_contain_exactly_the_component_ids_as_keys() {
        let size = 4;
        let grid = Grid::generate(size);

        let graph = Graph::create(&grid);

        let component_keys: HashSet<_> = graph.components.keys().collect();
        let component_ids: HashSet<_> = graph.components.values().map(|x| &x.id).collect();
        assert_eq!(component_keys, component_ids);

        for (id, component) in graph.components {
            assert_eq!(id, component.id);
        }
    }
}
