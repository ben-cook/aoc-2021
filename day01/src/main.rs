use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    one(input);
    two(input);
}

fn one(input: &str) {
    println!(
        "part one: {}",
        input
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .tuple_windows()
            .filter(|(a, b)| a < b)
            .count()
    );
}

fn two(input: &str) {
    println!(
        "part two: {}",
        input
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .tuple_windows()
            .filter(|(a, _b, _c, d)| a < d)
            .count()
    );
}
