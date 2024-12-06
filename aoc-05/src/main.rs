use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input").unwrap();
    let (ordering, mut page_updates) = parse_input(&input);

    let part_one = page_updates
        .iter()
        .filter(|pages| is_ordered(pages, &ordering))
        .map(|pages| pages[pages.len() / 2] as u32)
        .sum::<u32>();

    println!("{part_one}");

    let part_two = page_updates
        .iter_mut()
        .filter(|pages| !is_ordered(pages, &ordering))
        .map(|pages| {
            fix_page_ordering(pages, &ordering);
            pages[pages.len() / 2] as u32
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

    let page_updates = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(',')
                .map(|val| val.parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    (ordering, page_updates)
}

fn is_ordered(mut pages: &[u8], ordering: &HashMap<u8, Vec<u8>>) -> bool {
    while !pages.is_empty() {
        let end_index = pages.len() - 1;
        let list = &ordering[&pages[end_index]];
        pages = &pages[..end_index];

        if pages.iter().any(|page| list.contains(page)) {
            return false;
        }
    }

    true
}

fn fix_page_ordering(mut pages: &mut [u8], ordering: &HashMap<u8, Vec<u8>>) {
    while !pages.is_empty() {
        let index = pages
            .iter()
            .position(|page| {
                let list = &ordering[page];
                pages
                    .iter()
                    .filter(|&p| p != page)
                    .all(|page| !list.contains(page))
            })
            .unwrap();

        let end_index = pages.len() - 1;
        pages.swap(index, end_index);
        pages = &mut pages[..end_index];
    }
}
