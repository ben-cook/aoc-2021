use std::fs;

fn main() {
    // one();
    two();
}

#[derive(Debug, Clone)]
struct Board {
    array: Vec<Vec<(i32, bool)>>,
    won: bool,
}

impl Board {
    fn row(&self, i: usize) -> Vec<(i32, bool)> {
        self.array[i].clone()
    }

    fn col(&self, i: usize) -> Vec<(i32, bool)> {
        self.array.iter().map(|row| row[i]).collect()
    }

    fn call_number(&mut self, number: i32) {
        for row in &mut self.array {
            for (num, called) in row {
                if number == *num {
                    *called = true;
                }
            }
        }
    }

    fn won(&self) -> bool {
        for i in 0..5 {
            let row = self.row(i);
            if row.iter().all(|(_num, called)| *called) {
                return true;
            }

            let col = self.col(i);
            if col.iter().all(|(_num, called)| *called) {
                return true;
            }
        }
        return false;
    }

    fn sum_uncalled(&self) -> i32 {
        let mut sum = 0;

        for row in &self.array {
            for (num, called) in row {
                if !called {
                    sum += *num;
                }
            }
        }

        sum
    }
}

fn read_board(lines: &Vec<&str>, starting_index: i32) -> Board {
    let mut array = Vec::new();

    for i in 0..5 {
        let line = lines
            .get((starting_index + i) as usize)
            .expect("Couldn't parse board line");

        let line: Vec<(i32, bool)> = line
            .split(" ")
            .map(|num_str| num_str.trim())
            .filter_map(|num| num.parse::<i32>().ok())
            .map(|num| (num, false))
            .collect();

        array.push(line);
    }

    Board { array, won: false }
}

fn one() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().collect();

    let numbers_called: Vec<i32> = lines
        .get(0)
        .expect("Empty Input")
        .split(",")
        .map(|num| num.parse::<i32>().expect("Couldn't parse number called"))
        .collect();

    println!("numbers called = {:?}", numbers_called);

    println!("board 1: {:?}", read_board(&lines, 2));

    let mut boards: Vec<Board> = Vec::new();

    for i in 0..100 {
        boards.push(read_board(&lines, i * 6 + 2))
    }

    let mut i: usize = 0;
    let mut cont = true;
    loop {
        if !cont {
            break;
        }

        let number_called = *numbers_called.get(i).expect("Ran out of numbers called");

        println!("Calling number {}", number_called);
        for index in 0..boards.len() {
            let board: &mut Board = boards.get_mut(index).unwrap();
            board.call_number(number_called);

            // check if the board has won

            if board.won() {
                println!("the winning board is {:#?}", board);
                cont = false;

                // sum the uncalled numbers in the board

                let sum = board.sum_uncalled();
                println!("sum of uncalled is {}", sum);
                println!("{} * {} = {}", number_called, sum, number_called * sum);
                break;
            }
        }

        i += 1;

        // println!("boards: {:#?}", boards);
    }
}

fn two() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().collect();

    let numbers_called: Vec<i32> = lines
        .get(0)
        .expect("Empty Input")
        .split(",")
        .map(|num| num.parse::<i32>().expect("Couldn't parse number called"))
        .collect();

    let mut boards: Vec<Board> = Vec::new();

    for i in 0..100 {
        boards.push(read_board(&lines, i * 6 + 2))
    }

    let mut i: usize = 0;
    let mut cont = true;
    let mut num_won = 0;
    let num_boards = boards.len();
    // let num_boards = 3;

    // let mut last_num_called = 0;

    loop {
        if !cont {
            break;
        }

        let number_called_option = numbers_called.get(i);

        match number_called_option {
            Some(number_called) => {
                let number_called = *number_called;

                // last_num_called = number_called;

                println!("Calling number {}", number_called);
                for index in 0..boards.len() {
                    let board: &mut Board = boards.get_mut(index).unwrap();

                    // check if the board has won
                    if board.won {
                        continue;
                    }

                    board.call_number(number_called);

                    if board.won() {
                        num_won += 1;
                        println!("board {} won. total number won is {}", index, num_won);

                        board.won = true;

                        // last_won = Box::new(*board);

                        if num_won == num_boards {
                            println!("the last winning board is {:#?}", board);
                            cont = false;

                            // sum the uncalled numbers in the board

                            let sum = board.sum_uncalled();
                            println!("sum of uncalled is {}", sum);
                            println!("{} * {} = {}", number_called, sum, number_called * sum);

                            break;
                        }
                    }
                }
            }
            None => {
                // let sum = last_won.sum_uncalled();
                // println!("sum of uncalled is {}", sum);
                // println!("{} * {} = {}", last_num_called, sum, last_num_called * sum);
                break;
            }
        }

        i += 1;
    }
    // println!("boards: {:#?}", boards);
}
