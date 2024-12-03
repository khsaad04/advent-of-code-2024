pub fn process_part1(input: &str) -> usize {
    let re = regex::Regex::new(r"mul\((?P<num1>\d{1,3}),(?P<num2>\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            let (_, [n1, n2]) = c.extract();
            n1.parse::<usize>().unwrap() * n2.parse::<usize>().unwrap()
        })
        .sum::<usize>()
}

pub fn process_part2(input: &str) -> usize {
    let re = regex::Regex::new(r"mul\((?P<num1>\d{1,3}),(?P<num2>\d{1,3})\)").unwrap();
    input
        .split("do()")
        .map(|x| {
            let y: Vec<_> = x.split("don't()").collect();
            re.captures_iter(y[0])
                .map(|c| {
                    let (_, [n1, n2]) = c.extract();
                    n1.parse::<usize>().unwrap() * n2.parse::<usize>().unwrap()
                })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 161);
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, 48);
    }
}
