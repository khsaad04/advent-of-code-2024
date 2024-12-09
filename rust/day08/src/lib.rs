use std::collections::{HashMap, HashSet};

pub fn process_part1(input: &str) -> usize {
    let line_len = (input.find('\n').unwrap() + 1) as isize;
    let mut antennas_map: HashMap<u8, Vec<usize>> = HashMap::new();
    input
        .as_bytes()
        .iter()
        .enumerate()
        .for_each(|(i, &antenna)| {
            if ![b'.', b'\n'].contains(&antenna) {
                if let Some(antenna_list) = antennas_map.get_mut(&antenna) {
                    antenna_list.push(i);
                } else {
                    antennas_map.insert(antenna, vec![i]);
                };
            }
        });

    let mut antinodes_set: HashSet<usize> = HashSet::new();
    antennas_map.iter().for_each(|(_, antenna_list)| {
        if antenna_list.len() > 2 {
            for (i, &antenna1) in antenna_list[..antenna_list.len() - 1].iter().enumerate() {
                for &antenna2 in antenna_list[i + 1..].iter() {
                    let antenna_offset =
                        antenna2 as isize % line_len - antenna1 as isize % line_len;
                    let antinode1 = 2 * antenna1 as isize - antenna2 as isize;
                    let antinode1_offset = antenna1 as isize % line_len - antinode1 % line_len;
                    if antinode1 >= 0
                        && antinode1_offset == antenna_offset
                        && input.as_bytes()[antinode1 as usize] != b'\n'
                    {
                        antinodes_set.insert(antinode1 as usize);
                    }

                    let antinode2 = 2 * antenna2 - antenna1;
                    let antinode2_offset =
                        antinode2 as isize % line_len - antenna2 as isize % line_len;
                    if antinode2 < input.len()
                        && antinode2_offset == antenna_offset
                        && input.as_bytes()[antinode2] != b'\n'
                    {
                        antinodes_set.insert(antinode2);
                    }
                }
            }
        }
    });
    antinodes_set.len()
}

pub fn process_part2(input: &str) -> usize {
    let line_len = (input.find('\n').unwrap() + 1) as isize;
    let mut antennas_map: HashMap<u8, Vec<usize>> = HashMap::new();
    input
        .as_bytes()
        .iter()
        .enumerate()
        .for_each(|(i, &antenna)| {
            if ![b'.', b'\n'].contains(&antenna) {
                if let Some(antenna_list) = antennas_map.get_mut(&antenna) {
                    antenna_list.push(i);
                } else {
                    antennas_map.insert(antenna, vec![i]);
                };
            }
        });

    let mut antinodes_set: HashSet<usize> = HashSet::new();
    antennas_map.iter().for_each(|(_, antenna_list)| {
        if antenna_list.len() > 2 {
            for (i, &antenna1) in antenna_list[..antenna_list.len() - 1].iter().enumerate() {
                for &antenna2 in antenna_list[i + 1..].iter() {
                    antinodes_set.insert(antenna1);
                    antinodes_set.insert(antenna2);
                    let diff = antenna2 - antenna1;
                    let antenna_offset =
                        antenna2 as isize % line_len - antenna1 as isize % line_len;
                    let mut count: usize = 1;
                    loop {
                        let antinode1 = antenna1 as isize - count as isize * diff as isize;
                        let antinode1_offset = antenna1 as isize % line_len - antinode1 % line_len;
                        if antinode1 >= 0
                            && antinode1_offset / count as isize == antenna_offset
                            && input.as_bytes()[antinode1 as usize] != b'\n'
                        {
                            antinodes_set.insert(antinode1 as usize);
                        } else {
                            count = 1;
                            break;
                        }
                        count += 1;
                    }
                    loop {
                        let antinode2 = antenna2 + count * diff;
                        let antinode2_offset =
                            antinode2 as isize % line_len - antenna2 as isize % line_len;
                        if antinode2 < input.len()
                            && antinode2_offset / count as isize == antenna_offset
                            && input.as_bytes()[antinode2] != b'\n'
                        {
                            antinodes_set.insert(antinode2);
                        } else {
                            break;
                        }
                        count += 1;
                    }
                }
            }
        }
    });
    antinodes_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 14);
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, 34);
    }
}
