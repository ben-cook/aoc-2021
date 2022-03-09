use itertools::Itertools;
use std::collections::HashMap;

pub fn method_one(input: &str, iterations: usize) -> i64 {
    let mut lines = input.lines();
    let polymer_template = lines.next().unwrap();
    lines.next();

    let mut map = HashMap::new();

    lines.for_each(|line| {
        let (a, b) = line.split_once(" -> ").unwrap();
        map.insert(a, b);
    });

    let mut answer = polymer_template.to_string();
    for _ in 0..iterations {
        answer = answer
            .chars()
            .tuple_windows()
            .map(|(c1, c2)| {
                let combined_str = c1.to_string() + &c2.to_string();

                match map.get(&combined_str as &str) {
                    Some(value) => c1.to_string() + &value.to_string(),
                    None => c1.to_string(),
                }
            })
            .collect::<String>()
            + &(answer.chars().last().unwrap().to_string());
    }

    let mut count = HashMap::new();

    for item in answer.chars() {
        *count.entry(item).or_insert(0) += 1;
    }

    let most_common = count.iter().sorted_by_key(|x| -x.1).next().unwrap();
    let least_common = count.iter().sorted_by_key(|x| x.1).next().unwrap();

    most_common.1 - least_common.1
}

pub fn method_two(input: &str, iterations: usize) -> i64 {
    let mut lines = input.lines();
    let polymer_template = lines.next().unwrap();
    lines.next();

    let mut transformation_map = HashMap::new();

    lines.for_each(|line| {
        let (a, b) = line.split_once(" -> ").unwrap();
        transformation_map.insert(a.to_string(), b);
    });

    let mut pair_map = HashMap::new();

    for (c1, c2) in polymer_template.chars().tuple_windows() {
        let combined_str = c1.to_string() + &c2.to_string();
        let entry = pair_map.entry(combined_str).or_insert(0);
        *entry += 1;
    }

    for _ in 0..iterations {
        for (current_pair, current_amount) in &pair_map.clone() {
            if transformation_map.contains_key(current_pair) && *current_amount > 0 {
                let target_char = *transformation_map.get(current_pair).unwrap();
                let pair1 = current_pair.chars().next().unwrap().to_string() + target_char;
                let pair2 =
                    target_char.to_string() + &current_pair.chars().nth(1).unwrap().to_string();

                let p1_entry = pair_map.entry(pair1).or_insert(0);
                *p1_entry += current_amount;

                let p2_entry = pair_map.entry(pair2).or_insert(0);
                *p2_entry += current_amount;

                let og_ref = pair_map.get_mut(current_pair).unwrap();
                *og_ref -= current_amount;
            }
        }
    }

    let mut char_count = HashMap::new();

    for (pair, pair_count) in pair_map {
        if pair_count > 0 {
            let char1 = pair.chars().next().unwrap();
            let char2 = pair.chars().nth(1).unwrap();

            let entry1 = char_count.entry(char1).or_insert(0);
            *entry1 += pair_count;

            let entry2 = char_count.entry(char2).or_insert(0);
            *entry2 += pair_count;
        }
    }

    let first_letter = polymer_template.chars().next().unwrap();
    let last_letter = polymer_template.chars().last().unwrap();

    let first_entry = char_count.get_mut(&first_letter).unwrap();
    *first_entry += 1;

    let last_entry = char_count.get_mut(&last_letter).unwrap();
    *last_entry += 1;

    for val in &mut char_count.values_mut() {
        *val /= 2;
    }

    let most_common = char_count.iter().sorted_by_key(|x| x.1).last().unwrap();
    let least_common = char_count.iter().sorted_by_key(|x| x.1).next().unwrap();

    most_common.1 - least_common.1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_10() {
        let input = include_str!("../example.txt");

        assert_eq!(method_one(input, 10), 1588);
    }

    #[test]
    fn two_x() {
        let input = include_str!("../example.txt");

        assert_eq!(method_two(input, 0), 1);
    }

    #[test]
    fn two_10() {
        let input = include_str!("../example.txt");

        assert_eq!(method_two(input, 10), 1588);
    }

    #[test]
    fn two_40() {
        let input = include_str!("../example.txt");

        assert_eq!(method_two(input, 40), 2188189693529);
    }
}
