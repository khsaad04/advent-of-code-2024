pub fn process_part1(input: &str) -> usize {
    let mut count = 0;
    let input_len = input.len();
    let line_len = input.find('\n').unwrap() + 1;
    (0..input_len).for_each(|i| {
        if (i + 3) < input_len {
            let w = format!(
                "{}{}{}{}",
                input.as_bytes()[i] as char,
                input.as_bytes()[i + 1] as char,
                input.as_bytes()[i + 2] as char,
                input.as_bytes()[i + 3] as char,
            );
            if w == "XMAS" || w == "SAMX" {
                count += 1;
            }
        };
        if (i + line_len * 3 + 3) < input_len {
            let w = format!(
                "{}{}{}{}",
                input.as_bytes()[i] as char,
                input.as_bytes()[i + line_len + 1] as char,
                input.as_bytes()[i + line_len * 2 + 2] as char,
                input.as_bytes()[i + line_len * 3 + 3] as char,
            );
            if w == "XMAS" || w == "SAMX" {
                count += 1;
            }
        };
        if (i + line_len * 3 - 3) < input_len {
            let w = format!(
                "{}{}{}{}",
                input.as_bytes()[i] as char,
                input.as_bytes()[i + line_len - 1] as char,
                input.as_bytes()[i + line_len * 2 - 2] as char,
                input.as_bytes()[i + line_len * 3 - 3] as char,
            );
            if w == "XMAS" || w == "SAMX" {
                count += 1;
            }
        };
        if (i + line_len * 3) < input_len {
            let w = format!(
                "{}{}{}{}",
                input.as_bytes()[i] as char,
                input.as_bytes()[i + line_len] as char,
                input.as_bytes()[i + line_len * 2] as char,
                input.as_bytes()[i + line_len * 3] as char,
            );
            if w == "XMAS" || w == "SAMX" {
                count += 1;
            }
        };
    });
    count
}

pub fn process_part2(input: &str) -> usize {
    let mut count = 0;
    let line_len = input.find('\n').unwrap() + 1;
    input.match_indices('A').for_each(|(i, _)| {
        if i as isize - line_len as isize - 1 > 0 && i + line_len + 1 < input.len() {
            let w = format!(
                "{}{}A{}{}",
                input.as_bytes()[i - line_len - 1] as char,
                input.as_bytes()[i - line_len + 1] as char,
                input.as_bytes()[i + line_len - 1] as char,
                input.as_bytes()[i + line_len + 1] as char,
            );
            if w == "MSAMS" || w == "SMASM" || w == "SSAMM" || w == "MMASS" {
                count += 1;
            }
        };
    });
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 18);
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, 9);
    }
}
