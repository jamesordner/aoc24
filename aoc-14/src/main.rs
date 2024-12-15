use std::{array::from_fn, iter::zip};

type Coord = [isize; 2];

const BOUNDS: [isize; 2] = [101, 103];
const HALF_BOUNDS: [isize; 2] = [BOUNDS[0] / 2, BOUNDS[1] / 2];

#[derive(Clone, Copy, Debug)]
struct Robot {
    position: Coord,
    velocity: Coord,
}

fn main() {
    let mut robots = parse_input(include_str!("../input"));

    println!("{}", part_one(&robots));
    println!("{}", part_two(&mut robots));

    // bonus: print tree
    for y in 0..BOUNDS[1] {
        for x in 0..BOUNDS[0] {
            if contains_robot([x, y], &robots) {
                print!("X");
            } else {
                print!(" ");
            }
        }

        println!();
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once('=').unwrap();
            let (position_x, line) = line.split_once(',').unwrap();
            let (position_y, line) = line.split_once(' ').unwrap();
            let (_, line) = line.split_once('=').unwrap();
            let (velocity_x, velocity_y) = line.split_once(',').unwrap();

            Robot {
                position: [position_x.parse().unwrap(), position_y.parse().unwrap()],
                velocity: [velocity_x.parse().unwrap(), velocity_y.parse().unwrap()],
            }
        })
        .collect()
}

fn part_one(robots: &[Robot]) -> usize {
    robots
        .iter()
        .fold([0; 4], |mut quadrant_tallies, robot| {
            let position = from_fn(|i| robot.position[i] + robot.velocity[i] * 100);

            if let Some(i) = quadrant_index(position) {
                quadrant_tallies[i] += 1;
            }

            quadrant_tallies
        })
        .into_iter()
        .reduce(|acc, val| acc * val)
        .unwrap()
}

fn quadrant_index(coord: Coord) -> Option<usize> {
    let coord = [coord[0].rem_euclid(101), coord[1].rem_euclid(103)];

    if coord[0] < HALF_BOUNDS[0] && coord[1] < HALF_BOUNDS[1] {
        Some(0)
    } else if coord[0] > HALF_BOUNDS[0] && coord[1] < HALF_BOUNDS[1] {
        Some(1)
    } else if coord[0] < HALF_BOUNDS[0] && coord[1] > HALF_BOUNDS[1] {
        Some(2)
    } else if coord[0] > HALF_BOUNDS[0] && coord[1] > HALF_BOUNDS[1] {
        Some(3)
    } else {
        None
    }
}

/// After realizing the tree is not centered, it's a miracle this approach worked
/// and there's no guarantee it will work with other inputs!
fn part_two(robots: &mut [Robot]) -> usize {
    let mut i = 0;

    loop {
        for robot in robots.iter_mut() {
            robot.position[0] += robot.velocity[0];
            robot.position[1] += robot.velocity[1];
            robot.position = from_fn(|i| robot.position[i].rem_euclid(BOUNDS[i]));
        }

        i += 1;

        let vertical_correlation = vertical_correlation(robots);
        let horizontal_correlation = horizontal_correlation(robots);

        if vertical_correlation > 4800 || horizontal_correlation > 4800 {
            return i;
        }
    }
}

fn vertical_correlation(robots: &[Robot]) -> usize {
    let mut correlation = 0;
    for y in 0..BOUNDS[1] {
        for (x_l, x_r) in zip(0..HALF_BOUNDS[0], (HALF_BOUNDS[0] + 1..BOUNDS[0]).rev()) {
            if contains_robot([x_l, y], robots) == contains_robot([x_r, y], robots) {
                correlation += 1;
            }
        }
    }

    correlation
}

fn horizontal_correlation(robots: &[Robot]) -> usize {
    let mut correlation = 0;
    for x in 0..BOUNDS[0] {
        for (y_l, y_r) in zip(0..HALF_BOUNDS[1], (HALF_BOUNDS[1] + 1..BOUNDS[1]).rev()) {
            if contains_robot([x, y_l], robots) == contains_robot([x, y_r], robots) {
                correlation += 1;
            }
        }
    }

    correlation
}

fn contains_robot(coord: Coord, robots: &[Robot]) -> bool {
    robots.iter().any(|robot| robot.position == coord)
}
