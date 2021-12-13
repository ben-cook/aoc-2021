use grid::Grid;
use itertools::Itertools;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = include_str!("../example2.txt");

    println!("[Part One]       {}", one(input));
    // println!("[Part Two]       {}", two(input));

    let duration = start.elapsed().as_secs_f64();
    println!("[Execution Time] {:.4}s", duration);
}

fn one(input: &str) -> i64 {
    let data = input.lines().fold(Vec::new(), |mut acc, nxt| {
        acc.push(
            nxt.chars()
                .map(|c| c.to_string().parse::<i64>().unwrap())
                .collect_vec(),
        );
        acc
    });

    let mut grid = Grid::from(data);

    const STEPS: i64 = 5;

    println!("Before any steps:");
    grid.show(0);

    for i in 1..(STEPS + 1) {
        // Add 1
        grid = Grid::from(
            grid.row_iter()
                .map(|row| row.iter().map(|v| v + 1).collect())
                .collect(),
        );

        let flashed_coords: Vec<(usize, usize)> = Vec::new();
        let mut did_flash = true;
        while did_flash {
            did_flash = false;
            for (y, row) in grid.data().iter().enumerate() {
                for (x, item) in row.iter().enumerate() {
                    if *item > 9 {}
                }
            }
        }

        println!("After step {}:", i);
        grid.show(0);
    }

    0
}

fn two(input: &str) -> i64 {
    todo!()
}
