#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Block {
    Free,
    Id(u16),
}

impl Block {
    fn as_id(&self) -> u16 {
        let Self::Id(id) = self else { panic!() };
        *id
    }
}

fn main() {
    let blocks = parse_input(include_str!("../input"));

    println!("{}", part_one(blocks.clone()));
    println!("{}", part_two(blocks));
}

fn parse_input(input: &str) -> Vec<Block> {
    input
        .lines()
        .next()
        .unwrap()
        .as_bytes()
        .chunks(2)
        .enumerate()
        .fold(Vec::new(), |mut acc, (id, chunk)| {
            let id = u16::try_from(id).unwrap();
            let size = chunk[0] - b'0';
            let free = chunk.get(1).map(|free| free - b'0').unwrap_or(0);
            acc.extend((0..size).map(|_| Block::Id(id)));
            acc.extend((0..free).map(|_| Block::Free));
            acc
        })
}

fn part_one(mut blocks: Vec<Block>) -> u64 {
    let mut i = 0;

    loop {
        while blocks.get(i).is_some_and(|block| *block != Block::Free) {
            i += 1;
        }

        if i == blocks.len() {
            break;
        }

        blocks.swap_remove(i);
    }

    blocks
        .iter()
        .enumerate()
        .map(|(i, id)| i as u64 * id.as_id() as u64)
        .sum()
}

fn part_two(mut blocks: Vec<Block>) -> u64 {
    let mut i_head = blocks.len() - 1;
    let mut current_id = blocks[i_head].as_id();

    while current_id > 0 {
        // find end of the next file
        while !matches!(blocks[i_head], Block::Id(id) if id == current_id) {
            i_head -= 1;
        }
        let span_end = i_head + 1;

        // find start of the file
        while matches!(blocks[i_head], Block::Id(id) if id == current_id) {
            i_head -= 1;
        }
        let span_start = i_head + 1;
        let file_size = span_end - span_start;

        // search for a spot for the span
        let mut i = 0;
        'a: while i < span_start {
            // find start of next free block
            while blocks[i] != Block::Free {
                i += 1;

                if i == span_start {
                    break 'a;
                }
            }

            // free block found, find end of free block
            let free_start = i;
            while blocks[i] == Block::Free {
                i += 1;
            }
            let free_end = i;

            // check if we can move into this block
            if free_end - free_start >= file_size {
                let (free_block, file_block) = blocks.split_at_mut(span_start);
                let free_block = &mut free_block[free_start..free_start + file_size];
                let file_block = &mut file_block[..file_size];
                free_block.swap_with_slice(file_block);
                break;
            }
        }

        current_id -= 1;
    }

    blocks
        .iter()
        .enumerate()
        .map(|(i, &block)| match block {
            Block::Id(id) => i as u64 * id as u64,
            Block::Free => 0,
        })
        .sum()
}
