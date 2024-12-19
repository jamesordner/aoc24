use std::{
    array::from_fn,
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

type Maze = [[Tile; BOUNDS]; BOUNDS];
type Coord = [isize; 2];

const BOUNDS: usize = 141;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Reindeer {
    score: u64,
    coord: Coord,
    dir_offset: Coord,
    visited: Vec<Coord>,
}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.score.cmp(&self.score))
    }
}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

fn main() {
    let (maze, start, end) = parse_input(include_str!("../input"));

    let (score, tile_count) = find_paths(&maze, start, end);

    println!("{score}");
    println!("{tile_count}");
}

fn parse_input(input: &str) -> (Maze, Coord, Coord) {
    let mut lines = input.lines();

    let mut start = None;
    let mut end = None;

    let maze = from_fn(|y| {
        let line = lines.next().unwrap().as_bytes();
        from_fn(|x| match line[x] {
            b'#' => Tile::Wall,
            b'.' => Tile::Empty,
            b'S' => {
                start = Some([x as isize, y as isize]);
                Tile::Empty
            }
            b'E' => {
                end = Some([x as isize, y as isize]);
                Tile::Empty
            }
            _ => unreachable!(),
        })
    });

    (maze, start.unwrap(), end.unwrap())
}

/// Returns the lowest score, and the number of unique tiles on any shortest path
fn find_paths(maze: &Maze, start: Coord, end: Coord) -> (u64, usize) {
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::from([Reindeer {
        score: 0,
        coord: start,
        dir_offset: [1, 0],
        visited: vec![start],
    }]);

    let mut lowest_score = None;
    let mut winning_tiles = HashSet::<Coord>::new();

    loop {
        let reindeer = queue.pop().unwrap();

        // check end conditions

        if lowest_score.is_some_and(|score| score < reindeer.score) {
            break;
        }

        if reindeer.coord == end {
            lowest_score = Some(reindeer.score);
            winning_tiles.extend(&reindeer.visited);
        }

        // record that we were here

        visited.insert((reindeer.coord, reindeer.dir_offset));

        // insert all possible moves

        for (rotations, score_increase) in [(0, 1), (1, 1001), (3, 1001)] {
            let dir_offset = rotate(reindeer.dir_offset, rotations);
            let coord = add(reindeer.coord, dir_offset);

            if maze[coord[1] as usize][coord[0] as usize] == Tile::Wall
                || visited.contains(&(coord, dir_offset))
            {
                continue;
            }

            let mut visited = reindeer.visited.clone();
            visited.push(coord);

            queue.push(Reindeer {
                score: reindeer.score + score_increase,
                coord,
                dir_offset,
                visited,
            });
        }
    }

    (lowest_score.unwrap(), winning_tiles.len())
}

fn add(a: Coord, b: Coord) -> Coord {
    from_fn(|i| a[i] + b[i])
}

fn rotate(mut coord: Coord, iterations: usize) -> Coord {
    for _ in 0..iterations {
        coord = [-coord[1], coord[0]];
    }

    coord
}
