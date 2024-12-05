use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input").unwrap();
    let (ordering, mut updates) = parse_input(&input);

    let part_one = updates
        .iter()
        .filter(|update| is_ordered(update, &ordering))
        .map(|update| update[update.len() / 2] as u32)
        .sum::<u32>();

    println!("{part_one}");

    let part_two = updates
        .iter_mut()
        .filter(|update| !is_ordered(update, &ordering))
        .map(|update| {
            fix_update(update, &ordering);
            update[update.len() / 2] as u32
        })
        .sum::<u32>();

    println!("{part_two}");
}

fn parse_input(input: &str) -> (HashMap<u8, Vec<u8>>, Vec<Vec<u8>>) {
    let ordering = input.lines().take_while(|line| !line.is_empty()).fold(
        HashMap::<u8, Vec<u8>>::new(),
        |mut acc, line| {
            let key = line[0..2].parse::<u8>().unwrap();
            let val = line[3..5].parse::<u8>().unwrap();

            acc.entry(key).or_default().push(val);
            acc
        },
    );

    let updates = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(',')
                .map(|val| val.parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    (ordering, updates)
}

fn is_ordered(mut update: &[u8], ordering: &HashMap<u8, Vec<u8>>) -> bool {
    while !update.is_empty() {
        let end_index = update.len() - 1;
        let list = &ordering[&update[end_index]];
        update = &update[..end_index];

        if update.iter().any(|page| list.contains(page)) {
            return false;
        }
    }

    true
}

fn fix_update(mut update: &mut [u8], ordering: &HashMap<u8, Vec<u8>>) {
    while !update.is_empty() {
        let index = update
            .iter()
            .position(|page| {
                let list = &ordering[page];
                update
                    .iter()
                    .filter(|&p| p != page)
                    .all(|page| !list.contains(page))
            })
            .unwrap();

        let end_index = update.len() - 1;
        update.swap(index, end_index);
        update = &mut update[..end_index];
    }
}
