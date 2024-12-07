use std::collections::HashSet;

pub fn process_part1(input: &str) -> usize {
    let mut visited: HashSet<usize> = HashSet::new();
    let line_len = input.find('\n').unwrap() + 1;
    let mut guard_pos = input.find('^').unwrap();
    let mut dir = -(line_len as isize);
    loop {
        if visited.get(&guard_pos).is_none() {
            visited.insert(guard_pos);
        }
        let next_pos = guard_pos as isize + dir;
        if 0 > next_pos || next_pos > input.len() as isize {
            break;
        };
        let next_char = input.as_bytes()[next_pos as usize] as char;
        if next_char == '#' {
            if dir == -1 {
                dir = -(line_len as isize);
            } else if dir == 1 {
                dir = line_len as isize;
            } else if dir == -(line_len as isize) {
                dir = 1;
            } else {
                dir = -1;
            }
            continue;
        } else if next_char == '\n' {
            break;
        }
        guard_pos = next_pos as usize;
    }
    visited.len()
}

pub fn process_part2(input: &str) -> usize {
    let mut obstruction_count = 0;
    for i in 0..input.len() {
        if ['#', '\n'].contains(&(input.as_bytes()[i] as char)) {
            continue;
        }
        let mut visited: HashSet<(usize, isize)> = HashSet::new();
        let line_len = input.find('\n').unwrap() + 1;
        let mut guard_pos = input.find('^').unwrap();
        let mut dir = -(line_len as isize);
        loop {
            if visited.get(&(guard_pos, dir)).is_some() {
                obstruction_count += 1;
                break;
            }
            visited.insert((guard_pos, dir));
            let next_pos = guard_pos as isize + dir;
            if 0 > next_pos || next_pos > input.len() as isize {
                break;
            };
            let next_char = input.as_bytes()[next_pos as usize] as char;
            if next_char == '#' || next_pos == i as isize {
                if dir == -1 {
                    dir = -(line_len as isize);
                } else if dir == 1 {
                    dir = line_len as isize;
                } else if dir == -(line_len as isize) {
                    dir = 1;
                } else {
                    dir = -1;
                }
                continue;
            } else if next_char == '\n' {
                break;
            }
            guard_pos = next_pos as usize;
        }
    }
    obstruction_count
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 41);
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, 6);
    }
}
