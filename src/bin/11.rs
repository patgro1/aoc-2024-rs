advent_of_code::solution!(11);

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

type RockCache = Mutex<HashMap<(u64, u8), u64>>;

static GLOBAL_CACHE: OnceLock<RockCache> = OnceLock::new();

pub fn get_cache() -> &'static RockCache {
    GLOBAL_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|x| x.parse::<u64>().expect("All inputs must be numbers"))
        .collect::<Vec<u64>>()
}

pub fn cnt_new_rocks(num: u64, step: u8) -> u64 {
    if step == 0 {
        return 1;
    }
    let cache = get_cache();
    {
        let cache_guard = cache.lock().expect("Failed getting the cache");
        if let Some(cnt) = cache_guard.get(&(num, step)) {
            return *cnt;
        }
    }
    let res: u64;
    let len = num.to_string().len();
    if num == 0 {
        res = cnt_new_rocks(1, step - 1);
    } else if len.is_multiple_of(2) {
        res = cnt_new_rocks(
            num.to_string()[0..len / 2]
                .parse::<u64>()
                .expect("Should be a number"),
            step - 1,
        ) + cnt_new_rocks(
            num.to_string()[len / 2..]
                .parse::<u64>()
                .expect("Should be a number"),
            step - 1,
        );
    } else {
        res = cnt_new_rocks(num * 2024, step - 1);
    }
    let mut cache_guard = cache.lock().unwrap();
    cache_guard.insert((num, step), res);
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let puzzle = parse_input(input);
    Some(puzzle.iter().map(|x| cnt_new_rocks(*x, 25)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let puzzle = parse_input(input);
    Some(puzzle.iter().map(|x| cnt_new_rocks(*x, 75)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
