use std::{array::from_fn, fs::read_to_string};

fn main() {
    let input = read_to_string("input").unwrap();
    let grid = parse_input(&input);

    println!("{}", part_one(&grid));

    println!("{}", part_two(&grid));
}

const GRID_SIZE: usize = 140;

type Grid = [[u8; GRID_SIZE]; GRID_SIZE];

fn parse_input(input: &str) -> Grid {
    let mut lines = input.lines();

    from_fn(|_| {
        let mut bytes = lines.next().unwrap().as_bytes().iter();
        from_fn(|_| *bytes.next().unwrap())
    })
}

fn part_one(grid: &Grid) -> usize {
    const PATTERN: [u8; 4] = [b'X', b'M', b'A', b'S'];

    /// Offsets for each direction
    const OFFSETS: [[isize; 2]; 8] = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];

    (0..GRID_SIZE as isize)
        .flat_map(|x| (0..GRID_SIZE as isize).map(move |y| [x, y]))
        .flat_map(|coord| OFFSETS.iter().map(move |&offset| (coord, offset)))
        // this iterates over all possible directions, over all possible starting [x, y] coords
        .filter(|&(mut coord, offset)| {
            for char in PATTERN {
                if !get(grid, coord).is_some_and(|c| c == char) {
                    return false;
                }

                coord[0] += offset[0];
                coord[1] += offset[1];
            }

            true
        })
        .count()
}

fn part_two(grid: &Grid) -> usize {
    const PATTERN: [u8; 2] = [b'M', b'S'];

    /// Two sets of opposite corners
    const CORNER_OFFSETS: [[[isize; 2]; 2]; 2] = [[[-1, -1], [1, 1]], [[-1, 1], [1, -1]]];

    (0..GRID_SIZE as isize)
        .flat_map(|x| (0..GRID_SIZE as isize).map(move |y| [x, y]))
        .filter(|coord| get(grid, *coord) == Some(b'A'))
        // this iterates over all locations with an 'A'
        .filter(|coord| {
            for offsets in CORNER_OFFSETS {
                let mut corner_chars = offsets
                    .map(|offset| [coord[0] + offset[0], coord[1] + offset[1]])
                    .map(|coord| get(grid, coord).unwrap_or(0));

                corner_chars.sort();

                if corner_chars != PATTERN {
                    return false;
                }
            }

            true
        })
        .count()
}

fn get(grid: &Grid, coord: [isize; 2]) -> Option<u8> {
    grid.get(coord[0] as usize)
        .and_then(|row| row.get(coord[1] as usize))
        .copied()
}
