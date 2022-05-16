use std::collections::HashMap;

pub struct Node {
    kind: NodeType,
    weight: usize,
}

impl Node {
    pub fn new_leaf(byte: u8, weight: usize) -> Self {
        Node {
            kind: NodeType::Leaf(byte),
            weight,
        }
    }

    pub fn merge(left: Node, right: Node) -> Self {
        Node {
            weight: left.weight + right.weight,
            kind: NodeType::Internal(Some(Box::new(left)), Some(Box::new(right))),
        }
    }

    pub fn lookup_table(&self) -> HashMap<u8, String> {
        let mut table = HashMap::new();
        self.lookup_table_rec(&mut table, "".to_owned());

        table
    }

    pub fn lookup_table_rec(&self, table: &mut HashMap<u8, String>, prefix: String) {
        match self.kind {
            NodeType::Leaf(byte) => {
                table.insert(byte, prefix);
            }
            NodeType::Internal(Some(ref left), Some(ref right)) => {
                left.lookup_table_rec(table, prefix.clone() + "0");
                right.lookup_table_rec(table, prefix + "1");
            }
            // Due to construction algorithm internal node always has exactly
            // childs
            _ => unreachable!(),
        }
    }
}

impl std::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl std::cmp::Eq for Node {}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}

pub enum NodeType {
    Internal(Option<Box<Node>>, Option<Box<Node>>),
    Leaf(u8),
}

#[cfg(test)]
mod tests {
    use crate::*;

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
