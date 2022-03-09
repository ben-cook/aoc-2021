use std::time::Instant;

use day14::*;

fn main() {
    let start = Instant::now();

    let input = include_str!("../input.txt");

    println!("[Part One]       {}", one(input));
    println!("[Part Two]       {}", two(input));

    let duration = start.elapsed().as_secs_f64();
    println!("[Execution Time] {:.4}s", duration);
}

fn one(input: &str) -> i64 {
    method_one(input, 10)
}

fn two(input: &str) -> i64 {
    method_two(input, 40)
}
