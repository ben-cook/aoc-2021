use std::fs;

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

    let nums = contents.lines();

    let mut count = 0;

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    for (index, num) in nums.enumerate() {
        let num = num.parse::<i32>().unwrap();

        if index > 2 {
            if num > a {
                count += 1;
            }

            a = b;
            b = c;
            c = num;
        } else if index == 0 {
            a = num;
        } else if index == 1 {
            b = num;
        } else if index == 2 {
            c = num;
        }
    }

    println!("{}", count);
}
