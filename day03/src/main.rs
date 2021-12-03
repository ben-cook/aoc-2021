use std::fs;

fn main() {
    one();
    two();
}

fn bin_to_dec(n: &[i32]) -> i32 {
    n.to_vec()
        .iter()
        .enumerate()
        .map(|(idx, value)| value * 2i32.pow((11 - idx).try_into().unwrap()))
        .sum::<i32>()
}

fn one() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().collect();

    let mut most_common_digits: Vec<i32> = Vec::new();

    for i in 0..12 {
        let mut count = 0;
        for line in &lines {
            let digit = line
                .chars()
                .nth(i)
                .expect("line too short")
                .to_digit(10)
                .expect("couldn't convert to digit");
            count += digit;
        }

        if count > 500 {
            most_common_digits.push(1);
        } else {
            most_common_digits.push(0);
        }
    }

    let gamma = bin_to_dec(&most_common_digits);

    let least_common_digits: Vec<i32> = most_common_digits
        .iter()
        .map(|i| if *i == 1 { 0 } else { 1 })
        .collect();

    let epsilon = bin_to_dec(&least_common_digits);

    println!("{} * {} = {}", gamma, epsilon, gamma * epsilon);
}

fn numbers_with_digit_at_position<'a>(
    digit: i32,
    position: i32,
    lines: &Vec<&'a str>,
) -> Vec<&'a str> {
    return lines
        .clone()
        .into_iter()
        .filter(|line| {
            line.chars()
                .nth(position as usize)
                .unwrap()
                .to_digit(10)
                .unwrap()
                == digit as u32
        })
        .collect();
}

fn two() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut lines: Vec<&str> = contents.lines().collect();

    let mut oxygen = 0;
    let mut co2 = 0;

    let mut i = 0;

    while lines.len() > 1 {
        let mut count = 0;
        for line in &lines {
            let digit = line
                .chars()
                .nth(i)
                .expect("line too short")
                .to_digit(10)
                .expect("couldn't convert to digit");
            count += digit;
        }
        let mut most_common_digit = 0;

        if count as f32 >= ((lines.len() as f32) / 2f32) {
            // most common is 1
            most_common_digit = 1;
        }

        let filtered_nums = numbers_with_digit_at_position(most_common_digit, i as i32, &lines);

        if filtered_nums.len() == 1 {
            let a = *filtered_nums.get(0).unwrap();

            oxygen = a
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .enumerate()
                .map(|(idx, value)| value * 2i32.pow((11 - idx).try_into().unwrap()))
                .sum::<i32>();
        } else {
            i += 1;
        }
        lines = filtered_nums;
    }

    lines = contents.lines().collect();
    i = 0;

    while lines.len() > 1 {
        let mut count = 0;
        for line in &lines {
            let digit = line
                .chars()
                .nth(i)
                .expect("line too short")
                .to_digit(10)
                .expect("couldn't convert to digit");
            count += digit;
        }
        let mut least_common_digit = 1;

        if count as f32 >= ((lines.len() as f32) / 2f32) {
            // most common is 1
            least_common_digit = 0;
        }

        let filtered_nums = numbers_with_digit_at_position(least_common_digit, i as i32, &lines);

        if filtered_nums.len() == 1 {
            let a = *filtered_nums.get(0).unwrap();

            co2 = a
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .enumerate()
                .map(|(idx, value)| value * 2i32.pow((11 - idx).try_into().unwrap()))
                .sum::<i32>();
        } else {
            i += 1;
        }
        lines = filtered_nums;
    }

    println!("{} * {} = {}", oxygen, co2, oxygen * co2);
}
