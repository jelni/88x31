pub fn colorstream_to_codestream(color_stream: &[u8], min_code_size: u8) -> Vec<u16> {
    assert!(
        (2..=12).contains(&min_code_size),
        "2 >= min_code_size <= 12"
    );

    let clear_code = 1 << min_code_size;
    let eoi_code = clear_code + 1;

    let mut color_table = (0..1 << min_code_size)
        .map(|code| (None, code))
        .collect::<Vec<_>>();

    let mut code_stream = vec![clear_code];
    let mut input_index = 0;
    let mut current_code = (None, color_stream[input_index]);

    loop {
        if let Some(index) = get_color_code(&color_table, current_code, clear_code) {
            input_index += 1;

            if input_index >= color_stream.len() {
                code_stream.push(index);
                break;
            }

            current_code = (Some(index), color_stream[input_index]);
            continue;
        }

        color_table.push(current_code);
        code_stream.push(current_code.0.unwrap());

        if color_table.len() >= 4096 {
            color_table.truncate(1 << min_code_size);
            code_stream.push(clear_code);
        }

        current_code = (None, color_stream[input_index]);
    }

    code_stream.push(eoi_code);
    code_stream
}

fn get_color_code(
    color_table: &[(Option<u16>, u8)],
    current_code: (Option<u16>, u8),
    color_count: u16,
) -> Option<u16> {
    let index = color_table
        .iter()
        .position(|&code| code == current_code)?
        .try_into()
        .unwrap();

    if index >= color_count {
        return Some(index + 2);
    }

    Some(index)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lzw() {
        assert_eq!(
            colorstream_to_codestream(
                &[
                    1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 2,
                    2, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0, 2, 2, 2, 2, 2,
                    2, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 1,
                    1, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1,
                ],
                2
            ),
            vec![
                4, 1, 6, 6, 2, 9, 9, 7, 8, 10, 2, 12, 1, 14, 15, 6, 0, 21, 0, 10, 7, 22, 23, 18,
                26, 7, 10, 29, 13, 24, 12, 18, 16, 36, 12, 5
            ]
        );
    }
}
