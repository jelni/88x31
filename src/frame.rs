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

pub struct GifFrame {
    pub position: (u16, u16),
    pub size: (u16, u16),
    pub transparent_color_index: Option<u8>,
    pub color_table: Vec<Color>,
    pub colorstream: Vec<Vec<u8>>,
}

impl GifFrame {
    pub fn to_bytes(&self) -> Vec<u8> {
        let color_table_size = self.color_table.len().next_power_of_two().max(2);
        let min_code_size = (u8::try_from(color_table_size.bit_width()).unwrap() - 1).max(2);

        [
            gif::graphic_control_extension(
                GraphicControlExtensionPackedField {
                    disposal_method: 0,
                    user_input_flag: false,
                    transparent_color_flag: self.transparent_color_index.is_some(),
                },
                2,
                self.transparent_color_index.unwrap_or_default(),
            )
            .as_ref(),
            gif::image_descriptor(
                self.position.0,
                self.position.1,
                self.size.0,
                self.size.1,
                ImageDescriptorPackedField {
                    local_color_table_flag: true,
                    interlace_flag: false,
                    sort_flag: false,
                    size_of_local_color_table: (color_table_size.trailing_zeros() - 1)
                        .try_into()
                        .unwrap(),
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
