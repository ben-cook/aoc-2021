pub fn show_board(board: &[Vec<bool>]) {
    for row in board {
        for element in row {
            if *element {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());

    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

pub fn up_fold(mut array: Vec<Vec<bool>>, n: usize) -> Vec<Vec<bool>> {
    let (top, bottom) = array.split_at_mut(n);
    let bottom = &mut bottom[1..];

    let mut result: Vec<Vec<bool>> = Vec::new();

    let longer_length = std::cmp::max(top.len(), bottom.len());

    for y in 0..longer_length {
        let top_row = top.get(top.len() - 1 - y);
        let btm_row = bottom.get(y);
        
        if top_row.is_some() && btm_row.is_none() {
            result.push(top_row.unwrap().to_vec());
        } else if top_row.is_none() && btm_row.is_some() {
            result.push(btm_row.unwrap().to_vec());
        } else if top_row.is_some() && btm_row.is_some() {
            let row = top_row.unwrap().iter().zip(btm_row.unwrap().iter()).map(|(a, b)| *a || *b).collect();

            result.push(row);
        }
    }

    result.reverse();
    result
}

pub fn left_fold(array: Vec<Vec<bool>>, n: usize) -> Vec<Vec<bool>> {
    let transposed = transpose(array);
    let folded = up_fold(transposed, n);

    transpose(folded)
}

pub fn count_dots(array: &[Vec<bool>]) -> i64 {
    array.iter().map(|line| line.iter().filter(|v| **v).count() as i64).sum()
}

pub fn execute_fold(array: Vec<Vec<bool>>, c: char, n: usize) -> Vec<Vec<bool>> {
    match c {
        'y' => up_fold(array, n),
        'x' => left_fold(array, n),
        _ => unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transpose_test_1() {
        let original = vec![vec![1, 2], vec![3, 4]];
        let transposed = vec![vec![1, 3], vec![2, 4]];

        assert_eq!(transpose(original), transposed);
    }

    #[test]
    fn transpose_test_2() {
        let original = vec![vec![1, 2, 3], vec![3, 4, 5]];
        let transposed = vec![vec![1, 3], vec![2, 4], vec![3, 5]];

        assert_eq!(transpose(original), transposed);
    }

    #[test]
    fn up_fold_test_middle() {
        let original = vec![vec![true, false], 
                                         vec![false, false],
                                         vec![false, true]];
        let folded = vec![vec![true, true]];
                                         
        assert_eq!(up_fold(original, 1), folded);
    }

    #[test]
    fn up_fold_test_offset_fold() {
        let original = vec![vec![true, false], 
                                         vec![false, false],
                                         vec![false, true],
                                         vec![false, true]];
        let folded = vec![vec![true, false], vec![false, true]];
                                         
        assert_eq!(up_fold(original, 2), folded);
    }

    #[test]
    fn left_fold_test() {
        let original = vec![vec![true, false, false], 
                                         vec![false, false, false],
                                         vec![false, true, true]];
        let folded = vec![vec![true], vec![false], vec![true]];
                                         
        assert_eq!(left_fold(original, 1), folded);
    }
}
