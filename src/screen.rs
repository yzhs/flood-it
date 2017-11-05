use types::*;

pub struct ScreenInfo {
    pub width: f32,
    pub height: f32,
    pub offsets: Position<f64>,
    pub matrix: [[f32; 4]; 4],
}

impl ScreenInfo {
    pub fn dummy() -> ScreenInfo {
        ScreenInfo {
            width: 0.0,
            height: 0.0,
            offsets: Position(0.0.into(), 0.0.into()),
            matrix: [[0.0; 4]; 4],
        }
    }

    pub fn cursor_to_grid_coords(&self, pos: Position<f64>, size: Size) -> Option<Position> {
        let Position(x, y) = pos - self.offsets;
        let Size(width, height) = size;
        if x >= Column(0.0) && y >= Row(0.0) {
            let column = (x.0 * f64::from(width.0) /
                              (f64::from(self.width) - (self.offsets.0 * 2.0).0))
                .floor() as u8;
            let row = (y.0 * f64::from(height.0) /
                           (f64::from(self.height) - (self.offsets.1 * 2.0).0))
                .floor() as u8;
            Some(Position(Column(column), Row(row)))
        } else {
            None
        }
    }

    pub fn resize(&mut self, width: u32, height: u32, grid_aspect_ratio: f32) {
        let aspect_ratio = height as f32 / width as f32;

        let offset = (f64::from(width) - f64::from(height)).abs() / 2.0;
        self.offsets = if aspect_ratio < grid_aspect_ratio {
            Position(offset.into(), 0.0.into())
        } else {
            Position(0.0.into(), offset.into())
        };

        let (ratio_x, ratio_y) = if aspect_ratio < grid_aspect_ratio {
            (aspect_ratio, 1.0)
        } else {
            (1.0, aspect_ratio.recip())
        };

        self.width = width as f32;
        self.height = height as f32;
        self.matrix = [
            [ratio_x, 0.0, 0.0, 0.0],
            [0.0, -ratio_y, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0_f32],
        ];
    }
}
