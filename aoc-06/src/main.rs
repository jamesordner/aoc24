use std::{array::from_fn, collections::HashSet, fs::read_to_string};

use rayon::prelude::*;

fn main() {
    let input = read_to_string("input").unwrap();
    let (grid, starting_coord) = parse_input(&input);

    println!("{}", part_one(grid, starting_coord));
    println!("{}", part_two(grid, starting_coord));
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Position {
    Empty,
    Obstacle,
    Visited,
}

const GRID_LEN: usize = 130;
type Grid = [[Position; GRID_LEN]; GRID_LEN];
type Coord = [isize; 2];

fn parse_input(input: &str) -> (Grid, Coord) {
    let mut coord = None;
    let mut lines = input.lines();
    let grid = from_fn(|y| {
        let bytes = lines.next().unwrap().as_bytes();
        from_fn(|x| match bytes[x] {
            b'.' => Position::Empty,
            b'#' => Position::Obstacle,
            b'^' => {
                coord = Some([x as isize, y as isize]);
                Position::Empty
            }
            _ => unreachable!(),
        })
    });

    (grid, coord.unwrap())
}

fn part_one(mut grid: Grid, mut coord: Coord) -> u32 {
    let mut count = 0;
    let mut dir = [0, -1];

    while let Some(position) = get_mut(&mut grid, coord) {
        if *position == Position::Empty {
            *position = Position::Visited;
            count += 1;
        }

        step_coord(&grid, &mut coord, &mut dir);
    }

    count
}

fn part_two(grid: Grid, starting_coord: Coord) -> usize {
    (0..GRID_LEN)
        .into_par_iter()
        .flat_map(|x| (0..GRID_LEN).into_par_iter().map(move |y| [x, y]))
        .filter(|coord| grid[coord[1]][coord[0]] == Position::Empty)
        .filter(|coord| {
            let mut grid = grid;
            grid[coord[1]][coord[0]] = Position::Obstacle;

            forms_loop(&grid, starting_coord)
        })
        .count()
}

fn step_coord(grid: &Grid, coord: &mut Coord, dir: &mut Coord) {
    loop {
        let try_coord = [coord[0] + dir[0], coord[1] + dir[1]];

        if get(grid, try_coord).is_some_and(|p| *p == Position::Obstacle) {
            *dir = [-dir[1], dir[0]]; // rotate
            continue;
        }

        *coord = try_coord;

        break;
    }
}

fn forms_loop(grid: &Grid, mut coord: Coord) -> bool {
    let mut visited = HashSet::new();
    let mut dir = [0, -1];

    while get(grid, coord).is_some() {
        if !visited.insert((coord, dir)) {
            return true;
        }

        step_coord(grid, &mut coord, &mut dir);
    }

    false
}

fn get(grid: &Grid, coord: Coord) -> Option<&Position> {
    grid.get(coord[1] as usize)
        .and_then(|row| row.get(coord[0] as usize))
}

fn get_mut(grid: &mut Grid, coord: Coord) -> Option<&mut Position> {
    grid.get_mut(coord[1] as usize)
        .and_then(|row| row.get_mut(coord[0] as usize))
}
