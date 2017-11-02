use std::borrow::Cow;
use std::collections::{HashSet, VecDeque};

use glium::texture::{RawImage2d, Texture2d, Texture2dDataSink};

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

impl ::rand::Rand for Color {
    fn rand<R: ::rand::Rng>(rng: &mut R) -> Self {
        *rng.choose(COLORS).unwrap()
    }
}

pub struct Grid {
    width: u8,
    height: u8,
    num_colors: u8,

    cells: Vec<Color>,
    population: Vec<u16>,

    max_clicks: u16,
    num_clicks: u16,
}

impl Grid {
    pub fn new(num_colors: u8, size: u8) -> Self {
        use rand::distributions::{IndependentSample, Range};
        let between = Range::new(0, num_colors as usize);
        let mut rng = ::rand::thread_rng();

        let cells: Vec<Color> = (0..size as u64 * size as u64)
            .map(|_| COLORS[between.ind_sample(&mut rng)])
            .collect();
        let max_clicks = 25 * 2 * size as u16 * num_colors as u16 / (2 * 14 * 6);
        let mut population = vec![0; COLORS.len()];
        for &c in &cells {
            population[c as usize] += 1;
        }

        Self {
            width: size,
            height: size,
            num_colors,

            cells,
            population,

            max_clicks,
            num_clicks: 0,
        }
    }

    pub fn width(&self) -> u8 {
        self.width
    }
    pub fn height(&self) -> u8 {
        self.height
    }
    pub fn num_clicks(&self) -> u16 {
        self.num_clicks
    }
    pub fn max_clicks(&self) -> u16 {
        self.max_clicks
    }

    fn index(&self, row: u8, column: u8) -> usize {
        self.width as usize * row as usize + column as usize
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

    pub fn click(&mut self, row: u8, column: u8) -> bool {
        if row >= self.height || column >= self.width {
            return false;
        }

        let i = self.index(row, column);
        let new_color = self.cells[i];
        if self.current_color() == new_color {
            return false;
        }

        self.num_clicks += 1;
        self.flood(new_color);
        self.solved()
    }

    /// Flood the grid from the top left cell and return the number of cell which are connected to
    /// the top left cell by cells of indentical color.
    fn flood(&mut self, new_color: Color) {
        let current_color = self.current_color();

        let rows = self.height as usize;
        let columns = self.width as usize;

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

            if i % columns > 0 && self.cells[i - 1] == current_color &&
                !visited.contains(&(i - 1))
            {
                queue.push_back(i - 1);
            }
            if i % columns < columns - 1 && self.cells[i + 1] == current_color &&
                !visited.contains(&(i + 1))
            {
                queue.push_back(i + 1);
            }
            if i >= columns && self.cells[i - columns] == current_color &&
                !visited.contains(&(i - columns))
            {
                queue.push_back(i - columns);
            }
            if i < (rows - 1) * columns && self.cells[i + columns] == current_color &&
                !visited.contains(&(i + columns))
            {
                queue.push_back(i + columns);
            }
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.height as f32 / self.width as f32
    }

    pub fn reset(&mut self) {
        use rand::distributions::{IndependentSample, Range};
        let between = Range::new(0, self.num_colors as usize);
        let mut rng = ::rand::thread_rng();

        self.cells.iter_mut().for_each(|x| {
            *x = COLORS[between.ind_sample(&mut rng)]
        });

        self.population = vec![0; COLORS.len()];
        for &c in &self.cells {
            self.population[c as usize] += 1;
        }
        self.num_clicks = 0;
    }

    /// Render the grid to a texture containing one colored pixel for each cell.
    pub fn render<T: ::glium::backend::Facade>(&self, display: &T) -> Texture2d {
        let cell_colors: Vec<_> = self.cells.iter().map(|x| x.to_rgb()).collect();
        let cell_image = RawImage2d::from_raw(
            Cow::from(cell_colors),
            self.width as u32,
            self.height as u32,
        );
        Texture2d::new(display, cell_image).unwrap()
    }
}
