use itertools::Itertools;
use petgraph::{graph::NodeIndex, Graph};
use std::{collections::HashMap, time::Instant};

fn main() {
    let start = Instant::now();

    let input = include_str!("../input.txt");

    println!("[Part One]       {}", one(input));
    println!("[Part Two]       {}", two(input));

    let duration = start.elapsed().as_secs_f64();
    println!("[Execution Time] {:.4}s", duration);
}

#[derive(PartialEq, Eq)]
enum CaveSize {
    Big,
    Small,
}

struct Node {
    label: String,
    size: CaveSize,
}

struct Edge {}

fn one(input: &str) -> i64 {
    let mut graph: Graph<Node, Edge, petgraph::Undirected> = petgraph::graph::UnGraph::default();

    let mut node_map = HashMap::new();

    // First add nodes
    input.lines().for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();

        if !node_map.keys().contains(&a) {
            let node_a = Node {
                label: a.to_string(),
                size: if a.to_uppercase() == a {
                    CaveSize::Big
                } else {
                    CaveSize::Small
                },
            };

            let pushed_node_idx = graph.add_node(node_a);
            node_map.insert(a, pushed_node_idx);
        }

        if !node_map.keys().contains(&b) {
            let node_b = Node {
                label: b.to_string(),
                size: if b.to_uppercase() == b {
                    CaveSize::Big
                } else {
                    CaveSize::Small
                },
            };

            let pushed_node_idx = graph.add_node(node_b);
            node_map.insert(b, pushed_node_idx);
        }
    });

    // Then Add Edges
    input.lines().for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();
        graph.add_edge(
            *node_map.get(a).unwrap(),
            *node_map.get(b).unwrap(),
            Edge {},
        );
    });

    fn find_paths_to_end(
        current_node: NodeIndex,
        current_visited: &[NodeIndex],
        count: i64,
        graph: &Graph<Node, Edge, petgraph::Undirected>,
        end_index: NodeIndex,
    ) -> i64 {
        let neighbours = graph.neighbors(current_node);

        let available_neighbours = neighbours
            .filter(|neighbour| {
                !(graph
                    .node_weight(*neighbour)
                    .expect("couldn't find node in graph")
                    .size
                    == CaveSize::Small
                    && current_visited.contains(neighbour))
            })
            .collect_vec();

        if current_node == end_index {
            return 1;
        }

        let mut new_visited = <&[petgraph::prelude::NodeIndex]>::clone(&current_visited).to_vec();
        new_visited.push(current_node);

        available_neighbours
            .iter()
            .map(|neighbour| {
                find_paths_to_end(*neighbour, &new_visited, count + 1, graph, end_index)
            })
            .sum()
    }

    let start_idx = *node_map.get("start").unwrap();
    let end_idx = *node_map.get("end").unwrap();

    let mut visited: Vec<NodeIndex> = Vec::new();

    find_paths_to_end(start_idx, &visited, 0, &graph, end_idx)
}

fn two(input: &str) -> i64 {
    let mut graph: Graph<Node, Edge, petgraph::Undirected> = petgraph::graph::UnGraph::default();

    let mut node_map = HashMap::new();

    // First add nodes
    input.lines().for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();

        if !node_map.keys().contains(&a) {
            let node_a = Node {
                label: a.to_string(),
                size: if a.to_uppercase() == a {
                    CaveSize::Big
                } else {
                    CaveSize::Small
                },
            };

            let pushed_node_idx = graph.add_node(node_a);
            node_map.insert(a, pushed_node_idx);
        }

        if !node_map.keys().contains(&b) {
            let node_b = Node {
                label: b.to_string(),
                size: if b.to_uppercase() == b {
                    CaveSize::Big
                } else {
                    CaveSize::Small
                },
            };

            let pushed_node_idx = graph.add_node(node_b);
            node_map.insert(b, pushed_node_idx);
        }
    });

    // Then Add Edges
    input.lines().for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();
        graph.add_edge(
            *node_map.get(a).unwrap(),
            *node_map.get(b).unwrap(),
            Edge {},
        );
    });

    fn find_paths_to_end(
        current_node: NodeIndex,
        current_visited: &[NodeIndex],
        count: i64,
        graph: &Graph<Node, Edge, petgraph::Undirected>,
        end_index: NodeIndex,
    ) -> i64 {
        let neighbours = graph.neighbors(current_node);

        let available_neighbours = neighbours
            .filter(|neighbour| {
                !(graph
                    .node_weight(*neighbour)
                    .expect("couldn't find node in graph")
                    .size
                    == CaveSize::Small
                    && current_visited.contains(neighbour))
            })
            .collect_vec();

        if current_node == end_index {
            return 1;
        }

        let mut new_visited = <&[petgraph::prelude::NodeIndex]>::clone(&current_visited).to_vec();
        new_visited.push(current_node);

        available_neighbours
            .iter()
            .map(|neighbour| {
                find_paths_to_end(*neighbour, &new_visited, count + 1, graph, end_index)
            })
            .sum()
    }

    let start_idx = *node_map.get("start").unwrap();
    let end_idx = *node_map.get("end").unwrap();

    let mut visited: Vec<NodeIndex> = Vec::new();

    find_paths_to_end(start_idx, &visited, 0, &graph, end_idx)
}
