use std::fs;

fn main() {
    one();
    two();
}

fn one() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines = contents.lines();

    let mut horizontal = 0;
    let mut depth = 0;

    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();

        let dir: &str = split.get(0).unwrap();
        let amount: i32 = split.get(1).unwrap().parse::<i32>().unwrap();

        match dir {
            "up" => depth -= amount,
            "down" => depth += amount,
            "forward" => horizontal += amount,
            _ => panic!("bad direction"),
        }
    }

    println!("{} * {} = {}", horizontal, depth, horizontal * depth);
}

fn two() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines = contents.lines();

    let mut horizontal = 0;
    let mut aim = 0;
    let mut depth = 0;

    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();

        let dir: &str = split.get(0).unwrap();
        let amount: i32 = split.get(1).unwrap().parse::<i32>().unwrap();

        match dir {
            "up" => aim -= amount,
            "down" => aim += amount,
            "forward" => {
                horizontal += amount;
                depth += aim * amount
            }
            _ => panic!("bad direction"),
        }
    }

    println!("{} * {} = {}", horizontal, depth, horizontal * depth);
}
