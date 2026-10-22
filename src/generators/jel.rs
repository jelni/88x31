use std::borrow::Cow;

use crate::frame::GifFrame;
use crate::generators::Generator;
use crate::generators::text::Character;
use crate::generators::text_lines::TextLinesGenerator;

pub struct JelGenerator {
    generator: TextLinesGenerator<'static>,
}

impl JelGenerator {
    pub fn new() -> Self {
        let lines = {
            let mut jel = [Character::J, Character::E, Character::L]
                .iter()
                .copied()
                .cycle();

            (0..4)
                .map(|line| {
                    let mut characters =
                        Box::new(jel.by_ref().take(17)) as Box<dyn Iterator<Item = _>>;

                    if line >= 3 {
                        characters = Box::new(
                            characters
                                .take(15)
                                .chain([Character::Colon, Character::Three]),
                        );
                    }

                    Cow::Owned(characters.collect())
                })
                .collect()
        };

        Self {
            generator: TextLinesGenerator::new((2, 0), lines),
        }
    }
}

impl Generator for JelGenerator {
    fn next_frame(&mut self) -> Option<GifFrame> {
        self.generator.next_frame()
    }
}
