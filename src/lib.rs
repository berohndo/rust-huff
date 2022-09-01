use std::collections::{BinaryHeap, HashMap};

pub mod bitwriter;
pub mod tree;

use crate::tree::Tree;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    fn to_bool(self) -> bool {
        match self {
            Direction::Left => false,
            Direction::Right => true,
        }
    }
}

pub fn encode(input: &[u8]) -> (HashMap<u8, Vec<bool>>, Tree) {
    let tree = to_tree(input);
    let mut encoding = HashMap::new();
    traverse_inner(&tree, vec![], &mut encoding);

    (encoding, tree)
}

fn frequencies(input: &[u8]) -> HashMap<u8, usize> {
    let mut f = HashMap::new();
    for byte in input {
        *f.entry(*byte).or_insert(0) += 1_usize;
    }

    f
}

fn bin_heap_min(input: &[u8]) -> BinaryHeap<Tree> {
    let f = frequencies(input);

    let mut h = BinaryHeap::new();
    for (byte, count) in f {
        h.push(Tree::Leaf { value: count, byte })
    }

    h
}

fn to_tree(input: &[u8]) -> Tree {
    let mut heap = bin_heap_min(input);

    while heap.len() > 1 {
        let first = heap.pop().unwrap();
        let second = heap.pop().unwrap();
        heap.push(Tree::Branch {
            value: get_value(&first) + get_value(&second),
            left: Some(Box::new(first)),
            right: Some(Box::new(second)),
        })
    }

    heap.pop().unwrap()
}

fn traverse_inner(tree: &Tree, path: Vec<Direction>, encoding: &mut HashMap<u8, Vec<bool>>) {
    match tree {
        Tree::Leaf { byte, .. } => {
            encoding.insert(*byte, to_bool_path(&path));
        }
        Tree::Branch { left, right, .. } => {
            if let Some(left) = left {
                let mut path_left = path.clone();
                path_left.push(Direction::Left);
                traverse_inner(left, path_left, encoding);
            }

            if let Some(right) = right {
                let mut path_right = path;
                path_right.push(Direction::Right);
                traverse_inner(right, path_right, encoding);
            }
        }
    }
}

fn to_bool_path(path: &[Direction]) -> Vec<bool> {
    path.iter().map(|d| Direction::to_bool(*d)).collect()
}

fn get_value(tree: &Tree) -> usize {
    match tree {
        Tree::Branch { value, .. } | Tree::Leaf { value, .. } => *value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequencies() {
        let result = frequencies("aabcccca".as_bytes());
        assert_eq!(result.len(), 3);

        assert_eq!(result.get(&b'a').unwrap(), &3);
        assert_eq!(result.get(&b'b').unwrap(), &1);
        assert_eq!(result.get(&b'c').unwrap(), &4);
    }

    #[test]
    fn test_bin_min_heap() {
        let mut result = bin_heap_min("aabcccca".as_bytes());
        assert_eq!(result.len(), 3);

        assert_eq!(
            match result.pop() {
                Some(Tree::Leaf { byte: x, .. }) => x,
                _ => panic!("Expected leaf"),
            },
            b'b'
        );
    }

    #[test]
    fn test_travers() {
        let (table, _) = encode("aaabbc".as_bytes());
        assert_eq!(table.get(&b'a').unwrap(), &vec![false]);
        assert_eq!(table.get(&b'c').unwrap(), &vec![true, false]);
        assert_eq!(table.get(&b'b').unwrap(), &vec![true, true]);
    }
}
