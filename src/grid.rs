use macroquad::rand::gen_range;

use crate::colour::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Grid {
    pub number_of_rows: usize,
    pub number_of_columns: usize,

    pub cells: Vec<Colour>,
}

impl Grid {
    pub fn generate(size: usize) -> Self {
        let mut cells = vec![Colour::Red; size * size];
        for i in 0..size {
            for j in 0..size {
                cells[size * i + j] = ALL_COLOURS[gen_range(0, ALL_COLOURS.len())];
            }
        }

        Self {
            number_of_columns: size,
            number_of_rows: size,
            cells,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn generates_grid_of_correct_size() {
        let size = 3;

        let grid = Grid::generate(size);

        assert_eq!(grid.number_of_rows, size);
        assert_eq!(grid.number_of_columns, size);
        assert_eq!(grid.cells.len(), size * size);
    }
}
