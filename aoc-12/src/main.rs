use std::{array::from_fn, collections::HashSet};

type Map = [[u8; BOUNDS]; BOUNDS];
type Coord = [isize; 2];

const BOUNDS: usize = 140;

fn main() {
    let map = parse_input(include_str!("../input"));
    let regions = regions(&map);

    println!("{}", part_one(&regions));
    println!("{}", part_two(&regions));
}

fn parse_input(input: &str) -> Map {
    let mut lines = input.lines();
    from_fn(|_| {
        let line = lines.next().unwrap().as_bytes();
        from_fn(|i| line[i])
    })
}

fn regions(map: &Map) -> Vec<HashSet<Coord>> {
    (0..BOUNDS)
        .flat_map(|y| (0..BOUNDS).map(move |x| [x as isize, y as isize]))
        .fold(Vec::new(), |mut regions, coord| {
            if regions.iter().all(|region| !region.contains(&coord)) {
                let mut region = HashSet::new();
                recurse_expand_region(coord, get(coord, map).unwrap(), &mut region, map);
                regions.push(region);
            }

            regions
        })
}

fn recurse_expand_region(coord: Coord, region_type: u8, region: &mut HashSet<Coord>, map: &Map) {
    if region.contains(&coord) || get(coord, map).is_none_or(|val| val != region_type) {
        return;
    }

    region.insert(coord);

    for coord in adjacent(coord) {
        recurse_expand_region(coord, region_type, region, map);
    }
}

fn get(coord: Coord, map: &Map) -> Option<u8> {
    map.get(coord[1] as usize)
        .and_then(|row| row.get(coord[0] as usize))
        .copied()
}

fn adjacent(coord: Coord) -> [Coord; 4] {
    [
        [coord[0] + 1, coord[1]],
        [coord[0] - 1, coord[1]],
        [coord[0], coord[1] + 1],
        [coord[0], coord[1] - 1],
    ]
}

fn part_one(regions: &[HashSet<Coord>]) -> usize {
    regions
        .iter()
        .map(|region| {
            let fences = region
                .iter()
                .map(|&coord| {
                    adjacent(coord)
                        .iter()
                        .filter(|&coord| !region.contains(coord))
                        .count()
                })
                .sum::<usize>();

            fences * region.len()
        })
        .sum()
}

fn part_two(regions: &[HashSet<Coord>]) -> usize {
    regions
        .iter()
        .map(|region| {
            let mut edges = 0;

            // track checked edges, so that we don't miss inner areas
            let mut checked_right_edges = HashSet::new();

            // start following fence on a right edge
            while let Some(&starting_coord) = region.iter().find(|&coord| {
                is_right_edge(coord, region)
                    && is_bottom_of_right_edge(coord, region)
                    && !checked_right_edges.contains(coord)
            }) {
                let mut orientation = 0;
                let mut coord = starting_coord;

                loop {
                    let up = rotate_cw([0, -1], orientation);
                    let right = rotate_cw([1, 0], orientation);

                    if orientation.rem_euclid(4) == 0 {
                        checked_right_edges.insert(coord);
                    }

                    let coord_up = from_fn(|i| coord[i] + up[i]);
                    let coord_up_right = from_fn(|i| coord_up[i] + right[i]);

                    let a = region.contains(&coord_up);
                    let b = region.contains(&coord_up_right);

                    if a && b {
                        // turn right
                        orientation += 1;
                        coord = coord_up;
                        edges += 1;
                    } else if !a {
                        // turn left
                        orientation -= 1;
                        edges += 1;
                    } else {
                        // continue on
                        coord = coord_up;
                    }

                    if orientation.rem_euclid(4) == 0 && coord == starting_coord {
                        break;
                    }
                }
            }

            edges * region.len()
        })
        .sum()
}

fn is_right_edge(coord: &Coord, region: &HashSet<Coord>) -> bool {
    !region.contains(&[coord[0] + 1, coord[1]])
}

fn is_bottom_of_right_edge(coord: &Coord, region: &HashSet<Coord>) -> bool {
    let coord_down = [coord[0], coord[1] + 1];
    let coord_down_right = [coord[0] + 1, coord[1] + 1];

    !region.contains(&coord_down)
        || region.contains(&coord_down) && region.contains(&coord_down_right)
}

fn rotate_cw(mut coord: Coord, rotations: isize) -> Coord {
    for _ in 0..rotations.rem_euclid(4) {
        coord = [-coord[1], coord[0]];
    }

    coord
}
