use clap::Parser;
use std::collections::{BinaryHeap, HashMap};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    input_file: String,
}

fn main() {
    let args = Args::parse();

    let input = RawFile::new(args.input_file);
    let raw_size = input.size();
    println!("Raw file size: {} bits", raw_size);
    let encoded = encode(&input);
    let encoded_size = encoded.size();
    println!("Encoded file size: {} bits", encoded_size);

    println!(
        "Compression ratio: {:.3}",
        raw_size as f64 / encoded_size as f64
    );
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

    fn lookup_table(&self) -> HashMap<u8, String> {
        let mut table = HashMap::new();
        self.lookup_table_rec(&mut table, "".to_owned());

        table
    }

    fn lookup_table_rec(&self, table: &mut HashMap<u8, String>, prefix: String) {
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

enum NodeType {
    Internal(Option<Box<Node>>, Option<Box<Node>>),
    Leaf(u8),
}

fn build_huffman_codes_tree(counts: HashMap<u8, usize>) -> Result<Node, String> {
    if counts.keys().len() <= 2 {
        return Err(
            "It makes no sense to use Huffman coding for less than three symbols".to_owned(),
        );
    }

    let mut trees = BinaryHeap::new();
    for (symbol, weight) in counts {
        let root = Node::new_leaf(symbol, weight);
        trees.push(root);
    }

    while trees.len() > 1 {
        let left = trees.pop().unwrap();
        let right = trees.pop().unwrap();
        trees.push(Node::merge(left, right));
    }

    Ok(trees.pop().unwrap())
}

struct RawFile(Vec<u8>);

impl RawFile {
    fn new(path: String) -> Self {
        let content = std::fs::read(path).expect("Can't read input file");
        Self(content)
    }

    fn size(&self) -> usize {
        self.0.len() * 8
    }
}

struct EncodedFile(HashMap<u8, String>, String);

impl EncodedFile {
    fn size(&self) -> usize {
        self.1.len()
    }
}

fn encode(file: &RawFile) -> EncodedFile {
    let counts = count_bytes(&file.0);
    let huffman_tree = build_huffman_codes_tree(counts).unwrap();
    let codes = huffman_tree.lookup_table();
    let mut encoded_content = String::new();
    for symbol in &file.0 {
        encoded_content.push_str(codes.get(symbol).unwrap());
    }

    EncodedFile(codes, encoded_content)
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

    #[test]
    fn huffman_codes_construction_fails_for_one_symbol() {
        let mut counts = HashMap::new();
        counts.insert(42, 1);
        assert_eq!(matches!(build_huffman_codes_tree(counts), Err(_)), true);
    }

    #[test]
    fn huffman_codes_construction_fails_for_two_symbols() {
        let mut counts = HashMap::new();
        counts.insert(42, 1);
        counts.insert(3, 5);
        assert_eq!(matches!(build_huffman_codes_tree(counts), Err(_)), true);
    }

    #[test]
    fn huffman_codes_are_no_longer_than_two_for_symbols_in_word_test() {
        let data = vec![b't', b'e', b's', b't'];
        let counts = count_bytes(&data);

        let root = build_huffman_codes_tree(counts).unwrap();
        let table = root.lookup_table();

        assert_eq!(table.get(&b't').unwrap().len(), 1);
        assert_eq!(table.get(&b'e').unwrap().len(), 2);
        assert_eq!(table.get(&b's').unwrap().len(), 2);
    }
}
