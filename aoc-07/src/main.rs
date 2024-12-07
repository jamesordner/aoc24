fn main() {
    let equations = parse_input(include_str!("../input"));

    let part_one = sum_valid_equations(&equations, |a, b| [a + b, a * b]);
    let part_two = sum_valid_equations(&equations, |a, b| [a + b, a * b, concat_numbers(a, b)]);

    println!("{part_one}");
    println!("{part_two}");
}

struct Equation {
    sum: u64,
    values: Vec<u64>,
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (sum, values) = line.split_once(':').unwrap();

            let sum = sum.parse().unwrap();
            let values = values
                .split_whitespace()
                .map(|val| val.parse().unwrap())
                .collect();

            Equation { sum, values }
        })
        .collect()
}

fn sum_valid_equations<F, const LEN: usize>(equations: &[Equation], apply_ops: F) -> u64
where
    F: Fn(u64, u64) -> [u64; LEN] + Sync,
{
    equations
        .iter()
        .filter_map(|equation| {
            // insert initial value into results accumulator
            let mut acc = vec![equation.values[0]];

            for &value in &equation.values[1..] {
                let acc_len = acc.len();

                for i_acc in 0..acc_len {
                    acc.extend(apply_ops(acc[i_acc], value));
                }

                // remove old accumulated values
                acc.drain(..acc_len);
            }

            acc.into_iter().find(|&val| val == equation.sum)
        })
        .sum()
}

fn concat_numbers(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}
