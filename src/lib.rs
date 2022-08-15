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

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

// TODO proper joining???
static ZERO: &str = "0";
static ONE: &str = "1";

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Left => ZERO.to_string(),
            _ => ONE.to_string(),
        }
    }
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

pub fn encode(input: &str) -> HashMap<char, String> {
    let tree = to_tree(input);
    let mut encoding = HashMap::new();
    traverse_inner(&tree, Box::new(vec![]), &mut encoding);

    encoding
}

fn traverse_inner(tree: &Tree, path: Box<Vec<Direction>>, encoding: &mut HashMap<char, String>) {
    match tree {
        Tree::Leaf { character, .. } => {
            encoding.insert(*character, to_path(&path));
        }
        Tree::Node { left, right, .. } => {
            if let Some(left) = left {
                let mut path_left = Box::new(*path.clone());
                path_left.push(Direction::Left);
                traverse_inner(left, path_left, encoding);
            }

            if let Some(right) = right {
                let mut path_right = Box::new(*path);
                path_right.push(Direction::Right);
                traverse_inner(right, path_right, encoding);
            }
        }
    }
}
// TODO proper joining???
fn to_path(path: &Vec<Direction>) -> String {
    let mut s = String::from("");

    for d in path {
        s.push_str(&d.to_string())
    }

    s
}

fn get_value(tree: &Tree) -> usize {
    match tree {
        Tree::Node { value, .. } | Tree::Leaf { value, .. } => *value,
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
    fn test_travers() {
        let table = encode("aaabbc");
        assert_eq!(table.get(&'a').unwrap(), "0");
        assert_eq!(table.get(&'c').unwrap(), "10");
        assert_eq!(table.get(&'b').unwrap(), "11");
    }
}
