use std::fs::read_to_string;

use regex::RegexBuilder;

fn main() {
    let input = read_to_string("input").unwrap();

    println!("{}", part_one(&input));

    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> u32 {
    let re = RegexBuilder::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    re.captures_iter(input)
        .map(|val| val[1].parse::<u32>().unwrap() * val[2].parse::<u32>().unwrap())
        .sum()
}

fn part_two(input: &str) -> u32 {
    let re = RegexBuilder::new(r"(^|do\(\)).*?(don't\(\)|$)")
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    re.captures_iter(input).map(|val| part_one(&val[0])).sum()
}
