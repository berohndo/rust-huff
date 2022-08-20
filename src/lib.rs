use core::fmt;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Display;

mod tree;

use crate::tree::Tree;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Direction::Left => "0",
            Direction::Right => "1",
        };

        write!(f, "{}", s)
    }
}

pub fn encode(input: &str) -> (HashMap<char, String>, Tree) {
    let tree = to_tree(input);
    let mut encoding = HashMap::new();
    traverse_inner(&tree, vec![], &mut encoding);

    (encoding, tree)
}

pub fn decode(tree: &Tree, input: &str) -> String {
    let mut pointer = tree;

    let mut result = String::new();

    for bit in input.chars() {
        match bit {
            '0' => {
                if let Tree::Node {
                    left: Some(left), ..
                } = pointer
                {
                    pointer = left.as_ref();
                }
            }
            '1' => {
                if let Tree::Node {
                    right: Some(right), ..
                } = pointer
                {
                    pointer = right.as_ref();
                }
            }
            _ => panic!("unexpected char"),
        }

        if let Tree::Leaf { character, .. } = pointer {
            result.push(*character);
            pointer = tree;
        }
    }

    result
}

fn frequencies(input: &str) -> HashMap<char, usize> {
    let mut f = HashMap::new();
    for character in input.chars() {
        *f.entry(character).or_insert(0) += 1;
    }

    f
}

fn bin_heap_min(input: &str) -> BinaryHeap<Tree> {
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

fn to_tree(input: &str) -> Tree {
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

fn traverse_inner(tree: &Tree, path: Vec<Direction>, encoding: &mut HashMap<char, String>) {
    match tree {
        Tree::Leaf { character, .. } => {
            encoding.insert(*character, to_path(&path));
        }
        Tree::Node { left, right, .. } => {
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

fn to_path(path: &[Direction]) -> String {
    path.iter()
        .map(Direction::to_string)
        .collect::<Vec<String>>()
        .join("")
}

fn get_value(tree: &Tree) -> usize {
    match tree {
        Tree::Node { value, .. } | Tree::Leaf { value, .. } => *value,
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
    fn test_travers() {
        let (table, _) = encode("aaabbc");
        assert_eq!(table.get(&'a').unwrap(), "0");
        assert_eq!(table.get(&'c').unwrap(), "10");
        assert_eq!(table.get(&'b').unwrap(), "11");
    }
}
