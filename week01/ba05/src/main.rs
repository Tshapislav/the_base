//! ba05

fn main() {
    todo!()
}

pub fn parse_bitmap_8x8(lines: [&str; 8]) -> [u8; 8] {
    lines.map(|line| {
        let mut line_buff: u8 = 0b0000_0000;
        for (char_index, char) in line.chars().enumerate() {
            match char {
                '#' => line_buff |= 1 << (7 - char_index),
                _ => continue,
            };
        }
        line_buff
    })
}


pub fn render_bitmap_8x8(bytes: [u8; 8]) -> [String; 8] {
    bytes.map(|byte| {
        let mut line = String::with_capacity(8);
        for i in 0..8 {
            let symbol = match (byte >> (7 - i)) & 1 {
                1 => '#',
                _ => '.',
            };
            line.push(symbol);
        };
        line
    })
}

pub fn invert_bitmap_8x8(bytes: [u8; 8]) -> [u8; 8] {
    bytes.map(|x| !x)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bitmap() {
        // Given
        let image = [
            "..####..",
            ".#....#.",
            "#.#..#.#",
            "#..##..#",
            "#......#",
            "#.#..#.#",
            ".#....#.",
            "..####..",
        ];

        // When
        let bytes = parse_bitmap_8x8(image);

        // Then
        let control_values: [u8; 8] = [
            0b0011_1100,
            0b0100_0010,
            0b1010_0101,
            0b1001_1001,
            0b1000_0001,
            0b1010_0101,
            0b0100_0010,
            0b0011_1100,
        ];

        assert_eq!(bytes, control_values);
    }

    #[test]
    fn test_render_bitmap() {
        // Given
        let values: [u8; 8] = [
            0b0011_1100,
            0b0100_0010,
            0b1010_0101,
            0b1001_1001,
            0b1000_0001,
            0b1010_0101,
            0b0100_0010,
            0b0011_1100,
        ];

        // When
        let bytes = render_bitmap_8x8(values);

        // Then
        let control_image = [
            "..####..",
            ".#....#.",
            "#.#..#.#",
            "#..##..#",
            "#......#",
            "#.#..#.#",
            ".#....#.",
            "..####..",
        ];

        assert_eq!(bytes, control_image);
    }

    #[test]
    fn test_invert_bitmap() {
        // Given
        let image = [
            "..####..",
            ".#....#.",
            "#.#..#.#",
            "#..##..#",
            "#......#",
            "#.#..#.#",
            ".#....#.",
            "..####..",
        ];

        // When
        let bytes = parse_bitmap_8x8(image);
        let invert_image = render_bitmap_8x8(invert_bitmap_8x8(bytes));
        // Then
        let control_image = [
            "##....##",
            "#.####.#",
            ".#.##.#.",
            ".##..##.",
            ".######.",
            ".#.##.#.",
            "#.####.#",
            "##....##",
        ];

        assert_eq!(invert_image, control_image);
    }

    #[test]
    fn test_parse_asymmetric() {
        // Given
        let image = [
            "#.......",
            "........",
            "........",
            "........",
            "........",
            "........",
            "........",
            "........",
        ];

        // When
        let bytes = parse_bitmap_8x8(image);

        // Then
        assert_eq!(bytes[0], 0b1000_0000);
    }
}
