fn main() {
    let input = parse_input(include_str!("../input"));

    let mut buffers = [input, Vec::new()];

    for i in 0..25 {
        let (old_buffer, new_buffer) = buffers.split_at_mut(1);
        let old_buffer = &mut old_buffer[0];
        let new_buffer = &mut new_buffer[0];

        for stone in old_buffer.iter() {
            insert_new_stone(stone, new_buffer);
        }

        old_buffer.clear();
        buffers.rotate_left(1);
    }

    dbg!(buffers[0].len());
}

fn parse_input(input: &str) -> Vec<String> {
    input.split_whitespace().map(|val| val.to_owned()).collect()
}

fn insert_new_stone(stone: &str, stones: &mut Vec<String>) {
    if stone.len() == 1 && stone.starts_with('0') {
        stones.push("1".to_owned());
    } else if stone.len() % 2 == 0 {
        let half = stone.len() / 2;
        stones.push(stone[..half].to_owned());

        let right = stone[half..].trim_start_matches('0');
        if right.is_empty() {
            stones.push("0".to_owned());
        } else {
            stones.push(right.to_owned());
        }
    } else {
        stones.push((stone.parse::<u128>().unwrap() * 2024).to_string());
    }
}
