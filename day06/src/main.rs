use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    one(input);
    two(input);
}

fn one(input: &str) {
    let mut fishes: Vec<i32> = input
        .trim()
        .split(',')
        .map(|num| num.parse().expect("Couldn't parse input"))
        .collect();

    const DAYS: usize = 80;
    for _ in 0..DAYS {
        fishes = fishes.into_iter().map(|fish| fish - 1).collect::<Vec<_>>();

        let mut new_fish = 0;

        for fish in &fishes {
            if *fish == -1 {
                new_fish += 1;
            }
        }

        fishes = fishes
            .into_iter()
            .map(|fish| if fish == -1 { 6 } else { fish })
            .collect::<Vec<_>>();

        fishes.resize(fishes.len() + new_fish, 8);
    }

    println!("part one: {} total fish after {} days", fishes.len(), DAYS);
}

fn two(input: &str) {
    // Count the fish into a HashMap
    let fishes: Vec<i32> = input
        .trim()
        .split(',')
        .map(|num| num.parse().expect("Couldn't parse input"))
        .collect();

    let mut fish_map = HashMap::new();

    for i in 0..9 {
        fish_map.insert(i, 0);
    }

    for item in fishes {
        *fish_map.entry(item).or_insert(0) += 1;
    }

    const DAYS: usize = 256;
    for _ in 0..DAYS {
        let old_fish_map = fish_map.clone();

        // Age the fish
        for n in 0..8 {
            let entry = fish_map.get_mut(&n).unwrap();
            let entry_above = *old_fish_map.get(&(n + 1)).unwrap();
            *entry = entry_above;
        }

        // Turn 0's into 6's and 8's
        let zeroes = *old_fish_map.get(&0).unwrap();
        let sixes = fish_map.get_mut(&6).unwrap();
        *sixes += zeroes;
        let eights = fish_map.get_mut(&8).unwrap();
        *eights = zeroes;
    }

    println!(
        "part two: {} fish after {} days",
        fish_map.values().sum::<i128>(),
        DAYS
    );
}
