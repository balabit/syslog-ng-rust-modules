use std::cmp::{Ord, Ordering};
use std::borrow::Borrow;

use matcher::trie::node::Node;

pub struct LiteralNode <'a, 'b> {
    literal: &'a str,
    node: Option<Box<Node<'a, 'b>>>,
}

impl <'a, 'b> LiteralNode<'a, 'b> {
    pub fn new(literal: &'a str) -> LiteralNode<'a, 'b> {
        LiteralNode{ literal: literal,
                     node: None}
    }

    fn compare_first_chars(&self, other : &LiteralNode) -> Ordering {
        if self.literal.is_empty() && other.literal.is_empty() {
            Ordering::Equal
        } else if self.literal.is_empty() {
            Ordering::Less
        } else if other.literal.is_empty() {
            Ordering::Greater
        } else {
            self.literal[0..1].cmp(&other.literal[0..1])
        }
    }
}

impl <'a, 'b> Eq for LiteralNode<'a, 'b> {}

impl <'a, 'b> PartialEq for LiteralNode<'a, 'b> {
    fn eq(&self, other: &Self) -> bool {
        self.compare_first_chars(other) == Ordering::Equal
    }

    fn ne(&self, other: &Self) -> bool {
        self.compare_first_chars(other) != Ordering::Equal
    }
}

impl <'a, 'b> Ord for LiteralNode<'a, 'b> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare_first_chars(other)
    }
}

impl <'a, 'b> PartialOrd for LiteralNode<'a, 'b> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare_first_chars(other))
    }
}

impl <'a, 'b> Borrow<str> for LiteralNode<'a, 'b> {
    fn borrow(&self) -> &str {
        self.literal
    }
}

#[cfg(test)]
mod test {
    use matcher::trie::node::LiteralNode;
    use std::cmp::Ordering;

    #[test]
    fn given_literal_node_when_it_is_compared_to_an_other_literal_node_then_only_their_first_chars_are_checked() {
        let alpha = LiteralNode::new("alpha");
        let beta = LiteralNode::new("beta");
        let aleph = LiteralNode::new("aleph");
        let empty = LiteralNode::new("");

        assert_eq!(alpha.cmp(&beta), Ordering::Less);
        assert_eq!(alpha.cmp(&aleph), Ordering::Equal);
        assert_eq!(beta.cmp(&alpha), Ordering::Greater);
        assert_eq!(alpha.cmp(&empty), Ordering::Greater);
        assert_eq!(empty.cmp(&alpha), Ordering::Less);
    }
}
