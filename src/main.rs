use std::fs;

use crate::gif::{ImageDescriptorPackedField, LogicalScreenDescriptorPackedField};

mod gif;
mod lzw;

fn main() {
    let data = [
        gif::header().as_ref(),
        gif::logical_screen_descriptor(
            10,
            10,
            LogicalScreenDescriptorPackedField {
                global_color_table_flag: true,
                color_resolution: 0,
                sort_flag: false,
                size_of_global_color_table: 1,
            },
            0,
            0,
        )
        .as_ref(),
        &gif::color_table(&[[255, 255, 255], [255, 0, 0], [0, 0, 255], [0, 0, 0]]),
        gif::image_descriptor(
            0,
            0,
            10,
            10,
            ImageDescriptorPackedField {
                local_color_table_flag: false,
                interlace_flag: false,
                sort_flag: false,
                size_of_local_color_table: 0,
            },
        )
        .as_ref(),
        gif::image_data(
            &gif::codestream_to_bytes(
                &lzw::colorstream_to_codestream(
                    &[
                        1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1,
                        2, 2, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0, 2, 2, 2,
                        2, 2, 2, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 2, 2,
                        1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1,
                    ],
                    2,
                ),
                2,
            ),
            2,
        )
        .as_ref(),
        &[gif::trailer()],
    ]
    .concat();

    fs::write("test.gif", data).unwrap();
}
