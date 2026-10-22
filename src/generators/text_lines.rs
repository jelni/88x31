use std::borrow::Cow;
use std::collections::VecDeque;

use crate::frame::GifFrame;
use crate::generators::Generator;
use crate::generators::text::{Character, TextGenerator};

pub struct TextLinesGenerator<'a> {
    lines: VecDeque<TextGenerator<'a>>,
}

impl<'a> TextLinesGenerator<'a> {
    pub fn new(position: (u16, u16), lines: Vec<Cow<'a, [Character]>>) -> Self {
        Self {
            lines: lines
                .into_iter()
                .zip(0..)
                .map(|(line, index)| TextGenerator::new((position.0, position.1 + 8 * index), line))
                .collect(),
        }
    }
}

impl Generator for TextLinesGenerator<'_> {
    fn next_frame(&mut self) -> Option<GifFrame> {
        while let Some(line) = self.lines.front_mut() {
            if let Some(frame) = line.next_frame() {
                return Some(frame);
            }

            self.lines.pop_front();
        }

        None
    }
}
