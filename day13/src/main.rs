use itertools::Itertools;
use std::time::Instant;

use day13::*;

fn main() {
    let start = Instant::now();

    let input = include_str!("../input.txt");

    println!("[Part One]       {}", one(input));
    println!("[Part Two]");
    two(input);

    let duration = start.elapsed().as_secs_f64();
    println!("[Execution Time] {:.4}s", duration);
}

fn parse_folds(input: &str) -> Vec<(char, usize)> {
    input
        .lines()
        .filter(|line| line.contains("fold along"))
        .map(|line| {
            if let Some(c) = line.chars().nth(11) {
                let str_slice = &line[13..];
                let number = str_slice.parse::<usize>().unwrap();
                return (c, number)
            }
            ('x', 0)
        })
        .collect_vec()
}

fn one(input: &str) -> i64 {
    let mut array: Vec<Vec<bool>> = Vec::new();

    let mut max_x = 0;
    let mut max_y = 0;

    for line in input.lines() {
        if let Some((x, y)) = line.trim().split_once(',') {
            let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        }
    }

    for _ in 0..(max_y + 1) {
        array.push(vec![false; max_x + 1]);
    }

    input.lines().for_each(|line| {
        if let Some((x, y)) = line.trim().split_once(',') {
            array[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = true;
        }
    });

    let folds = parse_folds(input);
    let (first_char, first_n) = folds.into_iter().next().expect("No folds in input");

    let array_after_first_fold = execute_fold(array, first_char, first_n);

    count_dots(&array_after_first_fold)
}

fn two(input: &str) {
    let mut array: Vec<Vec<bool>> = Vec::new();

    let mut max_x = 0;
    let mut max_y = 0;

    for line in input.lines() {
        if let Some((x, y)) = line.trim().split_once(',') {
            let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        }
    }

    for _ in 0..(max_y + 1) {
        array.push(vec![false; max_x + 1]);
    }

    input.lines().for_each(|line| {
        if let Some((x, y)) = line.trim().split_once(',') {
            array[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = true;
        }
    });

    let folds = parse_folds(input);
    for (c, n) in folds {
        array = execute_fold(array, c, n);
    }

    show_board(&array);
}


