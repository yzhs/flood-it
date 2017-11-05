pub struct ScreenInfo {
    pub width: f32,
    pub height: f32,
    pub offsets: (f64, f64),
    pub matrix: [[f32; 4]; 4],
}

impl ScreenInfo {
    pub fn dummy() -> ScreenInfo {
        ScreenInfo {
            width: 0.0,
            height: 0.0,
            offsets: (0.0, 0.0),
            matrix: [[0.0; 4]; 4],
        }
    }

    pub fn cursor_to_grid_coords(&self, x: f64, y: f64, width: u8, height: u8) -> Option<(u8, u8)> {
        if x - self.offsets.1 >= 0.0 && y - self.offsets.1 >= 0.0 {
            let column = ((x - self.offsets.0) * f64::from(width) /
                              (f64::from(self.width) - 2.0 * self.offsets.0))
                .floor() as u8;
            let row = ((y - self.offsets.1) * f64::from(height) /
                           (f64::from(self.height) - 2.0 * self.offsets.1))
                .floor() as u8;
            Some((column, row))
        } else {
            None
        }
    }

    pub fn resize(&mut self, width: u32, height: u32, grid_aspect_ratio: f32) {
        let aspect_ratio = height as f32 / width as f32;

        let offset = (f64::from(width) - f64::from(height)).abs() / 2.0;
        self.offsets = if aspect_ratio < grid_aspect_ratio {
            (offset, 0.0)
        } else {
            (0.0, offset)
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
