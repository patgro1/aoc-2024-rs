advent_of_code::solution!(6);

use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Hash, PartialEq, Clone)]
pub struct Puzzle {
    map: Vec<Vec<char>>,
    guard_direction: Direction,
    guard_position: Option<(usize, usize)>,
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for &ch in row {
                write!(f, "{}", ch)?;
            }
            writeln!(f)?; // newline after each row
        }
        Ok(())
    }
}

pub fn parse_input(input: &str) -> Puzzle {
    let mut map: Vec<Vec<char>> = vec![];
    let mut guard_position: Option<(usize, usize)> = None;
    let guard_direction = Direction::North;
    for (line_idx, line) in input.lines().enumerate() {
        let line = line.chars().collect::<Vec<char>>();
        if let Some(starting_col) = line.iter().position(|x| *x == '^') {
            guard_position = Some((starting_col, line_idx));
        }
        map.push(line);
    }
    Puzzle {
        map,
        guard_direction,
        guard_position,
    }
}

pub fn can_escapce(mut puzzle: Puzzle) -> bool {
    let mut seen_locations: HashSet<(usize, usize, Direction)> = HashSet::new();

    while let Some((guard_x, guard_y)) = puzzle.guard_position {
        if !seen_locations.insert((guard_x, guard_y, puzzle.guard_direction)) {
            return false;
        }

        match puzzle.guard_direction {
            Direction::North => {
                puzzle.guard_position = guard_y.checked_sub(1).map(|new_y| (guard_x, new_y));
            }
            Direction::South => {
                puzzle.guard_position = guard_y.checked_add(1).and_then(|new_y| {
                    if new_y >= puzzle.map.len() {
                        None
                    } else {
                        Some((guard_x, new_y))
                    }
                });
            }
            Direction::West => {
                puzzle.guard_position = guard_x.checked_sub(1).map(|new_x| (new_x, guard_y))
            }
            Direction::East => {
                puzzle.guard_position = guard_x.checked_add(1).and_then(|new_x| {
                    if new_x >= puzzle.map[0].len() {
                        None
                    } else {
                        Some((new_x, guard_y))
                    }
                });
            }
        }
        if let Some((new_x, new_y)) = puzzle.guard_position {
            if puzzle.map[new_y][new_x] == '.' {
                puzzle.map[new_y][new_x] = '^';
                puzzle.map[guard_y][guard_x] = '.';
            } else if puzzle.map[new_y][new_x] == '#' {
                puzzle.guard_position = Some((guard_x, guard_y));
                puzzle.guard_direction = match puzzle.guard_direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                };
            }
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut puzzle = parse_input(input);
    let mut seen_locations: HashSet<(usize, usize)> = HashSet::new();

    while let Some((guard_x, guard_y)) = puzzle.guard_position {
        seen_locations.insert((guard_x, guard_y));

        match puzzle.guard_direction {
            Direction::North => {
                puzzle.guard_position = guard_y.checked_sub(1).map(|new_y| (guard_x, new_y));
            }
            Direction::South => {
                puzzle.guard_position = guard_y.checked_add(1).and_then(|new_y| {
                    if new_y >= puzzle.map.len() {
                        None
                    } else {
                        Some((guard_x, new_y))
                    }
                });
            }
            Direction::West => {
                puzzle.guard_position = guard_x.checked_sub(1).map(|new_x| (new_x, guard_y))
            }
            Direction::East => {
                puzzle.guard_position = guard_x.checked_add(1).and_then(|new_x| {
                    if new_x >= puzzle.map[0].len() {
                        None
                    } else {
                        Some((new_x, guard_y))
                    }
                });
            }
        }
        if let Some((new_x, new_y)) = puzzle.guard_position {
            if puzzle.map[new_y][new_x] == '.' {
                puzzle.map[new_y][new_x] = '^';
                puzzle.map[guard_y][guard_x] = '.';
            } else if puzzle.map[new_y][new_x] == '#' {
                puzzle.guard_position = Some((guard_x, guard_y));
                puzzle.guard_direction = match puzzle.guard_direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                };
            }
        }
    }

    Some(seen_locations.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let puzzle = parse_input(input);
    let mut accum: u32 = 0;
    for (line_idx, line) in puzzle.map.iter().enumerate() {
        for (col_idx, c) in line.iter().enumerate() {
            if *c != '.' {
                continue;
            }
            let mut new_puzzle = puzzle.clone();
            new_puzzle.map[line_idx][col_idx] = '#';
            if !can_escapce(new_puzzle) {
                accum += 1;
            }
        }
    }
    Some(accum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
