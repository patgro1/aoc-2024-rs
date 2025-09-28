advent_of_code::solution!(12);

use std::collections::{HashMap, HashSet};

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

#[derive(Debug)]
pub struct Plot {
    species: char,
    coord: (usize, usize),
    other_species_neighbours: u32,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Direction {
    NW,
    N,
    NE,
    W,
    E,
    SW,
    S,
    SE,
}

impl Direction {
    fn offset(self) -> (i32, i32) {
        match self {
            Direction::NW => (-1, -1),
            Direction::N => (0, -1),
            Direction::NE => (1, -1),
            Direction::W => (-1, 0),
            Direction::E => (1, 0),
            Direction::SW => (-1, 1),
            Direction::S => (0, 1),
            Direction::SE => (1, 1),
        }
    }

    fn all() -> [Direction; 8] {
        [
            Direction::NW,
            Direction::N,
            Direction::NE,
            Direction::W,
            Direction::E,
            Direction::SW,
            Direction::S,
            Direction::SE,
        ]
    }
}

/*
 R R R R I
 R R R R I
 V V R R R
 V V R C C

Perimeter of R: 18
Sides of R: 10

*/
pub fn create_region(
    starting_coord: (usize, usize),
    map: &[Vec<char>],
    current_region_nodes: &mut HashSet<(usize, usize)>,
) -> Vec<Plot> {
    let mut region: Vec<Plot> = vec![];
    let (x, y) = starting_coord;
    let species = map[y][x];
    let mut other_species_neighbours = 0;

    let width = map[0].len();
    let height = map.len();

    current_region_nodes.insert(starting_coord);

    // Perimeter check
    if y > 0 && map[y - 1][x] == species {
        if !current_region_nodes.contains(&(x, y - 1)) {
            region.extend(create_region((x, y - 1), map, current_region_nodes))
        }
    } else {
        other_species_neighbours += 1;
    }
    if y < height - 1 && map[y + 1][x] == species {
        if !current_region_nodes.contains(&(x, y + 1)) {
            region.extend(create_region((x, y + 1), map, current_region_nodes))
        }
    } else {
        other_species_neighbours += 1;
    }
    if x > 0 && map[y][x - 1] == species {
        if !current_region_nodes.contains(&(x - 1, y)) {
            region.extend(create_region((x - 1, y), map, current_region_nodes))
        }
    } else {
        other_species_neighbours += 1;
    }
    if x < width - 1 && map[y][x + 1] == species {
        if !current_region_nodes.contains(&(x + 1, y)) {
            region.extend(create_region((x + 1, y), map, current_region_nodes))
        }
    } else {
        other_species_neighbours += 1;
    }

    region.push(Plot {
        species,
        coord: starting_coord,
        other_species_neighbours,
    });

    region
}

pub fn find_corners(region: &Vec<Plot>) -> u32 {
    let mut corners = 0;

    for space in region {
        let (x, y) = space.coord;

        let mut neib_map: HashMap<Direction, bool> = HashMap::new();
        for direction in Direction::all() {
            let (offset_x, offset_y) = direction.offset();
            if let (Some(key_x), Some(key_y)) = (
                x.checked_add_signed(offset_x as isize),
                y.checked_add_signed(offset_y as isize),
            ) {
                let key = (key_x, key_y);
                if region
                    .iter()
                    .filter(|x| x.coord == key)
                    .collect::<Vec<_>>()
                    .is_empty()
                {
                    neib_map.insert(direction, false);
                } else {
                    neib_map.insert(direction, true);
                }
            } else {
                neib_map.insert(direction, false);
            }
        }
        if neib_map[&Direction::N] && neib_map[&Direction::W] && !neib_map[&Direction::NW] {
            corners += 1;
        }
        if neib_map[&Direction::N] && neib_map[&Direction::E] && !neib_map[&Direction::NE] {
            corners += 1;
        }
        if neib_map[&Direction::S] && neib_map[&Direction::W] && !neib_map[&Direction::SW] {
            corners += 1;
        }
        if neib_map[&Direction::S] && neib_map[&Direction::E] && !neib_map[&Direction::SE] {
            corners += 1;
        }
        if !(neib_map[&Direction::N] || neib_map[&Direction::W]) {
            corners += 1;
        }
        if !(neib_map[&Direction::N] || neib_map[&Direction::E]) {
            corners += 1;
        }
        if !(neib_map[&Direction::S] || neib_map[&Direction::W]) {
            corners += 1;
        }
        if !(neib_map[&Direction::S] || neib_map[&Direction::E]) {
            corners += 1;
        }

        // Check all the surroundings to see oif we are in the same region
    }

    corners
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let mut cost = 0;
    let mut hash_map: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let pos = (x, y);
            if !hash_map.contains(&pos) {
                let region = create_region(pos, &map, &mut hash_map);
                let region_cost = region
                    .iter()
                    .map(|x| x.other_species_neighbours)
                    .sum::<u32>()
                    * region.len() as u32;
                cost += region_cost;
            }
        }
    }
    Some(cost)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let mut cost = 0;
    let mut hash_map: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let pos = (x, y);
            if !hash_map.contains(&pos) {
                let region = create_region(pos, &map, &mut hash_map);
                let corners = find_corners(&region);
                cost += corners * region.len() as u32;
            }
        }
    }
    Some(cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
