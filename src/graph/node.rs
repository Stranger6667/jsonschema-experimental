use std::num::NonZeroUsize;

/// `NodeId` is a unique identifier for each `Node` in a graph.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct NodeId(NonZeroUsize);

impl NodeId {
    #[inline]
    pub(super) fn new(value: usize) -> NodeId {
        NodeId(NonZeroUsize::new(value).expect("Value is zero"))
    }
    #[inline]
    pub(super) fn get(self) -> usize {
        self.0.get()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Node<T> {
    pub(crate) parent: Option<NodeId>,
    pub(crate) next_sibling: Option<NodeId>,
    pub(crate) previous_sibling: Option<NodeId>,
    pub(crate) first_child: Option<NodeId>,
    pub(crate) last_child: Option<NodeId>,
    value: T,
}

impl<T> Node<T> {
    #[inline]
    pub(crate) fn new(value: T) -> Node<T> {
        Node {
            parent: None,
            previous_sibling: None,
            next_sibling: None,
            first_child: None,
            last_child: None,
            value,
        }
    }
}
