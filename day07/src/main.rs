use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = include_str!("../input.txt");

    one(input);
    two(input);

    let duration = start.elapsed().as_secs_f64();
    println!("Execution time {} seconds", duration);
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

fn triangle(n: i64) -> i64 {
    (1..(n + 1)).into_iter().sum()
}

fn two(input: &str) {
    let numbers: Vec<i64> = input
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();

    let min_fuel: i64 = (min..(max + 1))
        .into_iter()
        .map(|r| numbers.iter().map(|num| triangle((num - r).abs())).sum())
        .min()
        .unwrap();

    println!("part two: min fuel: {}", min_fuel);
}
