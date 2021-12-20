use itertools::Itertools;
use std::{time::Instant, usize};

fn main() {
    let start = Instant::now();

    let input = include_str!("../input.txt");

    println!("[Part One]       {}", one(input));
    println!("[Part Two]       {}", two(input));

    let duration = start.elapsed().as_secs_f64();
    println!("[Execution Time] {:.4}s", duration);
}

fn show_grid(grid: &[Vec<i64>]) {
    grid.iter().for_each(|row| {
        row.iter().for_each(|item| print!("{:<1$}", *item, 1));
        println!();
    })
}

fn step(grid: &mut [Vec<i64>]) -> i64 {
    let adjacent_offset = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut result_flashes = 0;

    // Increment all
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            grid[y][x] += 1;
        }
    }

    // Flash
    loop {
        let mut flashes = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] > 9 {
                    // Flash This One
                    for (x1, y1) in adjacent_offset {
                        if let Ok(y_idx) = TryInto::<usize>::try_into(y as i32 + y1 as i32) {
                            if let Ok(x_idx) = TryInto::<usize>::try_into(x as i32 + x1 as i32) {
                                if let Some(row) = grid.get_mut(y_idx) {
                                    if let Some(elem) = row.get_mut(x_idx) {
                                        *elem += 1;
                                    }
                                }
                            }
                        }
                    }
                    grid[y][x] = i64::MIN;
                    flashes += 1;
                }
            }
        }
        result_flashes += flashes;
        if flashes == 0 {
            break;
        }
    }

    // Fix negative values
    for y in 0..grid.len() {
        for x in 0..grid.get(y).unwrap().len() {
            if grid[y][x] < 0 {
                grid[y][x] = 0;
            }
        }
    }

    result_flashes
}

fn one(input: &str) -> i64 {
    let mut grid = input.lines().fold(Vec::new(), |mut acc, nxt| {
        acc.push(
            nxt.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect_vec(),
        );
        acc
    });

    const STEPS: u32 = 100;

    println!("Before any steps:");
    show_grid(&grid);
    let mut flashes = 0;

    for i in 1..(STEPS + 1) {
        flashes += step(&mut grid);

        println!("After step {}:", i);
        show_grid(&grid);
    }

    flashes
}

fn two(input: &str) -> i64 {
    let mut grid = input.lines().fold(Vec::new(), |mut acc, nxt| {
        acc.push(
            nxt.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect_vec(),
        );
        acc
    });

    const STEPS: u32 = 100;

    println!("Before any steps:");
    show_grid(&grid);
    let mut i = 0;

    loop {
        let flashes = step(&mut grid);

        if flashes as usize == grid.len() * grid[0].len() {
            return i + 1;
        }

        println!("After step {}:", i);
        show_grid(&grid);
        i += 1;
    }
}
