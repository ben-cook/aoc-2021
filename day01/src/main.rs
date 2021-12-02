use std::fs;
use std::slice::Windows;

fn main() {
    one();
    two();
}

fn one() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let nums = contents.lines();

    let mut previous = 9999;
    let mut count = 0;
    for num in nums {
        let num = num.parse::<i32>().unwrap();

        if num > previous {
            count += 1;
        }

        previous = num;
    }

    println!("{}", count);
}

fn two() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let num_vec: Vec<u32> = contents
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    let nums: &[u32] = &num_vec[..];

    let mut count = 0;

    let mut prev_win = 99999;

    for win in nums.windows(3) {
        let sum = win.to_vec().into_iter().sum();

        if sum > prev_win {
            count += 1
        }

        prev_win = sum;
    }

    println!("{}", count);
}
