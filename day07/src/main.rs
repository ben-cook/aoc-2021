use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt");

    one(input);
    let start = Instant::now();
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

    let mut map: HashMap<i64, i64> = HashMap::new();

    for i in min..(max + 1) {
        let total_fuel: i64 = numbers.iter().map(|number| (number - i).abs()).sum();

        map.insert(i, total_fuel);
    }

    let (target, fuel) = map.iter().min_by_key(|entry| entry.1).unwrap();

    println!("part one: target: {}, min fuel: {}", target, fuel);
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

    let mut map: HashMap<i64, i64> = HashMap::new();

    for i in min..(max + 1) {
        let total_fuel: i64 = numbers
            .iter()
            .map(|number| triangle((number - i).abs()))
            .sum();

        map.insert(i, total_fuel);
    }

    let (target, fuel) = map.iter().min_by_key(|entry| entry.1).unwrap();

    println!("part two: target: {}, min fuel: {}", target, fuel);
}
