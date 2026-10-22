use std::borrow::Cow;

use crate::frame::{Color, GifFrame};
use crate::generators::Generator;

#[derive(Clone, Copy)]
pub enum Character {
    Three,
    E,
    J,
    L,
    Colon,
}

impl Character {
    const fn pixels(self) -> &'static [&'static [bool]] {
        match self {
            Self::Three => &[
                &[true, true, true, false],
                &[false, false, false, true],
                &[false, false, false, true],
                &[true, true, true, false],
                &[false, false, false, true],
                &[false, false, false, true],
                &[true, true, true, false],
            ],
            Self::E => &[
                &[true, true, true, true],
                &[true, false, false, false],
                &[true, false, false, false],
                &[true, true, true, true],
                &[true, false, false, false],
                &[true, false, false, false],
                &[true, true, true, true],
            ],
            Self::J => &[
                &[true, true, true, true],
                &[false, false, false, true],
                &[false, false, false, true],
                &[false, false, false, true],
                &[true, false, false, true],
                &[true, false, false, true],
                &[false, true, true, false],
            ],
            Self::L => &[
                &[true, false, false, false],
                &[true, false, false, false],
                &[true, false, false, false],
                &[true, false, false, false],
                &[true, false, false, false],
                &[true, false, false, false],
                &[true, true, true, true],
            ],
            Self::Colon => &[
                &[false],
                &[true],
                &[true],
                &[false],
                &[true],
                &[true],
                &[false],
            ],
        }
    }

    fn size(self) -> (u16, u16) {
        let pixels = self.pixels();

        (
            pixels[0].len().try_into().unwrap(),
            pixels.len().try_into().unwrap(),
        )
    }
}

pub struct TextGenerator<'a> {
    position: (u16, u16),
    characters: Cow<'a, [Character]>,
    iteration: usize,
}

impl<'a> TextGenerator<'a> {
    pub const fn new(position: (u16, u16), characters: Cow<'a, [Character]>) -> Self {
        Self {
            position,
            characters,
            iteration: 0,
        }
    }
}

impl Generator for TextGenerator<'_> {
    fn next_frame(&mut self) -> Option<GifFrame> {
        let position = (
            self.position.0
                + self
                    .characters
                    .iter()
                    .take(self.iteration)
                    .map(|character| character.size().0 + 1)
                    .sum::<u16>(),
            self.position.1,
        );

        if self.iteration >= self.characters.len() {
            return None;
        }

        let character = self.characters[self.iteration];
        self.iteration += 1;

        Some(GifFrame {
            position,
            size: character.size(),
            transparent_color_index: Some(0),
            color_table: vec![Color { r: 0, g: 0, b: 0 }],
            colorstream: character
                .pixels()
                .iter()
                .map(|line| line.iter().map(|&character| character.into()).collect())
                .collect(),
        })
    }
}
