use std::collections::HashMap;

type IntType = u64;

fn main() {
    let input = parse_input(include_str!("../input"));

    // map<stone value, map<remaining iteration count, stone count at iteration>>
    let mut cache = HashMap::new();

    println!("{}", count_stones(&input, 25, &mut cache));
    println!("{}", count_stones(&input, 75, &mut cache));
}

fn parse_input(input: &str) -> Vec<IntType> {
    input
        .split_whitespace()
        .map(|val| val.parse().unwrap())
        .collect()
}

fn count_stones(
    starting_values: &[IntType],
    iterations: usize,
    cache: &mut HashMap<IntType, HashMap<usize, u64>>,
) -> u64 {
    let mut sum = 0;

    for &value in starting_values {
        recurse_next(value, iterations, cache, &mut sum);
    }

    sum
}

fn recurse_next(
    value: IntType,
    iterations: usize,
    cache: &mut HashMap<IntType, HashMap<usize, u64>>,
    sum: &mut u64,
) {
    // check base case (no more iterations)
    let Some(next_iteration) = iterations.checked_sub(1) else {
        *sum += 1;
        return;
    };

    // check cache
    if let Some(count) = cache
        .get(&value)
        .and_then(|chain| chain.get(&next_iteration))
        .copied()
    {
        *sum += count;
        return;
    }

    // calculate the final stone count for the remaining iterations

    let mut local_sum = 0;

    if value == 0 {
        recurse_next(1, next_iteration, cache, &mut local_sum);
    } else {
        let log = value.ilog10();
        if log % 2 == 0 {
            // odd number of digits
            recurse_next(value * 2024, next_iteration, cache, &mut local_sum);
        } else {
            // even number of digits
            let pow = 10u64.pow((log + 1) / 2);
            let top_digits = value / pow;
            let bottom_digits = value - top_digits * pow;
            recurse_next(top_digits, next_iteration, cache, &mut local_sum);
            recurse_next(bottom_digits, next_iteration, cache, &mut local_sum);
        }
    }

    cache
        .entry(value)
        .or_default()
        .insert(next_iteration, local_sum);

    *sum += local_sum;
}
