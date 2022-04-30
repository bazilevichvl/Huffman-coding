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
}
