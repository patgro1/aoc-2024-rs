use std::cmp::Ordering;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports = input.split('\n');
    Some(
        reports
            .map(is_safe)
            .filter(|x| *x)
            .count()
            .try_into()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = input.split('\n');
    Some(
        reports
            .map(is_any_permutation_safe)
            .filter(|x| *x)
            .count()
            .try_into()
            .unwrap(),
    )
}

pub fn is_any_permutation_safe(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    if is_safe(input) {
        return true;
    }
    let v_reports: Vec<_> = input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    for skip_index in 0..v_reports.len() {
        if is_safe(
            &v_reports
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != skip_index)
                .map(|(_, v)| v.to_string())
                .collect::<Vec<_>>()
                .join(" "),
        ) {
            return true;
        }
    }
    false
}

pub fn is_safe(reports: &str) -> bool {
    if reports.is_empty() {
        return false;
    }
    let mut safe = true;
    let mut v_reports = reports
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let mut increasing = false;
    let mut last_val = v_reports
        .next()
        .expect("THere should be at least one value");
    let mut first_check = true;

    for val in v_reports {
        if first_check {
            match val.cmp(&last_val) {
                Ordering::Greater => {
                    increasing = true;
                    first_check = true;
                }
                Ordering::Less => {
                    increasing = false;
                    first_check = false;
                }
                Ordering::Equal => {}
            }
        }
        if !level_is_valid(last_val, val, increasing) {
            safe = false;
            break;
        }

        last_val = val;
    }
    safe
}

pub fn level_is_valid(last_level: u32, current_level: u32, increasing: bool) -> bool {
    if increasing && last_level > current_level {
        return false;
    }
    if !increasing && last_level < current_level {
        return false;
    }
    let delta = (current_level as i32 - last_level as i32).abs();
    (1..=3).contains(&delta)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
