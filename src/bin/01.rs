advent_of_code::solution!(1);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut splited = line.split_whitespace();

        left_list.push(
            splited
                .next()
                .expect("Should be an uint")
                .parse::<i32>()
                .unwrap(),
        );
        right_list.push(
            splited
                .next()
                .expect("Should be an uint")
                .parse::<i32>()
                .unwrap(),
        );
    }
    left_list.sort();
    right_list.sort();
    Some(
        left_list
            .iter()
            .zip(right_list.iter())
            .map(|(x, y)| (x - y).unsigned_abs())
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left_list: HashMap<u32, usize> = HashMap::new();
    let mut right_list: HashMap<u32, usize> = HashMap::new();
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut splited = line.split_whitespace();
        left_list
            .entry(
                splited
                    .next()
                    .expect("Should be a uint")
                    .parse::<u32>()
                    .unwrap(),
            )
            .and_modify(|count| *count += 1)
            .or_insert(1);
        right_list
            .entry(
                splited
                    .next()
                    .expect("Should be a uint")
                    .parse::<u32>()
                    .unwrap(),
            )
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    Some(
        left_list
            .iter()
            .map(|(key, count)| key * *count as u32 * hash_map_get_or_0(&right_list, *key))
            .sum(),
    )
}

pub fn hash_map_get_or_0(hash_map: &HashMap<u32, usize>, key: u32) -> u32 {
    match hash_map.get(&key) {
        Some(count) => *count as u32,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
