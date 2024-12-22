use std::{array::from_fn, cmp::Ordering, collections::BinaryHeap};

use rayon::prelude::*;

const BOUNDS: isize = 71;

type Coord = [isize; 2];
type StepMap = [[u16; BOUNDS as usize]; BOUNDS as usize];

#[derive(Default, PartialEq, Eq)]
struct Entry {
    coord: Coord,
    steps: u16,
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        // invert ordering to make `BinaryHeap` a min-heap
        other.steps.cmp(&self.steps)
    }
}

fn main() {
    let corrupted_bytes = parse_input(include_str!("../input"));

    // part one

    println!("{}", find_path_dijkstra(&corrupted_bytes[..1024]));

    // part two

    let (_, coords) = corrupted_bytes
        .par_iter()
        .enumerate()
        .find_first(|&(i, _)| find_path_dijkstra(&corrupted_bytes[..=i]) == u16::MAX)
        .unwrap();

    println!("{},{}", coords[0], coords[1]);
}

fn parse_input(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            [x.parse().unwrap(), y.parse().unwrap()]
        })
        .collect()
}

fn find_path_dijkstra(corrupted_bytes: &[Coord]) -> u16 {
    let mut queue = BinaryHeap::from([Entry::default()]);
    let mut min_steps: StepMap = from_fn(|_| from_fn(|_| u16::MAX));

    while let Some(entry) = queue.pop() {
        for offset in [[-1, 0], [1, 0], [0, -1], [0, 1]] {
            let neighbor = from_fn(|i| entry.coord[i] + offset[i]);

            let Some(min_steps) = min_steps
                .get_mut(neighbor[1] as usize)
                .and_then(|row| row.get_mut(neighbor[0] as usize))
            else {
                continue;
            };

            if corrupted_bytes.contains(&neighbor) {
                continue;
            }

            let steps = entry.steps + 1;

            if steps < *min_steps {
                *min_steps = steps;
                queue.push(Entry {
                    coord: neighbor,
                    steps,
                });
            }
        }
    }

    min_steps.last().unwrap().last().copied().unwrap()
}
