use std::collections::HashMap;

pub fn process_part1(input: &str) -> usize {
    let mut input = input.split("\n\n");
    let mut rules_map: HashMap<usize, Vec<usize>> = HashMap::new();
    input.next().unwrap().split("\n").for_each(|rule| {
        let (left, right) = rule.split_once("|").unwrap();
        let left = left.parse::<usize>().unwrap();
        let right = right.parse::<usize>().unwrap();
        if let Some(rule) = rules_map.get_mut(&left) {
            rule.push(right);
        } else {
            rules_map.insert(left, vec![right]);
        };
    });
    input
        .next()
        .unwrap()
        .split("\n")
        .filter(|&x| !x.is_empty())
        .map(|update| {
            let update: Vec<usize> = update
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            for i in 0..update.len() {
                for j in (i + 1)..update.len() {
                    if let Some(rule) = rules_map.get(&update[i]) {
                        if rule.contains(&update[j]) {
                            continue;
                        }
                    }
                    return 0;
                }
            }
            update[update.len() / 2]
        })
        .sum()
}

pub fn process_part2(input: &str) -> usize {
    let mut input = input.split("\n\n");
    let mut rules_map: HashMap<usize, Vec<usize>> = HashMap::new();
    input.next().unwrap().split("\n").for_each(|rule| {
        let (left, right) = rule.split_once("|").unwrap();
        let left = left.parse::<usize>().unwrap();
        let right = right.parse::<usize>().unwrap();
        if let Some(rule) = rules_map.get_mut(&left) {
            rule.push(right);
        } else {
            rules_map.insert(left, vec![right]);
        };
    });
    input
        .next()
        .unwrap()
        .split("\n")
        .filter(|&x| !x.is_empty())
        .map(|update| {
            let mut update: Vec<usize> = update
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let mut correct = true;
            for i in 0..update.len() {
                for j in (i + 1)..update.len() {
                    if let Some(rule) = rules_map.get(&update[i]) {
                        if rule.contains(&update[j]) {
                            continue;
                        }
                    }
                    correct = false;
                }
            }
            if correct {
                return 0;
            };
            update.sort_by(|a, b| {
                if let Some(rule) = rules_map.get(a) {
                    if rule.contains(b) {
                        return std::cmp::Ordering::Less;
                    }
                }
                std::cmp::Ordering::Greater
            });
            update[update.len() / 2]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 143);
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, 123);
    }
}
