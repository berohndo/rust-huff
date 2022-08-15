use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn frequencies(input: &str) -> HashMap<char, usize> {
    let mut f = HashMap::new();
    for character in input.chars() {
        *f.entry(character).or_insert(0) += 1;
    }
    f
}

pub fn bin_heap_min(input: &str) -> BinaryHeap<CharInfo> {
    let mut f = HashMap::new();
    for character in input.chars() {
        *f.entry(character).or_insert(0) += 1;
    }

    let mut h = BinaryHeap::new();
    for (character, count) in f {
        h.push(CharInfo {
            char: character,
            count,
        })
    }

    h
}
#[derive(Debug)]
pub struct CharInfo {
    count: usize,
    char: char,
}

impl PartialEq for CharInfo {
    fn eq(&self, other: &Self) -> bool {
        println!(
            "PartialEq.eq called with self: {:?} and other: {:?}",
            self, other
        );
        self.count == other.count
    }
}

impl Eq for CharInfo {}

impl PartialOrd for CharInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        println!(
            "PartialOrd.partial_cmp called with self: {:?} and other: {:?}",
            self, other
        );
        Some(self.cmp(&other)) // Delegate to the implementation in `Ord`.
    }
}

impl Ord for CharInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        println!(
            "Ord.cmp called with self: {:?} and other: {:?}",
            self, other
        );
        self.count.cmp(&other.count).reverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequencies() {
        let result = frequencies("aabcccca");
        assert_eq!(result.len(), 3);

        assert_eq!(result.get(&'a').unwrap(), &3);
        assert_eq!(result.get(&'b').unwrap(), &1);
        assert_eq!(result.get(&'c').unwrap(), &4);
    }

    #[test]
    fn test_bin_min_heap() {
        let mut result = bin_heap_min("aabcccca");
        assert_eq!(result.len(), 3);

        assert_eq!(result.pop().unwrap().char, 'b');
    }
}
