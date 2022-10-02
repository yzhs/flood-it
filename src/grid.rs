use macroquad::rand::gen_range;

use crate::colour::*;

struct Grid {
    number_of_rows: u8,
    number_of_columns: u8,

    cells: Vec<Colour>,
}

impl Grid {
    fn generate(size: usize) -> Self {
        let mut cells = vec![Colour::Red; size*size];
        for i in 0..size {
            for j in 0..size {
                cells[size * i + j] = AllColours[gen_range(0, AllColours.len())];
            }
        }

        Self {
            number_of_columns: size as u8,
            number_of_rows: size as u8,
            cells,
        }
    }
}
