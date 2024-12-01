use std::collections::HashMap;

pub fn process_part1(input: &str) -> u32 {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    input.lines().for_each(|line| {
        let v: Vec<_> = line.split_whitespace().collect();
        left.push(v[0].parse::<u32>().unwrap());
        right.push(v[1].parse::<u32>().unwrap());
    });
    left.sort();
    right.sort();
    left.iter()
        .enumerate()
        .map(|(i, n)| n.abs_diff(right[i]))
        .sum()
}

pub fn process_part2(input: &str) -> u32 {
    let mut left: Vec<u32> = vec![];
    let mut right: HashMap<u32, u32> = HashMap::new();
    input.lines().for_each(|line| {
        let v: Vec<_> = line.split_whitespace().collect();
        let n1 = v[0].parse::<u32>().unwrap();
        let n2 = v[1].parse::<u32>().unwrap();
        left.push(n1);
        if let Some(v) = right.get_mut(&n2) {
            *v += 1
        } else {
            right.insert(n2, 1);
        }
    });
    left.iter().map(|n| n * right.get(n).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 11);
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, 31);
    }
}
