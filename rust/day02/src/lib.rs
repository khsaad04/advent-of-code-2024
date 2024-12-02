pub fn process_part1(input: &str) -> usize {
    input
        .lines()
        .map(|report| {
            let levels: Vec<usize> = report
                .split_whitespace()
                .map(|level| level.parse::<usize>().unwrap())
                .collect();
            if is_safe(&levels) {
                1
            } else {
                0
            }
        })
        .sum()
}

pub fn process_part2(input: &str) -> usize {
    input
        .lines()
        .map(|report| {
            let levels: Vec<usize> = report
                .split_whitespace()
                .map(|level| level.parse::<usize>().unwrap())
                .collect();
            if is_safe(&levels) {
                1
            } else {
                for i in 0..levels.len() {
                    let mut temp = levels.clone();
                    temp.remove(i);
                    if is_safe(&temp) {
                        return 1;
                    };
                }
                0
            }
        })
        .sum()
}

fn is_safe(levels: &[usize]) -> bool {
    if levels.is_sorted() || levels.iter().rev().is_sorted() {
        for i in 0..levels.len() - 1 {
            let diff = levels[i].abs_diff(levels[i + 1]);
            if !(1..=3).contains(&diff) {
                return false;
            }
        }
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, 4);
    }
}
