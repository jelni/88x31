//! based on <https://www.matthewflickinger.com/lab/whatsinagif/>

#[derive(Clone, Copy)]
pub struct LogicalScreenDescriptorPackedField {
    pub global_color_table_flag: bool,
    pub color_resolution: u8,
    pub sort_flag: bool,
    pub size_of_global_color_table: u8,
}

impl LogicalScreenDescriptorPackedField {
    pub fn as_byte(self) -> u8 {
        assert!(
            self.color_resolution < (1 << 3),
            "color_resolution has only 3 bits"
        );
        assert!(
            self.size_of_global_color_table < (1 << 3),
            "color_resolution has only 3 bits"
        );

        let mut value = 0;

        if self.global_color_table_flag {
            value |= 1 << 7;
        }

        value |= self.color_resolution << 4;

        if self.sort_flag {
            value |= 1 << 3;
        }

        value | self.size_of_global_color_table
    }
}

#[derive(Clone, Copy)]
pub struct ImageDescriptorPackedField {
    pub local_color_table_flag: bool,
    pub interlace_flag: bool,
    pub sort_flag: bool,
    pub size_of_local_color_table: u8,
}

impl ImageDescriptorPackedField {
    pub fn as_byte(self) -> u8 {
        assert!(
            self.size_of_local_color_table < (1 << 3),
            "size_of_local_color_table has only 3 bits"
        );

        let mut value = 0;

        if self.local_color_table_flag {
            value |= 1 << 7;
        }

        if self.interlace_flag {
            value |= 1 << 6;
        }

        if self.sort_flag {
            value |= 1 << 5;
        }

        value | self.size_of_local_color_table
    }
}

pub const fn header() -> [u8; 6] {
    *b"GIF89a"
}

pub fn logical_screen_descriptor(
    canvas_width: u16,
    canvas_height: u16,
    packed_field: LogicalScreenDescriptorPackedField,
    background_color_index: u8,
    pixel_aspect_ratio: u8,
) -> [u8; 7] {
    [
        canvas_width.to_le_bytes().as_ref(),
        canvas_height.to_le_bytes().as_ref(),
        &[packed_field.as_byte()],
        background_color_index.to_le_bytes().as_ref(),
        pixel_aspect_ratio.to_le_bytes().as_ref(),
    ]
    .concat()
    .try_into()
    .unwrap()
}

pub fn color_table(colors: &[[u8; 3]]) -> Vec<u8> {
    colors.concat()
}

pub fn image_descriptor(
    image_left: u16,
    image_top: u16,
    image_width: u16,
    image_height: u16,
    packed_field: ImageDescriptorPackedField,
) -> [u8; 10] {
    [
        &[0x2c],
        image_left.to_le_bytes().as_ref(),
        image_top.to_le_bytes().as_ref(),
        image_width.to_le_bytes().as_ref(),
        image_height.to_le_bytes().as_ref(),
        &[packed_field.as_byte()],
    ]
    .concat()
    .try_into()
    .unwrap()
}

pub fn image_data(bytes: &[u8], min_code_size: u8) -> Vec<u8> {
    let mut output = vec![min_code_size];

    for chunk in bytes.chunks(255) {
        output.push(chunk.len().try_into().unwrap());
        output.extend(chunk);
    }

    output.push(0);
    output
}

pub const fn trailer() -> u8 {
    0x3b
}

pub fn codestream_to_bytes(code_stream: &[u16], min_code_size: u8) -> Vec<u8> {
    let clear_code = 1 << min_code_size;
    let mut dict_size = clear_code + 1;
    let mut current_code_size = min_code_size + 1;
    let mut output = Vec::new();
    let mut current_byte = 0;
    let mut bit_offset = 0;

    // based on https://docs.rs/lzw/latest/lzw/trait.BitWriter.html
    for &code in code_stream {
        current_byte |= u32::from(code) << bit_offset;
        bit_offset += current_code_size;

        while bit_offset >= 8 {
            output.push(current_byte.to_le_bytes()[0]);
            current_byte >>= 8;
            bit_offset -= 8;
        }

        if code == clear_code {
            dict_size = clear_code + 1;
            current_code_size = min_code_size + 1;
        } else {
            dict_size += 1;
        }

        if current_code_size < 12 && dict_size == 1 << current_code_size {
            current_code_size += 1;
        }
    }

    if bit_offset > 0 {
        output.push(current_byte.to_le_bytes()[0]);
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_logical_screen_descriptor_packed_field_as_byte() {
        assert_eq!(
            LogicalScreenDescriptorPackedField {
                global_color_table_flag: true,
                color_resolution: 1,
                sort_flag: false,
                size_of_global_color_table: 1
            }
            .as_byte(),
            0x91
        );
    }

    #[test]
    fn test_image_data() {
        assert_eq!(
            image_data(
                &[
                    0x8c, 0x2d, 0x99, 0x87, 0x2a, 0x1c, 0xdc, 0x33, 0xa0, 0x02, 0x75, 0xec, 0x95,
                    0xfa, 0xa8, 0xde, 0x60, 0x8c, 0x04, 0x91, 0x4c, 0x01
                ],
                2
            ),
            &[
                0x02, 0x16, 0x8c, 0x2d, 0x99, 0x87, 0x2a, 0x1c, 0xdc, 0x33, 0xa0, 0x02, 0x75, 0xec,
                0x95, 0xfa, 0xa8, 0xde, 0x60, 0x8c, 0x04, 0x91, 0x4c, 0x01, 0x00,
            ]
        );
    }

    #[test]
    fn test_codestream_to_bytes() {
        assert_eq!(
            codestream_to_bytes(
                &[
                    0b100, 0b001, 0b110, 0b110, 0b0010, 0b1001, 0b1001, 0b0111, 0b1000, 0b1010,
                    0b0010, 0b1100
                ],
                2
            ),
            &[
                0b10001100, 0b00101101, 0b10011001, 0b10000111, 0b00101010, 0b00001100
            ]
        );

        assert_eq!(
            codestream_to_bytes(
                &[
                    4, 1, 6, 6, 2, 9, 9, 7, 8, 10, 2, 12, 1, 14, 15, 6, 0, 21, 0, 10, 7, 22, 23,
                    18, 26, 7, 10, 29, 13, 24, 12, 18, 16, 36, 12, 5
                ],
                2
            ),
            &[
                0x8c, 0x2d, 0x99, 0x87, 0x2a, 0x1c, 0xdc, 0x33, 0xa0, 0x02, 0x75, 0xec, 0x95, 0xfa,
                0xa8, 0xde, 0x60, 0x8c, 0x04, 0x91, 0x4c, 0x01
            ]
        );
    }
}
