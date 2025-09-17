advent_of_code::solution!(8);

use std::collections::HashMap;
use std::collections::HashSet;

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

pub fn find_all_antennas(input: Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antenna_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (y, l) in input.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c != '.' {
                if let Some(simple_antenna_vec) = antenna_map.get_mut(c) {
                    simple_antenna_vec.push((x, y));
                } else {
                    antenna_map.insert(*c, vec![(x, y)]);
                }
            }
        }
    }
    antenna_map
}

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle = parse_input(input);
    let height = puzzle.len() as i32;
    let width = puzzle[0].len() as i32;
    let antenna_map = find_all_antennas(puzzle);

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for antennas_type in antenna_map {
        let locations = antennas_type.1;
        for (idx, location) in locations.iter().enumerate() {
            for second_antenna in &locations[idx + 1..] {
                let x_dis = location.0 as i32 - second_antenna.0 as i32;
                let y_dis = location.1 as i32 - second_antenna.1 as i32;
                // Add the first possible node from the node we are looking
                let new_x = location.0 as i32 + x_dis;
                let new_y = location.1 as i32 + y_dis;
                if new_x >= 0 && new_x < width && new_y >= 0 && new_y < height {
                    let new_x = new_x.try_into().unwrap();
                    let new_y = new_y.try_into().unwrap();
                    antinodes.insert((new_x, new_y));
                }
                // Add the second possible node from the second node we are looking
                // Add the first possible node from the node we are looking
                let new_x = second_antenna.0 as i32 - x_dis;
                let new_y = second_antenna.1 as i32 - y_dis;
                if new_x >= 0 && new_x < width && new_y >= 0 && new_y < height {
                    let new_x = new_x.try_into().unwrap();
                    let new_y = new_y.try_into().unwrap();
                    antinodes.insert((new_x, new_y));
                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let puzzle = parse_input(input);
    let height = puzzle.len() as i32;
    let width = puzzle[0].len() as i32;
    let antenna_map = find_all_antennas(puzzle);

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for antennas_type in antenna_map {
        let locations = antennas_type.1;
        for location in &locations {
            for second_antenna in locations.clone().into_iter().filter(|x| *x != *location) {
                let x_dis = location.0 as i32 - second_antenna.0 as i32;
                let y_dis = location.1 as i32 - second_antenna.1 as i32;
                // Add the first possible node from the node we are looking
                let mut new_x = location.0 as i32 + x_dis;
                let mut new_y = location.1 as i32 + y_dis;

                while new_x >= 0 && new_x < width && new_y >= 0 && new_y < height {
                    let new_x_idx = new_x.try_into().unwrap();
                    let new_y_idx = new_y.try_into().unwrap();
                    antinodes.insert((new_x_idx, new_y_idx));
                    new_x += x_dis;
                    new_y += y_dis;
                }
                // Add the second possible node from the second node we are looking
                // Add the first possible node from the node we are looking
                let mut new_x = location.0 as i32 - x_dis;
                let mut new_y = location.1 as i32 - y_dis;
                while new_x >= 0 && new_x < width && new_y >= 0 && new_y < height {
                    let new_x_idx = new_x.try_into().unwrap();
                    let new_y_idx = new_y.try_into().unwrap();
                    antinodes.insert((new_x_idx, new_y_idx));
                    new_x -= x_dis;
                    new_y -= y_dis;
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
