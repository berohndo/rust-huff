use std::cmp::Ordering;

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
