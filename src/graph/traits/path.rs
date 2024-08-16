// path trait
use crate::graph::traits::edge::Edge;
use crate::graph::traits::graph::Graph;
use crate::graph::traits::node::Node;

/// a path
pub trait Path<N: Node, E: Edge<N>>: Graph<N, E> {
    /// number of edges inside the path, see Diestel 2017, p. 6
    fn length(&self) -> usize;

    /// end nodes of path
    fn endvertices(&self) -> (&N, &N);
}
