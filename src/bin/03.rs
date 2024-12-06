use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    make_mult(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    let dont_re = Regex::new(r"(mul\((?<op1>\d{1,3}),(?<op2>\d{1,3})\))|(?<dt>don't\(\))|(?<d>do\(\))").unwrap();
    let matches = dont_re.captures_iter(input);
    let mut enabled = true;
    let mut sum = 0;
    for mat in matches {
        if mat.name("dt").is_some() {
            enabled = false;
        } else if mat.name("d").is_some() {
            enabled = true;
        }

        if enabled {
            if let (Some(op1), Some(op2)) = (mat.name("op1"), mat.name("op2")) {
                sum += op1.as_str().parse::<u32>().unwrap() * op2.as_str().parse::<u32>().unwrap();
            }
        }
    }
    Some(sum)
}

pub fn make_mult(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let matches = re.captures_iter(input);
    Some(matches.map(|x| x[1].parse::<u32>().unwrap() * x[2].parse::<u32>().unwrap()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
