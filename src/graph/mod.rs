mod node;

pub(crate) use node::{Node, NodeId};

#[derive(Debug, Clone)]
pub(crate) struct Graph<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Graph<T> {
    pub(crate) fn new() -> Graph<T> {
        Graph { nodes: Vec::new() }
    }

    pub(super) fn push_node(&mut self, node: T) -> NodeId {
        let next_index = self.nodes.len();
        self.nodes.push(Node::new(node));
        NodeId::new(next_index)
    }
}
