mod edge;
mod node;

pub use edge::*;
pub use node::*;

use std::{rc::Rc, sync::RwLock, vec};

/// Represents an unidirectional graph.
#[derive(Debug)]
pub struct Graph<D, E> {
    nodes: Vec<Rc<RwLock<GraphNode<D, E>>>>,
}

impl<D, E> Graph<D, E> {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    /// Adds a new node to the graph.
    pub fn add_node(&mut self, node_data: D) -> Rc<RwLock<GraphNode<D, E>>> {
        let node = GraphNode::new(node_data);
        let node = Rc::new(RwLock::new(node));
        self.nodes.push(node.clone());
        node
    }

    /// Iterates over each node of the graph.
    pub fn iter_nodes(&self) -> impl Iterator<Item = Rc<RwLock<GraphNode<D, E>>>> + '_ {
        self.nodes.iter().map(Clone::clone)
    }
}
