use rand::RngExt;

use crate::frame::{Color, GifFrame, ToBytes};
use crate::generators::Generator;

pub struct TestSquare;

impl Generator for TestSquare {
    fn next_frame(&mut self) -> impl ToBytes {
        {
            let mut rng = rand::rng();

            GifFrame {
                position: (rng.random_range(0..=78), rng.random_range(0..=21)),
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
                colorstream: [
                    [1, 1, 1, 1, 1, 2, 2, 2, 2, 2],
                    [1, 1, 1, 1, 1, 2, 2, 2, 2, 2],
                    [1, 1, 1, 1, 1, 2, 2, 2, 2, 2],
                    [1, 1, 1, 0, 0, 0, 0, 2, 2, 2],
                    [1, 1, 1, 0, 0, 0, 0, 2, 2, 2],
                    [2, 2, 2, 0, 0, 0, 0, 1, 1, 1],
                    [2, 2, 2, 0, 0, 0, 0, 1, 1, 1],
                    [2, 2, 2, 2, 2, 1, 1, 1, 1, 1],
                    [2, 2, 2, 2, 2, 1, 1, 1, 1, 1],
                    [2, 2, 2, 2, 2, 1, 1, 1, 1, 1],
                ],
            }
        }
    }
}
