use std::{fs::read_to_string, iter::zip};

fn main() {
    let input = read_to_string("input").unwrap();
    let mut lists = parse_input(&input);

    lists[0].sort();
    lists[1].sort();

    let part_one_result = zip(&lists[0], &lists[1])
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u32>();

    println!("{part_one_result}");

    let part_two_result = lists[0]
        .iter()
        .map(|a| a * lists[1].iter().filter(|&b| a == b).count() as u32)
        .sum::<u32>();

    println!("{part_two_result}");
}

fn parse_input(input: &str) -> [Vec<u32>; 2] {
    let mut lists = [Vec::new(), Vec::new()];

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        lists[0].push(iter.next().unwrap().parse().unwrap());
        lists[1].push(iter.next().unwrap().parse().unwrap());
    }

    lists
}
