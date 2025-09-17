advent_of_code::solution!(4);

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed_input = parse_input(input);
    // Scan the array for X.
    let lines = parsed_input.len();
    let columns = parsed_input[0].len();
    let mut counter = 0;
    for (j, line) in parsed_input.iter().enumerate() {
        for (i, c) in line.iter().enumerate() {
            if c.eq_ignore_ascii_case(&'x') {
                // Check to the left
                if i >= 3
                    && line[i - 1].eq_ignore_ascii_case(&'m')
                    && line[i - 2].eq_ignore_ascii_case(&'a')
                    && line[i - 3].eq_ignore_ascii_case(&'s')
                {
                    counter += 1;
                }
                // Check to the right
                if i < columns - 3
                    && line[i + 1].eq_ignore_ascii_case(&'m')
                    && line[i + 2].eq_ignore_ascii_case(&'a')
                    && line[i + 3].eq_ignore_ascii_case(&'s')
                {
                    counter += 1;
                }
                // Check Up
                if j >= 3
                    && parsed_input[j - 1][i].eq_ignore_ascii_case(&'m')
                    && parsed_input[j - 2][i].eq_ignore_ascii_case(&'a')
                    && parsed_input[j - 3][i].eq_ignore_ascii_case(&'s')
                {
                    counter += 1;
                }
                // Check Down
                if j < lines - 3
                    && parsed_input[j + 1][i].eq_ignore_ascii_case(&'m')
                    && parsed_input[j + 2][i].eq_ignore_ascii_case(&'a')
                    && parsed_input[j + 3][i].eq_ignore_ascii_case(&'s')
                {
                    counter += 1;
                }
                // Check up-left
                if j >= 3
                    && i >= 3
                    && parsed_input[j - 1][i - 1].eq_ignore_ascii_case(&'m')
                    && parsed_input[j - 2][i - 2].eq_ignore_ascii_case(&'a')
                    && parsed_input[j - 3][i - 3].eq_ignore_ascii_case(&'s')
                {
                    counter += 1;
                }

                // Check up-right
                if j >= 3
                    && i < columns - 3
                    && parsed_input[j - 1][i + 1].eq_ignore_ascii_case(&'m')
                    && parsed_input[j - 2][i + 2].eq_ignore_ascii_case(&'a')
                    && parsed_input[j - 3][i + 3].eq_ignore_ascii_case(&'s')
                {
                    counter += 1;
                }
                // Check down-left
                if j < lines - 3
                    && i >= 3
                    && parsed_input[j + 1][i - 1].eq_ignore_ascii_case(&'m')
                    && parsed_input[j + 2][i - 2].eq_ignore_ascii_case(&'a')
                    && parsed_input[j + 3][i - 3].eq_ignore_ascii_case(&'s')
                {
                    counter += 1;
                }
                // check down-right
                if j < lines - 3
                    && i < columns - 3
                    && parsed_input[j + 1][i + 1].eq_ignore_ascii_case(&'m')
                    && parsed_input[j + 2][i + 2].eq_ignore_ascii_case(&'a')
                    && parsed_input[j + 3][i + 3].eq_ignore_ascii_case(&'s')
                {
                    counter += 1;
                }
            }
        }
    }
    Some(counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed_input = parse_input(input);
    // Scan the array for X.
    let lines = parsed_input.len();
    let columns = parsed_input[0].len();
    let mut counter = 0;
    for (j, line) in parsed_input.iter().enumerate() {
        for (i, c) in line.iter().enumerate() {
            if c.eq_ignore_ascii_case(&'a') && i > 0 && i < columns - 1 && j > 0 && j < lines - 1 {
                // C1 and C2 are one diagonal, C3 and C4 are the other
                let c1 = parsed_input[j - 1][i - 1];
                let c2 = parsed_input[j + 1][i + 1];
                let c3 = parsed_input[j - 1][i + 1];
                let c4 = parsed_input[j + 1][i - 1];
                let first_check = (c1.eq_ignore_ascii_case(&'m') || c1.eq_ignore_ascii_case(&'s'))
                    && (c2.eq_ignore_ascii_case(&'m') || c2.eq_ignore_ascii_case(&'s'))
                    && c1 != c2;
                let second_check = (c3.eq_ignore_ascii_case(&'m') || c3.eq_ignore_ascii_case(&'s'))
                    && (c4.eq_ignore_ascii_case(&'m') || c4.eq_ignore_ascii_case(&'s'))
                    && c3 != c4;
                if first_check && second_check {
                    counter += 1;
                }
            }
        }
    }
    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
