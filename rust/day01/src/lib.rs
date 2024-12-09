use std::collections::HashMap;

pub fn process_part1(input: &str) -> usize {
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];
    input.lines().for_each(|line| {
        let (n1, n2) = line.split_once("   ").unwrap();
        left.push(n1.parse::<usize>().unwrap());
        right.push(n2.parse::<usize>().unwrap());
    });
    left.sort();
    right.sort();
    left.iter()
        .enumerate()
        .map(|(i, n)| n.abs_diff(right[i]))
        .sum()
}

pub fn process_part2(input: &str) -> usize {
    let mut left: Vec<usize> = vec![];
    let mut right: HashMap<usize, usize> = HashMap::new();
    input.lines().for_each(|line| {
        let (n1, n2) = line.split_once("   ").unwrap();
        let n1 = n1.parse::<usize>().unwrap();
        let n2 = n2.parse::<usize>().unwrap();
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
