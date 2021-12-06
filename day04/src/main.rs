fn main() {
    let input = include_str!("../input.txt");

    one(input);
    two(input);
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
        false
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

fn read_board(lines: &[&str], starting_index: i32) -> Board {
    let mut array = Vec::new();

    for i in 0..5 {
        let line = lines
            .get((starting_index + i) as usize)
            .expect("Couldn't parse board line");

        let line: Vec<(i32, bool)> = line
            .split(' ')
            .map(|num_str| num_str.trim())
            .filter_map(|num| num.parse::<i32>().ok())
            .map(|num| (num, false))
            .collect();

        array.push(line);
    }

    Board { array, won: false }
}

fn one(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    let numbers_called: Vec<i32> = lines
        .get(0)
        .expect("Empty Input")
        .split(',')
        .map(|num| num.parse::<i32>().expect("Couldn't parse number called"))
        .collect();

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

        for index in 0..boards.len() {
            let board: &mut Board = boards.get_mut(index).unwrap();
            board.call_number(number_called);

            // check if the board has won
            if board.won() {
                cont = false;

                // sum the uncalled numbers in the board
                let sum = board.sum_uncalled();
                println!(
                    "part one: {} * {} = {}",
                    number_called,
                    sum,
                    number_called * sum
                );
                break;
            }
        }

        i += 1;
    }
}

fn two(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    let numbers_called: Vec<i32> = lines
        .get(0)
        .expect("Empty Input")
        .split(',')
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

    loop {
        if !cont {
            break;
        }

        let number_called_option = numbers_called.get(i);

        match number_called_option {
            Some(number_called) => {
                let number_called = *number_called;

                for index in 0..boards.len() {
                    let board: &mut Board = boards.get_mut(index).unwrap();

                    // check if the board has won
                    if board.won {
                        continue;
                    }

                    board.call_number(number_called);

                    if board.won() {
                        num_won += 1;

                        board.won = true;

                        if num_won == num_boards {
                            cont = false;

                            // sum the uncalled numbers in the board
                            let sum = board.sum_uncalled();
                            println!(
                                "part two: {} * {} = {}",
                                number_called,
                                sum,
                                number_called * sum
                            );

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
