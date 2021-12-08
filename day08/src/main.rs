use itertools::Itertools;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = include_str!("../input.txt");

    one(input);
    two(input);

    let duration = start.elapsed().as_secs_f64();
    println!("Execution time:  {}s", duration);
}

fn one(input: &str) {
    let count: i64 = input
        .lines()
        .map(|line| {
            line.trim()
                .split('|')
                .nth(1)
                .unwrap()
                .split(' ')
                .map(|v| v.len())
                .filter(|a| *a == 2 || *a == 3 || *a == 4 || *a == 7)
                .count() as i64
        })
        .sum();

    println!("[Part One] {}", count);
}

fn two(input: &str) {
    let sum = input
        .lines()
        .map(|line| {
            let (signal_patterns, output_values) = line.trim().split_once('|').unwrap();

            let signal_patterns: Vec<_> = signal_patterns.trim().split_whitespace().collect();
            let output_values: Vec<_> = output_values.trim().split_whitespace().collect();

            // Return the decoded message, using the first permutation that works
            "abcdefg"
                .chars()
                .permutations(7)
                .find_map(|perm| try_permutation(&perm, &signal_patterns, &output_values))
                .unwrap()
        })
        .sum::<usize>();

    println!("[Part Two] {}", sum);
}

fn display_digit(s: &str) -> Option<usize> {
    let seven_segment_representations = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];

    seven_segment_representations
        .into_iter()
        .find_position(|x| *x == s)
        .map(|(index, _)| index)
}

fn try_permutation(
    perm: &[char],
    signal_patterns: &[&str],
    output_values: &[&str],
) -> Option<usize> {
    // If any of the signal patterns aren't valid seven-segment numbers, this permutation is wrong
    let invalid = signal_patterns
        .iter()
        .map(|s| display_digit(&translate_and_sort(perm, s)))
        .any(|o| o.is_none());

    if invalid {
        return None;
    }

    let ans = output_values
        .iter()
        .rev()
        .enumerate()
        .map(|(i, s)| display_digit(&translate_and_sort(perm, s)).unwrap() * 10usize.pow(i as u32))
        .sum();

    Some(ans)
}

fn translate_and_sort(perm: &[char], s: &str) -> String {
    s.chars()
        .map(|char| {
            let index_in_permutation = perm
                .iter()
                .position(|v| *v == char)
                .expect("permutation does not contain char");

            "abcdefg".chars().nth(index_in_permutation).unwrap()
        })
        .sorted()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_digit() {
        assert_eq!(Some(0), display_digit("abcefg"));
        assert_eq!(Some(1), display_digit("cf"));
    }
}
