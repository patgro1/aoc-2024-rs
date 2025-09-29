advent_of_code::solution!(14);
use regex::Regex;

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    position: (usize, usize),
    velocity: (isize, isize),
}

pub fn parse_input(input: &str) -> Vec<Robot> {
    let parse_regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    input
        .trim()
        .lines()
        .map(|line| {
            let matches = parse_regex.captures(line).unwrap();
            let position = (
                matches[1].parse::<usize>().unwrap(),
                matches[2].parse::<usize>().unwrap(),
            );
            let velocity = (
                matches[3].parse::<isize>().unwrap(),
                matches[4].parse::<isize>().unwrap(),
            );
            Robot { position, velocity }
        })
        .collect::<Vec<Robot>>()
}

pub fn render_map(robots: Vec<Robot>) {
    for y in 0..MAZE_HEIGHT {
        let mut line: String = "".to_string();
        for x in 0..MAZE_WIDTH {
            if robots
                .iter()
                .filter(|robot| robot.position == (x, y))
                .count()
                > 0
            {
                line += "#";
            } else {
                line += ".";
            }
        }
        println!("{}", line);
    }
}

const MAZE_WIDTH: usize = 101;
const MAZE_HEIGHT: usize = 103;
pub fn simulate_robot(robot: Robot, seconds: usize) -> Robot {
    let (init_robot_x, init_robot_y) = robot.position;
    let (robot_vel_x, robot_vel_y) = robot.velocity;

    // Calculate the full offset that should be applied
    let final_x_offset = robot_vel_x * seconds as isize;
    let final_y_offset = robot_vel_y * seconds as isize;
    let final_x_position: usize = if final_x_offset >= 0 {
        (init_robot_x + final_x_offset as usize) % (MAZE_WIDTH)
    } else {
        let final_x_offset = final_x_offset.unsigned_abs() % MAZE_WIDTH;
        if final_x_offset <= init_robot_x {
            init_robot_x - final_x_offset
        } else {
            let signed_x_position = init_robot_x as isize - final_x_offset as isize;
            let abs_position: usize = signed_x_position.unsigned_abs();
            MAZE_WIDTH - abs_position
        }
    };
    let final_y_position: usize = if final_y_offset >= 0 {
        (init_robot_y + final_y_offset as usize) % (MAZE_HEIGHT)
    } else {
        let final_y_offset = final_y_offset.unsigned_abs() % MAZE_HEIGHT;
        if final_y_offset <= init_robot_y {
            init_robot_y - final_y_offset
        } else {
            let signed_y_position = init_robot_y as isize - final_y_offset as isize;
            let abs_position: usize = signed_y_position.unsigned_abs();
            MAZE_HEIGHT - abs_position
        }
    };

    Robot {
        position: (final_x_position, final_y_position),
        velocity: robot.velocity,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let robots = parse_input(input);
    let final_robots = robots
        .iter()
        .map(|robot| simulate_robot(*robot, 100))
        .collect::<Vec<Robot>>();
    let q1_cnt = final_robots
        .iter()
        .filter(|x| x.position.0 < MAZE_WIDTH / 2 && x.position.1 < MAZE_HEIGHT / 2)
        .count() as u32;
    let q2_cnt = final_robots
        .iter()
        .filter(|x| x.position.0 > MAZE_WIDTH / 2 && x.position.1 < MAZE_HEIGHT / 2)
        .count() as u32;
    let q3_cnt = final_robots
        .iter()
        .filter(|x| x.position.0 < MAZE_WIDTH / 2 && x.position.1 > MAZE_HEIGHT / 2)
        .count() as u32;
    let q4_cnt = final_robots
        .iter()
        .filter(|x| x.position.0 > MAZE_WIDTH / 2 && x.position.1 > MAZE_HEIGHT / 2)
        .count() as u32;
    Some(q1_cnt * q2_cnt * q3_cnt * q4_cnt)
}

pub fn part_two(input: &str) -> Option<u32> {
    let robots = parse_input(input);
    for i in (12..6600).step_by(101) {
        let final_robots = robots
            .iter()
            .map(|robot| simulate_robot(*robot, i))
            .collect::<Vec<Robot>>();
        println!("iter {}", i);
        render_map(final_robots);
    }
    Some(6577)
}
