use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

type Coord = [isize; 2];
type AntennaMap = HashMap<u8, Vec<Coord>>;

const BOUNDS: isize = 50;

fn main() {
    let antenna_map = parse_input(include_str!("../input"));

    println!("{}", count_antinodes(&antenna_map, 1..=1));
    println!("{}", count_antinodes(&antenna_map, 0..=50));
}

fn parse_input(input: &str) -> AntennaMap {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(move |(x, &byte)| ([x as isize, y as isize], byte))
        })
        .filter(|&(_, byte)| byte != b'.')
        .fold(HashMap::new(), |mut acc, (coord, byte)| {
            acc.entry(byte).or_default().push(coord);
            acc
        })
}

fn count_antinodes(antenna_map: &AntennaMap, range: RangeInclusive<isize>) -> usize {
    let mut antinodes = HashSet::new();

    for antennas in antenna_map.values() {
        for i in 0..antennas.len() - 1 {
            for j in i + 1..antennas.len() {
                let a = &antennas[i];
                let b = &antennas[j];

                for multiple in range.clone() {
                    antinodes.insert(antinode(a, b, multiple));
                    antinodes.insert(antinode(b, a, multiple));
                }
            }
        }
    }

    antinodes
        .iter()
        .filter(|coord| coord.iter().all(|val| (0..BOUNDS).contains(val)))
        .count()
}

fn antinode(from: &Coord, to: &Coord, multiple: isize) -> Coord {
    let vector = [to[0] - from[0], to[1] - from[1]];
    [to[0] + vector[0] * multiple, to[1] + vector[1] * multiple]
}
