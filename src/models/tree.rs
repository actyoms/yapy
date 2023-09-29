use super::node::Node;
use super::tag::Tag;

pub(crate) struct Tree {
    pub(crate) tag: Tag,
    pub(crate) nodes: Vec<Node>,
}
