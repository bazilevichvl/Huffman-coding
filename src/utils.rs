use crate::Node;
use std::collections::{BinaryHeap, HashMap};

pub fn count_bytes(data: &[u8]) -> HashMap<u8, usize> {
    let mut counts = HashMap::new();
    for byte in data {
        *counts.entry(*byte).or_insert(0usize) += 1;
    }
    counts
}

pub fn build_huffman_codes_tree(counts: HashMap<u8, usize>) -> Result<Node, String> {
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

pub struct RawFile(Vec<u8>);

impl RawFile {
    pub fn new(path: String) -> Self {
        let content = std::fs::read(path).expect("Can't read input file");
        Self(content)
    }

    pub fn size(&self) -> usize {
        self.0.len() * 8
    }
}

pub struct EncodedFile(HashMap<u8, String>, String);

impl EncodedFile {
    pub fn size(&self) -> usize {
        self.1.len()
    }
}

pub fn encode(file: &RawFile) -> EncodedFile {
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
    use std::collections::HashMap;

    #[test]
    fn empty_counts_if_data_is_empty() {
        let data = vec![];
        let counts = count_bytes(&data);
        assert_eq!(counts.is_empty(), true);
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
