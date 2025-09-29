advent_of_code::solution!(13);

use rayon::prelude::*;
use regex::Regex;
use std::cmp::min;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Machine {
    button_a_offset: (i64, i64),
    button_b_offset: (i64, i64),
    prize: (i64, i64),
}

const BUTTON_A_PRICE: i64 = 3;
const BUTTON_B_PRICE: i64 = 1;

pub fn parse_input(input: &str, prize_offset: Option<i64>) -> Vec<Machine> {
    let prize_offset = prize_offset.unwrap_or(0);
    let mut machines: Vec<Machine> = vec![];
    let button_regex = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let lines: Vec<_> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    for chunk in lines.chunks(3) {
        let button_a = button_regex.captures(chunk[0]).unwrap();
        let button_a_offset = (
            button_a[1].parse::<i64>().unwrap(),
            button_a[2].parse::<i64>().unwrap(),
        );
        let button_b = button_regex.captures(chunk[1]).unwrap();
        let button_b_offset = (
            button_b[1].parse::<i64>().unwrap(),
            button_b[2].parse::<i64>().unwrap(),
        );
        let prize = prize_regex.captures(chunk[2]).unwrap();
        let prize_coord = (
            prize[1].parse::<i64>().unwrap() + prize_offset,
            prize[2].parse::<i64>().unwrap() + prize_offset,
        );

        machines.push(Machine {
            button_a_offset,
            button_b_offset,
            prize: prize_coord,
        });
    }

    machines
}
pub fn find_min_machine_solution_equation(machine: &Machine) -> i64 {
    /* This is a 2 var 2 equations system... the formula can be proved on paper... here are the
     * final equations:
     * M = (TxYa-TyXa)/(XbYa-YbXa)
     * N = (Tx-MXb) / Xa
     * If M and N are not integer, we do not have a solution
     */
    let (tx, ty) = machine.prize;
    let (xa, ya) = machine.button_a_offset;
    let (xb, yb) = machine.button_b_offset;
    let num: i64 = tx * ya - ty * xa;
    let den: i64 = xb * ya - yb * xa;
    // If den does not divide num, no solution with integers
    if num % den != 0 {
        return 0;
    }
    let m = num / den;

    let num: i64 = tx - m * xb;
    let den: i64 = xa;
    if num % den != 0 {
        return 0;
    }
    let n: i64 = num / den;
    if m < 0 || n < 0 {
        return 0;
    }
    m * BUTTON_B_PRICE + n * BUTTON_A_PRICE
}

pub fn find_min_machine_solution_walkback_mult_only(machine: &Machine) -> i64 {
    /* Here we will do the following... since the move order between A and B is not
     * important, (5B + A) == (3B + A + 2B), and we know B are more cost effective,
     * we can now just start the with X b moves, where x is the highest number B moves we can do
     * before going over. Then we check if we can reach target by an integer amount of a moves. If
     * we can, we found the optimal solution, else we reduce number of b moves by one and try again
     */
    let (target_x, target_y) = machine.prize;
    let (a_offset_x, a_offset_y) = machine.button_a_offset;
    let (b_offset_x, b_offset_y) = machine.button_b_offset;
    let mut number_of_b_moves: i64 = min(target_x / b_offset_x, target_y / b_offset_y)
        .try_into()
        .unwrap();
    while number_of_b_moves >= 0 {
        let coord_x = number_of_b_moves as i64 * b_offset_x as i64;
        let coord_y = number_of_b_moves as i64 * b_offset_y as i64;
        let delta_x = target_x as i64 - coord_x;
        let delta_y = target_y as i64 - coord_y;
        let mod_x_a = delta_x % a_offset_x as i64;
        let mod_y_a = delta_y % a_offset_y as i64;
        let div_x_a = delta_x / a_offset_x as i64;
        let div_y_a = delta_y / a_offset_y as i64;
        if mod_x_a == 0 && mod_y_a == 0 && div_x_a == div_y_a {
            return number_of_b_moves as i64 * BUTTON_B_PRICE + div_x_a * BUTTON_A_PRICE;
        }
        number_of_b_moves -= 1;
    }
    0
}
pub fn find_min_machine_solution_walkback(machine: &Machine) -> i64 {
    /* Here we will do the following... since the move order between A and B is not
     * important, (5B + A) == (3B + A + 2B), and we know B are more cost effective,
     * we can now just start the with X b moves, where x is the highest number B moves we can do
     * before going over. Then, we try to reach the target with only A moves. If it works, solution
     * found. Else, we backtrack by one B move and try again until we either get back to origin OR
     * get to target. This guarantees to always give the optimal solution */
    let (target_x, target_y) = machine.prize;
    let (a_offset_x, a_offset_y) = machine.button_a_offset;
    let (b_offset_x, b_offset_y) = machine.button_b_offset;
    let mut number_of_b_moves = min(target_x / b_offset_x, target_y / b_offset_y);
    let mut number_of_a_moves = 0;
    loop {
        // Set ourselves to current number of b moves forward
        let mut coord = (
            number_of_b_moves * b_offset_x,
            number_of_b_moves * b_offset_y,
        );
        while coord.0 < target_x && coord.1 < target_y {
            coord.0 += a_offset_x;
            coord.1 += a_offset_y;
            number_of_a_moves += 1;
        }
        if coord.0 == target_x && coord.1 == target_y {
            return BUTTON_B_PRICE * number_of_b_moves as i64
                + BUTTON_A_PRICE * number_of_a_moves as i64;
        }

        if number_of_b_moves == 0 {
            break;
        }
        number_of_b_moves -= 1;
        number_of_a_moves = 0;
    }
    0
}

pub fn find_min_machine_solution(machine: &Machine) -> i64 {
    let mut min_price: i64 = i64::MAX;
    let mut queue: Vec<((i64, i64), i64, i64)> = vec![];
    let mut visited: HashMap<(i64, i64), i64> = HashMap::new();
    // We always start at 0,0 so we enqueue that before. Since A move are more expensive than B
    // move, we prioritize these one.
    let a_move_coord = machine.button_a_offset;
    let b_move_coord = machine.button_b_offset;
    queue.push((a_move_coord, BUTTON_A_PRICE, 1));
    queue.push((b_move_coord, BUTTON_B_PRICE, 1));

    while let Some(step) = queue.pop() {
        let (coord, cost, steps) = step;
        // Did we ever get here at a lower cost
        if let Some(visited_cost) = visited.get(&coord) {
            if cost >= *visited_cost {
                continue;
            }
        }
        visited.insert(coord, cost);

        if coord == machine.prize && cost < min_price {
            min_price = cost;
        } else if coord.0 > machine.prize.0 || coord.1 > machine.prize.1 {
            // Since we cant go back, check if we are passed the prized on either x or y
            continue;
        } else {
            let a_move_coord = (
                coord.0 + machine.button_a_offset.0,
                coord.1 + machine.button_a_offset.1,
            );
            let b_move_coord = (
                coord.0 + machine.button_b_offset.0,
                coord.1 + machine.button_b_offset.1,
            );
            queue.push((a_move_coord, cost + BUTTON_A_PRICE, steps + 1));
            queue.insert(0, (b_move_coord, cost + BUTTON_B_PRICE, steps + 1));
        }
    }
    // We use the max value as an init value... if we are still on that, we did not find a solution
    // and we need to get out with 0 token used
    if min_price == i64::MAX {
        return 0;
    }
    min_price
}

pub fn part_one(input: &str) -> Option<i64> {
    let machines = parse_input(input, None);
    // Some(find_min_machine_solution_walkback_mult_only(&machines[1]))

    Some(
        machines
            .par_iter()
            .map(find_min_machine_solution_equation)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let machines = parse_input(input, Some(10000000000000));
    Some(
        machines
            .par_iter()
            .map(find_min_machine_solution_equation)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(239021));
    // }
}
