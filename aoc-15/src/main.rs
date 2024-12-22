use std::array::from_fn;

const BOUNDS: usize = 48;

type Map = [[Tile; BOUNDS]; BOUNDS];
type Coord = [isize; 2];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Box,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn main() {
    let (map, directions, robot) = parse_input(include_str!("../input"));

    println!("{}", part_one(robot, map, &directions));
    println!("{}", part_two(robot, &map, &directions));
}

fn parse_input(input: &str) -> (Map, Vec<Direction>, Coord) {
    let mut lines = input.lines();
    lines.next();

    let mut robot = None;

    let map = from_fn(|y| {
        let line = lines.next().unwrap().as_bytes();
        from_fn(|x| match line[x + 1] {
            b'#' => Tile::Wall,
            b'.' => Tile::Empty,
            b'O' => Tile::Box,
            b'@' => {
                robot = Some([x as isize, y as isize]);
                Tile::Empty
            }
            _ => unreachable!(),
        })
    });

    lines.next();
    lines.next();

    let directions = lines
        .flat_map(|line| line.as_bytes())
        .map(|byte| match byte {
            b'^' => Direction::Up,
            b'>' => Direction::Right,
            b'v' => Direction::Down,
            b'<' => Direction::Left,
            _ => unreachable!(),
        })
        .collect();

    (map, directions, robot.unwrap())
}

fn part_one(mut robot: Coord, mut map: Map, directions: &[Direction]) -> usize {
    for direction in directions {
        let offset = rotate([0, -1], *direction);

        let Some(dest) = can_move(robot, offset, &map) else {
            continue;
        };

        robot = add(robot, offset);

        // swap moved crate to end
        if get(robot, &map).is_some_and(|&tile| tile == Tile::Box) {
            *get_mut(robot, &mut map).unwrap() = Tile::Empty;
            *get_mut(dest, &mut map).unwrap() = Tile::Box;
        }
    }

    map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, tile)| (x, y, tile)))
        .filter(|(_, _, &tile)| tile == Tile::Box)
        .map(|(x, y, _)| 100 * (y + 1) + (x + 1))
        .sum()
}

/// Returns the first free space if the boxes can be moved starting from `coord`.
fn can_move(mut coord: Coord, offset: Coord, map: &Map) -> Option<Coord> {
    loop {
        coord = add(coord, offset);

        match get(coord, map) {
            Some(Tile::Empty) => {
                return Some(coord);
            }
            Some(Tile::Box) => {
                continue;
            }
            _ => {
                return None;
            }
        }
    }
}

fn rotate(mut coord: Coord, direction: Direction) -> Coord {
    for _ in 0..direction as u8 {
        coord = [-coord[1], coord[0]];
    }

    coord
}

fn add(a: Coord, b: Coord) -> Coord {
    from_fn(|i| a[i] + b[i])
}

fn get(coord: Coord, map: &Map) -> Option<&Tile> {
    map.get(coord[1] as usize)
        .and_then(|row| row.get(coord[0] as usize))
}

fn get_mut(coord: Coord, map: &mut Map) -> Option<&mut Tile> {
    map.get_mut(coord[1] as usize)
        .and_then(|row| row.get_mut(coord[0] as usize))
}

fn part_two(mut robot: Coord, map: &Map, directions: &[Direction]) -> usize {
    // tranlate robot coords to wide map
    robot = [robot[0] * 2, robot[1]];

    // this time, just get a vec of all the boxes (don't modify Map)
    let mut boxes: Vec<_> = map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, tile)| (x, y, tile)))
        .filter(|(_, _, &tile)| tile == Tile::Box)
        .map(|(x, y, _)| [x as isize * 2, y as isize])
        .collect();

    for direction in directions {
        let offset = rotate([0, -1], *direction);
        let coord = add(robot, offset);

        if is_wall_wide(coord, map) {
            continue;
        }

        let Some(indices) = attempt_move_part_two(coord, offset, map, &boxes) else {
            continue;
        };

        for i in indices {
            boxes[i] = add(boxes[i], offset);
        }

        robot = coord;
    }

    boxes
        .iter()
        .map(|&[x, y]| 100 * (y as usize + 1) + (x as usize + 2))
        .sum()
}

/// Returns a vec of box indices, if they are not blocked
fn attempt_move_part_two(
    coord: Coord,
    offset: Coord,
    map: &Map,
    boxes: &[Coord],
) -> Option<Vec<usize>> {
    let mut indices = Vec::new();

    if attempt_move_part_two_recursive(coord, offset, map, boxes, &mut indices) {
        Some(indices)
    } else {
        None
    }
}

fn attempt_move_part_two_recursive(
    coord: Coord,
    offset: Coord,
    map: &Map,
    boxes: &[Coord],
    indices: &mut Vec<usize>,
) -> bool {
    if is_wall_wide(coord, map) {
        return false;
    }

    // check if we're pushing another box, and if so recursively check what *it* pushes
    if let Some((i, box_coords)) = boxes
        .iter()
        .enumerate()
        .map(|(i, &other_box)| (i, box_coords(other_box)))
        // inefficient but easy way to check that we're checking *outside* the box
        // this catches movements on the x axis, where one coord is still inside the box
        .filter(|(i, _)| !indices.contains(i))
        .find(|(_, box_coords)| box_coords.contains(&coord))
    {
        indices.push(i);

        for coord in box_coords {
            let coord = add(coord, offset);

            if !attempt_move_part_two_recursive(coord, offset, map, boxes, indices) {
                return false;
            }
        }
    }

    true
}

fn is_wall_wide(coord: Coord, map: &Map) -> bool {
    get([coord[0] / 2, coord[1]], map).is_none_or(|&tile| tile == Tile::Wall)
}

fn box_coords(coord: Coord) -> [Coord; 2] {
    [coord, [coord[0] + 1, coord[1]]]
}
