use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Eq, PartialEq)]
pub enum Tree {
    Leaf {
        value: usize,
        character: char,
    },
    Node {
        value: usize,
        left: Option<Box<Tree>>,
        right: Option<Box<Tree>>,
    },
}

pub fn frequencies(input: &str) -> HashMap<char, usize> {
    let mut f = HashMap::new();
    for character in input.chars() {
        *f.entry(character).or_insert(0) += 1;
    }
    f
}

pub fn bin_heap_min(input: &str) -> BinaryHeap<Tree> {
    let f = frequencies(input);

    let mut h = BinaryHeap::new();
    for (character, count) in f {
        h.push(Tree::Leaf {
            value: count,
            character,
        })
    }

    h
}

pub fn to_tree(input: &str) -> Tree {
    let mut heap = bin_heap_min(input);

    while heap.len() > 1 {
        let first = heap.pop().unwrap();
        let second = heap.pop().unwrap();
        heap.push(Tree::Node {
            value: get_value(&first) + get_value(&second),
            left: Some(Box::new(first)),
            right: Some(Box::new(second)),
        })
    }

    heap.pop().unwrap()
}

fn get_value(tree: &Tree) -> usize {
    match tree {
        Tree::Node { value, .. } | Tree::Leaf { value, .. } => *value,
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Tree::Leaf { value: selfval, .. } => match other {
                Tree::Leaf {
                    value: otherval, ..
                }
                | Tree::Node {
                    value: otherval, ..
                } => selfval.cmp(otherval).reverse(),
            },
            Tree::Node { value: selfval, .. } => match other {
                Tree::Leaf {
                    value: otherval, ..
                }
                | Tree::Node {
                    value: otherval, ..
                } => selfval.cmp(otherval).reverse(),
            },
        }
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

        assert_eq!(
            match result.pop() {
                Some(Tree::Leaf { character: x, .. }) => x,
                _ => panic!("Expected leaf"),
            },
            'b'
        );
    }

    #[test]
    fn test_to_tree() {
        println!("{:?}", to_tree("aabcccca"));
    }
}
