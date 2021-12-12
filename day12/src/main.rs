use itertools::Itertools;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = include_str!("../example1.txt");

    println!("[Part One]       {}", one(input));
    println!("[Part Two]       {}", two(input));

    let duration = start.elapsed().as_secs_f64();
    println!("[Execution Time] {:.4}s", duration);
}

enum CaveSize {
    Big,
    Small,
}

struct Node<'a> {
    label: String,
    edges: Vec<Edge<'a>>,
    size: CaveSize,
}

impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Node) -> bool {
        self.label == other.label
    }

    fn ne(&self, other: &Node) -> bool {
        self.label != other.label
    }
}

struct Edge<'a> {
    node1: &'a Node<'a>,
    node2: &'a Node<'a>,
}

fn one(input: &str) -> i64 {
    let all_nodes: Vec<Node> = Vec::new();

    input.lines().for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();

        let node1 = Node {
            label: a.to_string(),
            edges: Vec::new(),
            size: if a.to_uppercase() == a {
                CaveSize::Big
            } else {
                CaveSize::Small
            },
        };

        let node2 = Node {
            label: b.to_string(),
            edges: Vec::new(),
            size: if b.to_uppercase() == b {
                CaveSize::Big
            } else {
                CaveSize::Small
            },
        };

        // if all_nodes.contains(node1) {
        //     let new
        // }

        // if all_nodes.contains
        // all_nodes.push()
    });
    todo()
}

fn two(input: &str) -> i64 {
    todo!()
}
