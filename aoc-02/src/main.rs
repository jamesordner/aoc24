use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").unwrap();
    let reports = parse_input(&input);

    let part_one_result = reports.iter().filter(|report| is_safe(report)).count();

    println!("{part_one_result}");

    let part_two_result = reports
        .iter()
        .filter(|report| {
            (0..report.len()).any(|i| {
                let mut report = report.to_vec();
                report.remove(i);
                is_safe(&report)
            })
        })
        .count();

    println!("{part_two_result}");
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(report: &[u32]) -> bool {
    let [a, b] = if report[0] < report[1] {
        [0, 1] // ascending
    } else {
        [1, 0] // descending
    };

    report
        .windows(2)
        .all(|win| win[a] < win[b] && (win[b] - win[a]) <= 3)
}
