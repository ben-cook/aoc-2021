use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = include_str!("../input.txt");

    one(input);
    two(input);

    let duration = start.elapsed().as_secs_f64();
    println!("Execution time:  {}s", duration);
}

fn one(input: &str) {
    let numbers: Vec<i64> = input
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();

    let min_fuel: i64 = (min..(max + 1))
        .into_iter()
        .map(|r| numbers.iter().map(|num| (num - r).abs()).sum())
        .min()
        .unwrap();

    println!("part one: min fuel: {}", min_fuel);
}

fn two(input: &str) {
    let numbers: Vec<i64> = input
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();

    fn sum_of_natural_numbers(n: i64) -> i64 {
        n * (n - 1) / 2
    }

    let min_fuel: i64 = (min..(max + 1))
        .into_iter()
        .map(|r| {
            numbers
                .iter()
                .map(|num| sum_of_natural_numbers((num - r).abs()))
                .sum()
        })
        .min()
        .unwrap();

    println!("part two: min fuel: {}", min_fuel);
}
