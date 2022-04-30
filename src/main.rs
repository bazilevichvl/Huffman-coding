use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn count_bytes(data: &[u8]) -> HashMap<u8, usize> {
    let mut counts = HashMap::new();
    for byte in data {
        *counts.entry(*byte).or_insert(0usize) += 1;
    }
    counts
}

struct Node {
    kind: NodeType,
    weight: usize,
}

impl Node {
    fn new_leaf(byte: u8, weight: usize) -> Self {
        Node {
            kind: NodeType::Leaf(byte),
            weight,
        }
    }

    fn merge(left: Node, right: Node) -> Self {
        Node {
            weight: left.weight + right.weight,
            kind: NodeType::Internal(Some(Box::new(left)), Some(Box::new(right))),
        }
    }
}

enum NodeType {
    Internal(Option<Box<Node>>, Option<Box<Node>>),
    Leaf(u8),
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn empty_counts_if_data_is_empty() {
        let data = vec![];
        let counts = count_bytes(&data);
        assert_eq!(counts.is_empty(), true);
    }

    #[test]
    fn counts_bytes_frequencies() {
        let data = vec![255, 255, 0, 1, 7];
        let counts = count_bytes(&data);
        assert_eq!(counts.keys().len(), 4);
        assert_eq!(counts.get(&255), Some(&2));
        assert_eq!(counts.get(&0), Some(&1));
        assert_eq!(counts.get(&1), Some(&1));
        assert_eq!(counts.get(&7), Some(&1));
    }

    #[test]
    fn merge_two_leaves() {
        let left = Node::new_leaf(3, 5);
        let right = Node::new_leaf(5, 1);
        let merged = Node::merge(left, right);
        assert_eq!(merged.weight, 6);
        assert_eq!(matches!(merged.kind, NodeType::Internal(_, _)), true);
    }
}
