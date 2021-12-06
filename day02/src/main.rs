fn main() {
    let input = include_str!("../input.txt");

    one(input);
    two(input);
}

fn one(input: &str) {
    let (depth, horizontal) = input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let instruction = split.next().unwrap();
            let magnitude = split.next().unwrap().parse::<i32>().unwrap();

            (instruction, magnitude)
        })
        .fold(
            (0, 0),
            |(depth, horizontal), (instruction, magnitude)| match instruction {
                "up" => (depth - magnitude, horizontal),
                "down" => (depth + magnitude, horizontal),
                "forward" => (depth, horizontal + magnitude),
                _ => unreachable!(),
            },
        );

    println!(
        "part one: {} * {} = {}",
        depth,
        horizontal,
        horizontal * depth
    );
}

fn two(input: &str) {
    let (depth, horizontal, _aim) = input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let instruction = split.next().unwrap();
            let magnitude = split.next().unwrap().parse::<i32>().unwrap();

            (instruction, magnitude)
        })
        .fold(
            (0, 0, 0),
            |(depth, horizontal, aim), (instruction, magnitude)| match instruction {
                "up" => (depth, horizontal, aim - magnitude),
                "down" => (depth, horizontal, aim + magnitude),
                "forward" => (depth + aim * magnitude, horizontal + magnitude, aim),
                _ => unreachable!(),
            },
        );

    println!(
        "part two: {} * {} = {}",
        depth,
        horizontal,
        horizontal * depth
    );
}
