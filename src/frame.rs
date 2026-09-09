use std::iter;

use crate::gif::{self, GraphicControlExtensionPackedField, ImageDescriptorPackedField};
use crate::lzw;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    const fn as_array(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

pub struct GifFrame<const WIDTH: usize, const HEIGHT: usize> {
    pub position: (u16, u16),
    pub color_table: Vec<Color>,
    pub colorstream: [[u8; WIDTH]; HEIGHT],
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl<const WIDTH: usize, const HEIGHT: usize> ToBytes for GifFrame<WIDTH, HEIGHT> {
    fn to_bytes(&self) -> Vec<u8> {
        let color_table_size = self.color_table.len().next_power_of_two();

        let min_code_size = u8::try_from(color_table_size.bit_width()).unwrap() - 1;

        [
            gif::graphic_control_extension(
                GraphicControlExtensionPackedField {
                    disposal_method: 0,
                    user_input_flag: false,
                    transparent_color_flag: false,
                },
                0,
                0,
            )
            .as_ref(),
            gif::image_descriptor(
                self.position.0,
                self.position.1,
                WIDTH.try_into().unwrap(),
                HEIGHT.try_into().unwrap(),
                ImageDescriptorPackedField {
                    local_color_table_flag: true,
                    interlace_flag: false,
                    sort_flag: false,
                    size_of_local_color_table: min_code_size >> 1,
                },
            )
            .as_ref(),
            gif::color_table(
                &self
                    .color_table
                    .iter()
                    .copied()
                    .chain(iter::repeat(Color { r: 0, g: 0, b: 0 }))
                    .take(color_table_size)
                    .map(Color::as_array)
                    .collect::<Vec<_>>(),
            )
            .as_ref(),
            gif::image_data(
                &gif::codestream_to_bytes(
                    &lzw::colorstream_to_codestream(&self.colorstream.concat(), min_code_size),
                    min_code_size,
                ),
                min_code_size,
            )
            .as_ref(),
        ]
        .concat()
    }
}
