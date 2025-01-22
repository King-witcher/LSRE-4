use std::{rc::Rc, sync::RwLock};

use super::GraphNode;

/// Represents an unidirectional edge in the graph.
#[derive(Debug)]
pub struct GraphEdge<D, E> {
    pub data: E,
    pub pointer: Rc<RwLock<GraphNode<D, E>>>,
}

impl<D, E> GraphEdge<D, E> {
    pub fn new(data: E, pointer: Rc<RwLock<GraphNode<D, E>>>) -> Self {
        GraphEdge { data, pointer }
    }
}
