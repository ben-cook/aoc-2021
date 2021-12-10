use itertools::Itertools;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = include_str!("../input.txt");

    one(input);
    two(input);

    let duration = start.elapsed().as_secs_f64();
    println!("Execution time:  {}s", duration);
}

fn score_char_p1(c: char) -> Option<i64> {
    match c {
        ')' => Some(3),
        ']' => Some(57),
        '}' => Some(1197),
        '>' => Some(25137),
        _ => None,
    }
}

fn is_opener(c: char) -> bool {
    "([{<".chars().contains(&c)
}

fn get_matching_closer(c: char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn one(input: &str) {
    let answer: i64 = input
        .lines()
        .filter_map(|line| {
            let mut stack: Vec<char> = Vec::new();

            for line_char in line.chars() {
                if is_opener(line_char) {
                    stack.push(line_char);
                } else {
                    // close bracket
                    if stack.is_empty() {
                        return Some(score_char_p1(line_char).unwrap());
                    }

                    if line_char != get_matching_closer(*stack.last().unwrap()).unwrap() {
                        return Some(score_char_p1(line_char).unwrap());
                    } else {
                        stack.pop();
                    }
                }
            }

            None
        })
        .sum();

    println!("[Part One] {}", answer);
}

fn score_char_p2(c: char) -> Option<i64> {
    match c {
        ')' => Some(1),
        ']' => Some(2),
        '}' => Some(3),
        '>' => Some(4),
        _ => None,
    }
}

fn two(input: &str) {
    let scores: Vec<i64> = input
        .lines()
        .filter(|line| {
            let mut stack: Vec<char> = Vec::new();
            let mut corrupt = false;

            for line_char in line.chars() {
                if is_opener(line_char) {
                    stack.push(line_char);
                } else {
                    // close bracket
                    if stack.is_empty() {
                        corrupt = true;
                    }

                    if line_char != get_matching_closer(*stack.last().unwrap()).unwrap() {
                        corrupt = true;
                    } else {
                        stack.pop();
                    }
                }
            }

            !corrupt
        })
        .map(|incomplete_line| {
            // map to score
            let mut stack: Vec<char> = Vec::new();

            for line_char in incomplete_line.chars() {
                if is_opener(line_char) {
                    stack.push(line_char);
                } else {
                    // close bracket
                    if stack.is_empty() {
                        unreachable!()
                    }

                    if line_char == get_matching_closer(*stack.last().unwrap()).unwrap() {
                        stack.pop();
                    } else {
                        unreachable!();
                    }
                }
            }

            stack
                .iter()
                .rev()
                .map(|c| get_matching_closer(*c).unwrap())
                .fold(0, |mut acc, next| {
                    acc *= 5;
                    acc += score_char_p2(next).unwrap();
                    acc
                })
        })
        .sorted()
        .collect();

    let middle = scores.get(scores.len() / 2).unwrap();

    println!("[Part Two] {}", *middle);
}
