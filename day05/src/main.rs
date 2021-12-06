fn main() {
    let input = include_str!("../input.txt");

    one(input);
    two(input);
}

struct Board {
    array: Vec<Vec<i32>>,
}

impl Board {
    fn _show(&self) {
        for row in &self.array {
            for num in row {
                if *num == 0 {
                    print!(".");
                } else {
                    print!("{}", *num);
                }
            }
            println!();
        }
    }

    fn add_coord(&mut self, x: usize, y: usize) {
        let reference: &mut i32 = self
            .array
            .get_mut(y)
            .expect("y out of range")
            .get_mut(x)
            .expect("x out of range");

        *reference += 1
    }

    fn count_overlaps(&self) -> i32 {
        let mut count = 0;
        for row in &self.array {
            for num in row {
                if *num > 1 {
                    count += 1
                }
            }
        }
        count
    }
}

fn one(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    // Initialise Board
    let max_number: usize = lines
        .iter()
        .flat_map(|line| {
            line.split(" -> ")
                .flat_map(|pair| pair.split(',').map(|n| n.parse::<usize>().unwrap()))
        })
        .reduce(|accum, item| if accum >= item { accum } else { item })
        .unwrap()
        + 1;

    let mut array: Vec<Vec<i32>> = Vec::new();
    for _ in 0..max_number {
        array.push(vec![0; max_number]);
    }

    let mut board: Board = Board { array };

    // Filter to straight lines
    let filtered_coords: Vec<Vec<(i32, i32)>> = lines
        .iter()
        .map(|line| {
            return line
                .split(" -> ")
                .map(|pair| {
                    let parsed: Vec<i32> =
                        pair.split(',').map(|n| n.parse::<i32>().unwrap()).collect();
                    (parsed[0], parsed[1])
                })
                .collect::<Vec<(i32, i32)>>();
        })
        .filter(|coord_pair| {
            let (x1, y1) = *coord_pair.get(0).unwrap();
            let (x2, y2) = *coord_pair.get(1).unwrap();

            x1 == x2 || y1 == y2
        })
        .collect();

    for coord_pair in filtered_coords {
        let (mut x1, mut y1) = *coord_pair.get(0).unwrap();
        let (x2, y2) = *coord_pair.get(1).unwrap();

        let direction = ((x2 - x1).signum(), (y2 - y1).signum());

        // Add the lines
        while (x1, y1) != (x2, y2) {
            board.add_coord(x1 as usize, y1 as usize);
            x1 += direction.0;
            y1 += direction.1;
        }
        board.add_coord(x1 as usize, y1 as usize);
    }

    println!("part one: {} overlaps", board.count_overlaps());
}

fn two(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    // Initialise Board
    let max_number: usize = lines
        .iter()
        .flat_map(|line| {
            line.split(" -> ")
                .flat_map(|pair| pair.split(',').map(|n| n.parse::<usize>().unwrap()))
        })
        .reduce(|accum, item| if accum >= item { accum } else { item })
        .unwrap()
        + 1;

    let mut array: Vec<Vec<i32>> = Vec::new();
    for _ in 0..max_number {
        array.push(vec![0; max_number]);
    }

    let mut board: Board = Board { array };

    // Read in the lines
    let coords: Vec<Vec<(i32, i32)>> = lines
        .iter()
        .map(|line| {
            return line
                .split(" -> ")
                .map(|pair| {
                    let parsed: Vec<i32> =
                        pair.split(',').map(|n| n.parse::<i32>().unwrap()).collect();
                    (parsed[0], parsed[1])
                })
                .collect::<Vec<(i32, i32)>>();
        })
        .collect();

    for coord_pair in coords {
        let (mut x1, mut y1) = *coord_pair.get(0).unwrap();
        let (x2, y2) = *coord_pair.get(1).unwrap();

        let direction = ((x2 - x1).signum(), (y2 - y1).signum());

        // Add the lines
        while (x1, y1) != (x2, y2) {
            board.add_coord(x1 as usize, y1 as usize);
            x1 += direction.0;
            y1 += direction.1;
        }
        board.add_coord(x1 as usize, y1 as usize);
    }

    println!("part two: {} overlaps", board.count_overlaps());
}
