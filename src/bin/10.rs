advent_of_code::solution!(10);
use rayon::prelude::*;
use std::collections::HashSet;

pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_digit(10).expect("Puzzle should only containts number") as u8)
                .collect()
        })
        .collect()
}

pub fn find_trail_head(puzzle: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut trail_heads: Vec<(usize, usize)> = vec![];
    for (y, line) in puzzle.iter().enumerate() {
        for (x, val) in line.iter().enumerate() {
            if *val == 0 {
                trail_heads.push((x, y));
            }
        }
    }
    trail_heads
}

pub fn find_trailends(
    trailhead: (usize, usize),
    puzzle: &Vec<Vec<u8>>,
    trailhead_val: u8,
) -> Vec<(usize, usize)> {
    let mut trailends: Vec<(usize, usize)> = vec![];
    let puzzle_max_y = puzzle.len() - 1;
    let puzzle_max_x = puzzle[0].len() - 1;
    if trailhead_val == 9 {
        trailends.push(trailhead);
    } else {
        let (trailhead_x, trailhead_y) = trailhead;
        // Check if we can move in any direction, and recursively extend the trailends vector with
        // find trailends of the place
        if trailhead_x > 0 && puzzle[trailhead_y][trailhead_x - 1] == trailhead_val + 1 {
            trailends.extend(find_trailends(
                (trailhead_x - 1, trailhead_y),
                puzzle,
                trailhead_val + 1,
            ));
        }
        if trailhead_x < puzzle_max_x && puzzle[trailhead_y][trailhead_x + 1] == trailhead_val + 1 {
            trailends.extend(find_trailends(
                (trailhead_x + 1, trailhead_y),
                puzzle,
                trailhead_val + 1,
            ));
        }
        if trailhead_y > 0 && puzzle[trailhead_y - 1][trailhead_x] == trailhead_val + 1 {
            trailends.extend(find_trailends(
                (trailhead_x, trailhead_y - 1),
                puzzle,
                trailhead_val + 1,
            ));
        }
        if trailhead_y < puzzle_max_y && puzzle[trailhead_y + 1][trailhead_x] == trailhead_val + 1 {
            trailends.extend(find_trailends(
                (trailhead_x, trailhead_y + 1),
                puzzle,
                trailhead_val + 1,
            ));
        }
    }

    trailends
}

pub fn uniquify_ends(ends: Vec<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut trail_ends_unique: HashSet<(usize, usize)> = HashSet::new();
    for end in ends.iter() {
        trail_ends_unique.insert(*end);
    }

    trail_ends_unique
}

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle = parse_input(input);
    let trail_heads = find_trail_head(&puzzle);
    Some(
        trail_heads
            .par_iter()
            .map(|x| uniquify_ends(find_trailends(*x, &puzzle, 0)).len())
            .map(|x| x as u32)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let puzzle = parse_input(input);
    let trail_heads = find_trail_head(&puzzle);
    Some(
        trail_heads
            .par_iter()
            .map(|x| find_trailends(*x, &puzzle, 0).len())
            .map(|x| x as u32)
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
