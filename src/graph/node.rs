use std::ops::{Deref, DerefMut};

use super::GraphEdge;

/// Represents a node from the graph.
#[derive(Debug)]
pub struct GraphNode<D, E> {
    data: D,
    edges: Vec<GraphEdge<D, E>>,
}

impl<D, E> GraphNode<D, E> {
    pub fn new(data: D) -> Self {
        Self {
            data,
            edges: vec![],
        }
    }

    /// Returns an iterator over each edge of the node.
    pub fn iter_edges(&self) -> impl Iterator<Item = &GraphEdge<D, E>> + '_ {
        self.edges.iter()
    }

    /// Adds a new edge to the node.
    pub fn add_edge(&mut self, edge: GraphEdge<D, E>) {
        self.edges.push(edge);
    }

    /// Gets how many edges the node has.
    pub fn edges_count(&self) -> usize {
        self.edges.len()
    }
}

impl<D, E> Deref for GraphNode<D, E> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<D, E> DerefMut for GraphNode<D, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
