use std::fmt::Display;

pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T: Display + Ord> Grid<T> {
    pub fn new() -> Self {
        Grid { data: Vec::new() }
    }

    pub fn from(data: Vec<Vec<T>>) -> Self {
        Grid { data }
    }

    pub fn data(&self) -> &Vec<Vec<T>> {
        &self.data
    }

    pub fn add_row(&mut self, row: Vec<T>) {
        self.data.push(row);
    }

    pub fn row_iter(&self) -> impl Iterator<Item = &Vec<T>> + '_ {
        self.data.iter()
    }

    pub fn row_iter_mut(&mut self) -> impl Iterator<Item = &Vec<T>> + '_ {
        self.data.iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.data.iter().flatten()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &T> + '_ {
        self.data.iter().flatten()
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        if let Some(row) = self.data.get(y as usize) {
            if let Some(value) = row.get(x as usize) {
                return Some(value);
            }
        }

        None
    }

    pub fn show(&self, gap: usize) {
        self.data.iter().for_each(|row| {
            row.iter()
                .for_each(|item| print!("{:<1$}", *item, self.max().to_string().len() + gap));
            println!();
        })
    }

    pub fn max(&self) -> &T {
        self.data.iter().flatten().max().unwrap()
    }

    pub fn neighbours4_values(&self, x: isize, y: isize) -> Vec<&T> {
        let up = self.get(x, y - 1);
        let down = self.get(x, y + 1);
        let left = self.get(x - 1, y);
        let right = self.get(x + 1, y);

        vec![up, down, left, right]
            .iter()
            .filter_map(|x| *x)
            .collect()
    }

    pub fn neighbours4_coords(&self, x: isize, y: isize) -> Vec<(usize, usize, &T)> {
        let up = (x, y - 1, self.get(x, y - 1));
        let down = (x, y + 1, self.get(x, y + 1));
        let left = (x - 1, y, self.get(x - 1, y));
        let right = (x + 1, y, self.get(x + 1, y));

        vec![up, down, left, right]
            .iter()
            .filter(|(_x, _y, value)| match *value {
                Some(_) => true,
                None => false,
            })
            .map(|(x, y, value)| (*x as usize, *y as usize, value.unwrap()))
            .collect()
    }

    pub fn neighbours8_values(&self, x: isize, y: isize) -> Vec<&T> {
        let up = self.get(x, y - 1);
        let down = self.get(x, y + 1);
        let left = self.get(x - 1, y);
        let right = self.get(x + 1, y);
        let top_right = self.get(x + 1, y - 1);
        let top_left = self.get(x - 1, y - 1);
        let bottom_right = self.get(x + 1, y + 1);
        let bottom_left = self.get(x - 1, y + 1);

        vec![
            up,
            down,
            left,
            right,
            top_right,
            top_left,
            bottom_right,
            bottom_left,
        ]
        .iter()
        .filter_map(|x| *x)
        .collect()
    }

    pub fn neighbours8_coords(&self, x: isize, y: isize) -> Vec<(usize, usize, &T)> {
        let up = (x, y - 1, self.get(x, y - 1));
        let down = (x, y + 1, self.get(x, y + 1));
        let left = (x - 1, y, self.get(x - 1, y));
        let right = (x + 1, y, self.get(x + 1, y));
        let top_right = (x + 1, y - 1, self.get(x + 1, y - 1));
        let top_left = (x - 1, y - 1, self.get(x - 1, y - 1));
        let bottom_right = (x + 1, y + 1, self.get(x + 1, y + 1));
        let bottom_left = (x - 1, y + 1, self.get(x - 1, y + 1));

        vec![
            up,
            down,
            left,
            right,
            top_right,
            top_left,
            bottom_right,
            bottom_left,
        ]
        .iter()
        .filter(|(_x, _y, value)| match *value {
            Some(_) => true,
            None => false,
        })
        .map(|(x, y, value)| (*x as usize, *y as usize, value.unwrap()))
        .collect()
    }
}
