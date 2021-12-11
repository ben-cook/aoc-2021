use itertools::Itertools;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = include_str!("../input.txt");

    println!("[Part One]       {}", one(input));
    println!("[Part Two]       {}", two(input));

    let duration = start.elapsed().as_secs_f64();
    println!("[Execution Time] {:.4}s", duration);
}

fn one(input: &str) -> i64 {
    todo!()
}

fn two(input: &str) -> i64 {
    todo!()
}
