use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn main() {
    let start = Instant::now();

    let input = include_str!("../input.txt");

    one(input);
    two(input);

    let duration = start.elapsed().as_secs_f64();
    println!("Execution time:  {}s", duration);
}

fn get_adjacent(arr: &[Vec<i64>], x: usize, y: usize) -> Vec<i64> {
    let up;
    if y > 0 {
        up = arr.get(y - 1).unwrap().get(x);
    } else {
        up = None
    }

    let down;
    if y < arr.len() - 1 {
        down = arr.get(y + 1).unwrap().get(x);
    } else {
        down = None
    }

    let left;
    if x > 0 {
        left = arr.get(y).unwrap().get(x - 1);
    } else {
        left = None
    }

    let right;
    if x < arr.get(0).unwrap().len() - 1 {
        right = arr.get(y).unwrap().get(x + 1);
    } else {
        right = None
    }

    let mut adjacent = vec![up, down, left, right];
    adjacent.retain(|a| a.is_some());

    return adjacent.iter().map(|o| o.unwrap()).copied().collect();
}

fn is_smallest(arr: &[Vec<i64>], x: usize, y: usize) -> bool {
    let point = arr[y].get(x).unwrap();

    let adjacent = get_adjacent(arr, x, y);
    adjacent.iter().filter(|a| *a > point).count() == adjacent.len()
}

fn one(input: &str) {
    let input: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input.get(y).unwrap().len() {
            let point = input.get(y).unwrap().get(x).unwrap();

            if is_smallest(&input, x, y) {
                count += 1 + point;
            }
        }
    }

    println!("[Part One] {}", count);
}

fn get_adjacent_coords(arr: &[Vec<i64>], x: usize, y: usize) -> Vec<(i64, usize, usize)> {
    let up;
    if y > 0 {
        up = (arr.get(y - 1).unwrap().get(x), x, y - 1)
    } else {
        up = (None, x, y)
    }

    let down;
    if y < arr.len() - 1 {
        down = (arr.get(y + 1).unwrap().get(x), x, y + 1)
    } else {
        down = (None, x, y)
    }

    let left;
    if x > 0 {
        left = (arr.get(y).unwrap().get(x - 1), x - 1, y)
    } else {
        left = (None, x, y)
    }

    let right;
    if x < arr.get(0).unwrap().len() - 1 {
        right = (arr.get(y).unwrap().get(x + 1), x + 1, y)
    } else {
        right = (None, x, y)
    }

    let mut adjacent = vec![up, down, left, right];
    adjacent.retain(|a| a.0.is_some());

    return adjacent.iter().map(|o| (*o.0.unwrap(), o.1, o.2)).collect();
}

fn flows_to(arr: &[Vec<i64>], x: usize, y: usize) -> (usize, usize) {
    let point = arr[y].get(x).unwrap();

    if *point == 9 {
        return (x, y);
    }

    // base case: it's the smallest
    if is_smallest(arr, x, y) {
        return (x, y);
    }

    // recursive case: it flows to the smallest adjacent tile
    let adjacent = get_adjacent_coords(arr, x, y);
    let smallest_adjacent = adjacent.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap();

    // println!("({}, {}, {}) -> {:?}", point, x, y, smallest_adjacent);
    flows_to(arr, smallest_adjacent.1, smallest_adjacent.2)
}

fn two(input: &str) {
    let input: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    // Map each point to the point where it flows to
    let mut flow_vec = Vec::new();

    for y in 0..input.len() {
        let mut row = Vec::new();
        for x in 0..input.get(y).unwrap().len() {
            row.push(flows_to(&input, x, y));
        }
        flow_vec.push(row);
    }

    let mut flows: HashMap<(usize, usize), i64> = HashMap::new();

    // Find the basins
    for flow_end in flow_vec.iter().flatten() {
        if !flows.keys().contains(flow_end) {
            let mut basin_size = 0;
            for flow_start in flow_vec.iter().flatten() {
                if flow_start == flow_end {
                    basin_size += 1;
                }
            }
            flows.insert(*flow_end, basin_size);
        }
    }

    let answer = flows.values().sorted().rev().take(3).product::<i64>();

    println!("[Part Two] {}", answer);
}
