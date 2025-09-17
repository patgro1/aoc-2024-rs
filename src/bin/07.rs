advent_of_code::solution!(7);

use rayon::prelude::*;

#[derive(Debug)]
pub struct Equation {
    result: u64,
    numbers: Vec<u64>,
    operations: Vec<Operator>,
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Mult,
    Concat,
}

pub fn parse_input(input: &str) -> Vec<Equation> {
    let mut eq_list: Vec<Equation> = vec![];
    for line in input.lines() {
        let mut split_line_iter = line.split(':').filter(|x| !x.is_empty());
        let result = split_line_iter
            .next()
            .expect("We should have a result")
            .parse::<u64>()
            .expect("Result should be a u64");
        let numbers = split_line_iter
            .next()
            .expect("We should have some numbers")
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u64>().expect("We should have a number"))
            .collect();
        eq_list.push(Equation {
            result,
            numbers,
            operations: vec![],
        });
    }
    eq_list
}

pub fn create_all_permutations(
    list: Vec<Vec<Operator>>,
    len: usize,
    use_concat: bool,
) -> Vec<Vec<Operator>> {
    if len == 0 {
        return list;
    }

    let mut new_list: Vec<Vec<Operator>> = vec![];

    for comb in list {
        // Create the add
        let mut new_internal_list = comb.clone();
        new_internal_list.extend(vec![Operator::Add]);
        new_list.push(new_internal_list);

        // Create the mult
        new_internal_list = comb.clone();
        new_internal_list.extend(vec![Operator::Mult]);
        new_list.push(new_internal_list);

        if use_concat {
            // Create the mult
            new_internal_list = comb.clone();
            new_internal_list.extend(vec![Operator::Concat]);
            new_list.push(new_internal_list);
        }
    }
    create_all_permutations(new_list, len - 1, use_concat)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut accum: u64 = 0;
    let equations = parse_input(input);
    for equation in equations {
        // For every equation create all possible combination of the operators
        let all_operators =
            create_all_permutations(vec![vec![]], equation.numbers.len() - 1, false);

        // In parallel, evaluate all equations to see if one will get to the result
        let matching_comb = all_operators
            .par_iter()
            .map(|x| {
                let mut result: u64 = equation.numbers[0];
                for (idx, operator) in x.iter().enumerate() {
                    result = match operator {
                        Operator::Add => result + equation.numbers[idx + 1],
                        Operator::Mult => result * equation.numbers[idx + 1],
                        Operator::Concat => panic!("We should not have concat in part one"),
                    }
                }
                result
            })
            .filter(|x| *x == equation.result)
            .count();
        if matching_comb > 0 {
            accum += equation.result
        }
    }

    Some(accum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    let total = equations
        .par_iter()
        .map(|x| {
            // For every equation create all possible combination of the operators
            let all_operators = create_all_permutations(vec![vec![]], x.numbers.len() - 1, true);
            // println!("All operations combinations: {:?}", all_operators);

            // In parallel, evaluate all equations to see if one will get to the result
            let matching_comb = all_operators
                .par_iter()
                .map(|comb| {
                    let mut result: u64 = x.numbers[0];
                    for (idx, operator) in comb.iter().enumerate() {
                        result = match operator {
                            Operator::Add => result + x.numbers[idx + 1],
                            Operator::Mult => result * x.numbers[idx + 1],
                            Operator::Concat => (result.to_string()
                                + &x.numbers[idx + 1].to_string())
                                .parse()
                                .expect("We should still have a number"),
                        };
                    }
                    result
                })
                .filter(|y| *y == x.result)
                .count();
            if matching_comb > 0 {
                // println!("Found a solution for {:?}", equation);
                x.result
            } else {
                0
            }
        })
        .sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
