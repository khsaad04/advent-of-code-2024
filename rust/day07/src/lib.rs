pub fn process_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (sol, eq) = line.split_once(": ").unwrap();
            let sol = sol.parse::<usize>().unwrap();
            let eq: Vec<usize> = eq
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .rev()
                .collect();

            if is_valid_part1(sol, &eq) {
                sol
            } else {
                0
            }
        })
        .sum()
}

fn is_valid_part1(sol: usize, eq: &[usize]) -> bool {
    if eq.is_empty() {
        return sol == 0;
    };
    let sum_sol = is_valid_part1((sol as isize - eq[0] as isize) as usize, &eq[1..]);
    let mult_sol = if sol % eq[0] == 0 {
        is_valid_part1(sol / eq[0], &eq[1..])
    } else {
        false
    };
    sum_sol || mult_sol
}

pub fn process_part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (sol, eq) = line.split_once(": ").unwrap();
            let sol = sol.parse::<usize>().unwrap();
            let eq: Vec<usize> = eq
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .rev()
                .collect();

            if is_valid_part2(Some(sol), &eq) {
                sol
            } else {
                0
            }
        })
        .sum()
}

fn is_valid_part2(sol: Option<usize>, eq: &[usize]) -> bool {
    if sol.is_none() {
        return false;
    };
    let sol = sol.unwrap();
    if eq.is_empty() {
        return sol == 0;
    };
    let sum_sol = is_valid_part2(Some((sol as isize - eq[0] as isize) as usize), &eq[1..]);
    let mult_sol = if sol % eq[0] == 0 {
        is_valid_part2(Some(sol / eq[0]), &eq[1..])
    } else {
        false
    };
    let concat_sol = is_valid_part2(deconcat(sol, eq[0]), &eq[1..]);
    sum_sol || mult_sol || concat_sol
}

fn deconcat(log: usize, splinter: usize) -> Option<usize> {
    let splinter_size_in_hundreds =
        f32::powf(10_f32, f32::floor(f32::log10(splinter as f32)) + 1_f32) as usize;
    let after_deconcat = log / splinter_size_in_hundreds;
    if after_deconcat as u128 * splinter_size_in_hundreds as u128 + splinter as u128 == log as u128
    {
        Some(after_deconcat)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 3749);
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, 11387);
    }
}
