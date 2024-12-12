use std::{array::from_fn, collections::HashSet};

type Map = [[u8; BOUNDS]; BOUNDS];
type Coord = [isize; 2];

const BOUNDS: usize = 56;

fn main() {
    let trail_map = parse_input(include_str!("../input"));

    let part_one = sum_trailhead_scores(|trailhead| {
        let mut peaks = HashSet::new();
        find_trails_from_trailhead(&trail_map, trailhead, |peak| {
            peaks.insert(peak);
        });
        peaks.len()
    });

    println!("{part_one}");

    let part_two = sum_trailhead_scores(|trailhead| {
        let mut trails = 0;
        find_trails_from_trailhead(&trail_map, trailhead, |_| {
            trails += 1;
        });
        trails
    });

    println!("{part_two}");
}

fn parse_input(input: &str) -> Map {
    let mut lines = input.lines();
    from_fn(|_| {
        let line = lines.next().unwrap().as_bytes();
        from_fn(|i| line[i] - b'0')
    })
}

fn sum_trailhead_scores<F: FnMut(Coord) -> usize>(trailhead_score: F) -> usize {
    (0..BOUNDS)
        .flat_map(|x| (0..BOUNDS).map(move |y| [x as isize, y as isize]))
        .map(trailhead_score)
        .sum()
}

fn find_trails_from_trailhead<F: FnMut(Coord)>(trail_map: &Map, coord: Coord, mut completion: F) {
    (0..10).fold(vec![coord], |mut next_coords, next_height| {
        let len = next_coords.len();

        for i in 0..len {
            let coord = next_coords[i];

            if trail_map
                .get(coord[0] as usize)
                .and_then(|row| row.get(coord[1] as usize))
                .is_none_or(|&height| height != next_height)
            {
                continue;
            }

            if next_height < 9 {
                next_coords.extend([
                    [coord[0] + 1, coord[1]],
                    [coord[0] - 1, coord[1]],
                    [coord[0], coord[1] + 1],
                    [coord[0], coord[1] - 1],
                ]);
            } else {
                completion(coord);
            }
        }

        next_coords.drain(..len);
        next_coords
    });
}
