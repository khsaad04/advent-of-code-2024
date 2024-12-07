use day06::process_part2;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = process_part2(&input);

    println!("Part 2: {result}");
}
