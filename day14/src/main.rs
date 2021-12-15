use std::{collections::HashMap, time::Instant};

use itertools::Itertools;

fn main() {
    let start = Instant::now();

    let input = include_str!("../example.txt");

    println!("[Part One]       {}", one(input));
    println!("[Part Two]       {}", two(input));

    let duration = start.elapsed().as_secs_f64();
    println!("[Execution Time] {:.4}s", duration);
}

fn one(input: &str) -> i64 {
    let mut lines = input.lines();
    let polymer_template = lines.next().unwrap();
    lines.next();

    let mut map = HashMap::new();

    lines.for_each(|line| {
        let (a, b) = line.split_once(" -> ").unwrap();
        map.insert(a, b);
    });

    const ITERATIONS: usize = 10;
    let mut answer = polymer_template.to_string();
    for _ in 0..ITERATIONS {
        answer = answer
            .chars()
            .tuple_windows()
            .map(|(c1, c2)| {
                let combined_str = c1.to_string() + &c2.to_string();

                match map.get(&combined_str as &str) {
                    Some(value) => c1.to_string() + &value.to_string(),
                    None => c1.to_string(),
                }
            })
            .collect::<String>()
            + &(answer.chars().last().unwrap().to_string());
    }

    let mut count = HashMap::new();

    for item in answer.chars() {
        *count.entry(item).or_insert(0) += 1;
    }

    let most_common = count.iter().sorted_by_key(|x| -x.1).next().unwrap();
    let least_common = count.iter().sorted_by_key(|x| x.1).next().unwrap();

    most_common.1 - least_common.1
}

fn two(input: &str) -> i64 {
    0
}
