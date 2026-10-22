use rand::RngExt;

use crate::frame::{Color, GifFrame};
use crate::generators::Generator;

pub struct TestSquare {
    iterations_left: u8,
}

impl TestSquare {
    pub const fn new(iterations: u8) -> Self {
        Self {
            iterations_left: iterations,
        }
    }
}

impl Generator for TestSquare {
    fn next_frame(&mut self) -> Option<GifFrame> {
        self.iterations_left = self.iterations_left.checked_sub(1)?;
        let mut rng = rand::rng();

        Some(GifFrame {
            position: (rng.random_range(0..=78), rng.random_range(0..=21)),
            size: (10, 10),
            transparent_color_index: None,
            color_table: [
                Color {
                    r: 255,
                    g: 255,
                    b: 255,
                },
                Color { r: 255, g: 0, b: 0 },
                Color { r: 0, g: 0, b: 255 },
            ]
            .into(),
            colorstream: vec![
                vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 2],
                vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 2],
                vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 2],
                vec![1, 1, 1, 0, 0, 0, 0, 2, 2, 2],
                vec![1, 1, 1, 0, 0, 0, 0, 2, 2, 2],
                vec![2, 2, 2, 0, 0, 0, 0, 1, 1, 1],
                vec![2, 2, 2, 0, 0, 0, 0, 1, 1, 1],
                vec![2, 2, 2, 2, 2, 1, 1, 1, 1, 1],
                vec![2, 2, 2, 2, 2, 1, 1, 1, 1, 1],
                vec![2, 2, 2, 2, 2, 1, 1, 1, 1, 1],
            ],
        })
    }
}
