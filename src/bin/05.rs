use std::ops::Index;

advent_of_code::solution!(5);

pub fn parse_data(input: &str) -> (Vec<Vec<u32>>, Vec<(u32, u32)>) {
    let mut parsing_updates = false;
    let mut rules: Vec<(u32, u32)> = vec![];
    let mut updates: Vec<Vec<u32>> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            parsing_updates = true;
            continue;
        }

        if !parsing_updates {
            let t: Vec<_> = line.split('|').collect();
            let f = t[0].parse::<u32>().unwrap();
            let s = t[1].parse::<u32>().unwrap();
            rules.push((f, s));
        } else {
            updates.push(line.split(',').map(|x| x.parse::<u32>().unwrap()).collect());
        }
    }

    (updates, rules)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (updates, rules) = parse_data(input);
    let mut accum: u32 = 0;

    'update_loop: for update in updates {
        for (idx, page) in update.iter().enumerate() {
            let specific_rules = rules
                .iter()
                .filter(|(first, _)| first == page)
                .cloned()
                .filter(|(_, second)| update[0..idx].contains(second))
                .collect::<Vec<(u32, u32)>>();
            if !specific_rules.is_empty() {
                continue 'update_loop;
            }
        }
        accum += update[update.len() / 2];
    }
    Some(accum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (updates, rules) = parse_data(input);
    let mut accum: u32 = 0;
    for mut update in updates {
        let mut curr_page_idx: usize = 0;
        let mut fixed = false;
        while curr_page_idx < update.len() {
            let specific_rules = rules
                .iter()
                .filter(|(first, _)| *first == update[curr_page_idx])
                .cloned()
                .filter(|(_, second)| update[0..curr_page_idx].contains(second))
                .collect::<Vec<(u32, u32)>>();
            // We swap the two elements that failed
            if !specific_rules.is_empty() {
                fixed = true;
                let failed_idx = update
                    .iter()
                    .position(|x| *x == specific_rules[0].1)
                    .expect("We know it exists");
                update.swap(curr_page_idx, failed_idx);
                curr_page_idx = failed_idx;
            } else {
                curr_page_idx += 1;
            }
        }
        if fixed {
            accum += update[update.len() / 2];
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
