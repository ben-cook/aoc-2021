use petgraph::{algo::astar, visit::EdgeRef, Graph};
use std::{collections::HashMap, time::Instant};

fn main() {
    let start = Instant::now();

    let input = include_str!("../input.txt");

    println!("[Part One]       {}", one(input));
    println!("[Part Two]       {}", two(input));

    let duration = start.elapsed().as_secs_f64();
    println!("[Execution Time] {:.4}s", duration);
}

struct Edge();

fn one(input: &str) -> i64 {
    let mut graph: Graph<i64, Edge, petgraph::Undirected> = petgraph::graph::UnGraph::default();

    let mut node_map = HashMap::new();

    let grid: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|o| o.into())
                .collect()
        })
        .collect();

    let col_length = grid.len();
    let row_length = input.lines().next().unwrap().len();

    // Add Nodes
    for y in 0..col_length {
        for x in 0..row_length {
            let pushed_node_idx = graph.add_node(*grid.get(y).unwrap().get(x).unwrap());
            node_map.insert((x, y), pushed_node_idx);
        }
    }

    // Add Edges (left-to-rights)
    for y in 0..col_length {
        for x in 0..(row_length - 1) {
            graph.add_edge(
                *node_map.get(&(x, y)).unwrap(),
                *node_map.get(&(x + 1, y)).unwrap(),
                Edge {},
            );
        }
    }

    // Add Edges (top-to-bottoms)
    for y in 0..(col_length - 1) {
        for x in 0..row_length {
            graph.add_edge(
                *node_map.get(&(x, y)).unwrap(),
                *node_map.get(&(x, y + 1)).unwrap(),
                Edge {},
            );
        }
    }

    let start = *node_map.get(&(0, 0)).unwrap();
    let end = *node_map.get(&(row_length - 1, col_length - 1)).unwrap();

    let (cost, _path) = astar(
        &graph,
        start,
        |finish| finish == end,
        |e| graph[e.target()],
        |_| 0,
    )
    .unwrap();

    cost
}

fn two(input: &str) -> i64 {
    let mut graph: Graph<i64, Edge, petgraph::Undirected> = petgraph::graph::UnGraph::default();

    let mut node_map = HashMap::new();

    let input_grid: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|o| o.into())
                .collect()
        })
        .collect();

    let og_col_length = input_grid.len();
    let og_row_length = input.lines().next().unwrap().len();

    let mut tall_grid: Vec<Vec<i64>> = Vec::new();

    // for i in 0..4 {
    let input_grid2 = &mut input_grid.clone();
    let input_grid3 = &mut input_grid
        .iter()
        .map(|row| -> Vec<i64> {
            row.iter()
                .map(|v| if *v < 9 { *v + 1 } else { 1 })
                .collect::<Vec<i64>>()
        })
        .collect();

    let input_grid4: &mut Vec<Vec<i64>> = &mut input_grid
        .iter()
        .map(|row| -> Vec<i64> {
            row.iter()
                .map(|v| if *v < 9 { *v + 1 } else { 1 })
                .map(|v| if v < 9 { v + 1 } else { 1 })
                .collect::<Vec<i64>>()
        })
        .collect();

    let input_grid5: &mut Vec<Vec<i64>> = &mut input_grid
        .iter()
        .map(|row| -> Vec<i64> {
            row.iter()
                .map(|v| if *v < 9 { *v + 1 } else { 1 })
                .map(|v| if v < 9 { v + 1 } else { 1 })
                .map(|v| if v < 9 { v + 1 } else { 1 })
                .collect::<Vec<i64>>()
        })
        .collect();

    let input_grid6: &mut Vec<Vec<i64>> = &mut input_grid
        .iter()
        .map(|row| -> Vec<i64> {
            row.iter()
                .map(|v| if *v < 9 { *v + 1 } else { 1 })
                .map(|v| if v < 9 { v + 1 } else { 1 })
                .map(|v| if v < 9 { v + 1 } else { 1 })
                .map(|v| if v < 9 { v + 1 } else { 1 })
                .collect::<Vec<i64>>()
        })
        .collect();

    tall_grid.append(input_grid2);
    tall_grid.append(input_grid3);
    tall_grid.append(input_grid4);
    tall_grid.append(input_grid5);
    tall_grid.append(input_grid6);

    let mut grid: Vec<Vec<i64>> = Vec::new();

    tall_grid.iter().for_each(|row| {
        let mut new_vec = row.clone();
        new_vec.extend(row.iter().map(|v| if *v < 9 { *v + 1 } else { 1 }));
        new_vec.extend(
            row.iter()
                .map(|v| if *v < 9 { *v + 1 } else { 1 })
                .map(|v| if v < 9 { v + 1 } else { 1 }),
        );
        new_vec.extend(
            row.iter()
                .map(|v| if *v < 9 { *v + 1 } else { 1 })
                .map(|v| if v < 9 { v + 1 } else { 1 })
                .map(|v| if v < 9 { v + 1 } else { 1 }),
        );
        new_vec.extend(
            row.iter()
                .map(|v| if *v < 9 { *v + 1 } else { 1 })
                .map(|v| if v < 9 { v + 1 } else { 1 })
                .map(|v| if v < 9 { v + 1 } else { 1 })
                .map(|v| if v < 9 { v + 1 } else { 1 }),
        );

        grid.push(new_vec);
    });

    show_grid(&grid);

    fn show_grid(grid: &[Vec<i64>]) {
        for row in grid {
            for element in row {
                print!("{}", element);
            }
            println!();
        }
    }

    let col_length = grid.len();
    let row_length = grid.get(0).unwrap().len();

    // Add Nodes
    for y in 0..col_length {
        for x in 0..row_length {
            let pushed_node_idx = graph.add_node(*grid.get(y).unwrap().get(x).unwrap());
            node_map.insert((x, y), pushed_node_idx);
        }
    }

    // Add Edges (left-to-rights)
    for y in 0..col_length {
        for x in 0..(row_length - 1) {
            graph.add_edge(
                *node_map.get(&(x, y)).unwrap(),
                *node_map.get(&(x + 1, y)).unwrap(),
                Edge {},
            );
        }
    }

    // Add Edges (top-to-bottoms)
    for y in 0..(col_length - 1) {
        for x in 0..row_length {
            graph.add_edge(
                *node_map.get(&(x, y)).unwrap(),
                *node_map.get(&(x, y + 1)).unwrap(),
                Edge {},
            );
        }
    }

    let start = *node_map.get(&(0, 0)).unwrap();
    let end = *node_map.get(&(row_length - 1, col_length - 1)).unwrap();

    let (cost, _path) = astar(
        &graph,
        start,
        |finish| finish == end,
        |e| graph[e.target()],
        |_| 0,
    )
    .unwrap();

    cost
}
