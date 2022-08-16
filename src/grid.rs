use std::borrow::Cow;
use std::collections::{HashSet, VecDeque};

use glium::texture::{RawImage2d, Texture2d, Texture2dDataSink};

use types::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Red,
    Yellow,
    Green,
    LightBrown,
    Purple,
    Cyan,
    Blue,
    Fuchsia,
}

pub const COLORS: &[Color] = &[
    Color::Red,
    Color::Yellow,
    Color::Green,
    Color::LightBrown,
    Color::Purple,
    Color::Cyan,
    Color::Blue,
    Color::Fuchsia,
];

impl Color {
    fn to_rgb(self) -> (u8, u8, u8) {
        match self {
            Color::Red => (255, 0, 0),
            Color::Yellow => (255, 255, 0),
            Color::Green => (0, 176, 0),
            Color::LightBrown => (255, 204, 102),
            Color::Purple => (128, 0, 128),
            Color::Cyan => (0, 255, 255),
            Color::Blue => (0, 0, 255),
            Color::Fuchsia => (255, 0, 255),
        }
    }
}

pub struct Grid {
    width: Width,
    height: Height,
    num_colors: NumberOfColors,

    cells: Vec<Color>,
    population: Vec<u16>,

    max_clicks: NumberOfClicks,
    num_clicks: NumberOfClicks,
}

impl Grid {
    pub fn new(num_colors: u8, size: u8) -> Self {
        use rand::Rng;
        let mut rng = ::rand::thread_rng();

        let cells: Vec<Color> = (0..u16::from(size) * u16::from(size))
            .map(|_| COLORS[rng.gen_range(0..num_colors as usize)])
            .collect();
        let max_clicks = 25 * 2 * u16::from(size) * u16::from(num_colors) / (2 * 14 * 6);
        let mut population = vec![0; COLORS.len()];
        for &c in &cells {
            population[c as usize] += 1;
        }

        Self {
            width: ::types::Width(size),
            height: Height(size),
            num_colors: NumberOfColors(num_colors),

            cells,
            population,

            max_clicks: NumberOfClicks(max_clicks),
            num_clicks: NumberOfClicks(0),
        }
    }

    pub fn size(&self) -> Size {
        Size(self.width, self.height)
    }
    pub fn num_clicks(&self) -> NumberOfClicks {
        self.num_clicks
    }
    pub fn max_clicks(&self) -> NumberOfClicks {
        self.max_clicks
    }

    fn index(&self, row: Row, column: Column) -> Index {
        Index(usize::from(self.width.0) * usize::from(row.0) + usize::from(column.0))
    }

    fn current_color(&self) -> Color {
        self.cells[0]
    }

    pub fn solved(&self) -> bool {
        let mut colors_present = 0;
        for &x in &self.population {
            if x > 0 {
                colors_present += 1;
            }
        }
        colors_present == 1
    }

    pub fn won(&self) -> bool {
        self.solved() && self.num_clicks <= self.max_clicks
    }

    pub fn click(&mut self, row: Row, column: Column) -> bool {
        if row >= self.height || column >= self.width {
            return false;
        }

        let i = self.index(row, column);
        let new_color = self.cells[i.0];
        if self.current_color() == new_color {
            return false;
        }

        self.num_clicks.0 += 1;
        self.flood(new_color);
        self.solved()
    }

    /// Flood the grid from the top left cell and return the number of cell which are connected to
    /// the top left cell by cells of indentical color.
    fn flood(&mut self, new_color: Color) {
        let current_color = self.current_color();

        let rows = usize::from(self.height.0);
        let columns = usize::from(self.width.0);

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(0);

        while let Some(i) = queue.pop_front() {
            if self.cells[i] == new_color {
                continue;
            }
            let c = self.cells[i];
            self.population[c as usize] -= 1;
            self.cells[i] = new_color;
            self.population[new_color as usize] += 1;
            visited.insert(i);

            if i % columns > 0 && self.cells[i - 1] == current_color && !visited.contains(&(i - 1))
            {
                queue.push_back(i - 1);
            }
            if i % columns < columns - 1
                && self.cells[i + 1] == current_color
                && !visited.contains(&(i + 1))
            {
                queue.push_back(i + 1);
            }
            if i >= columns
                && self.cells[i - columns] == current_color
                && !visited.contains(&(i - columns))
            {
                queue.push_back(i - columns);
            }
            if i < (rows - 1) * columns
                && self.cells[i + columns] == current_color
                && !visited.contains(&(i + columns))
            {
                queue.push_back(i + columns);
            }
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        f32::from(self.height.0) / f32::from(self.width.0)
    }

    pub fn reset(&mut self) {
        use rand::Rng;
        let mut rng = ::rand::thread_rng();
        let number_of_colors = self.num_colors.0 as usize;

        self.cells
            .iter_mut()
            .for_each(|x| *x = COLORS[rng.gen_range(0..number_of_colors)]);

        self.population = vec![0; COLORS.len()];
        for &c in &self.cells {
            self.population[c as usize] += 1;
        }
        self.num_clicks = NumberOfClicks(0);
    }

    /// Render the grid to a texture containing one colored pixel for each cell.
    pub fn render<T: ::glium::backend::Facade>(&self, display: &T) -> Texture2d {
        let cell_colors: Vec<_> = self.cells.iter().map(|x| x.to_rgb()).collect();
        let cell_image = RawImage2d::from_raw(
            Cow::from(cell_colors),
            u32::from(self.width.0),
            u32::from(self.height.0),
        );
        Texture2d::new(display, cell_image).unwrap()
    }
}
