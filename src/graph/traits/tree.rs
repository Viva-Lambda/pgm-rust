// tree trait
use crate::graph::traits::edge::Edge;
use crate::graph::traits::graph::Graph;
use crate::graph::traits::node::Node;
use std::collections::HashSet;

/// a tree
pub trait Tree<N: Node, E: Edge<N>>: Graph<N, E> {
    /// From Diestel 2017, p. 15
    /// is x up closure of y
    fn is_upclosure_of(&self, x_src: &N, y_dst: &N) -> bool;

    /// From Diestel 2017, p. 15
    /// is x down closure of y
    fn is_downclosure_of(&self, x_src: &N, y_dst: &N) -> bool;

    /// From Diestel 2017, p. 15
    fn upset_of(&self, x_src: &N) -> HashSet<&N>;

    /// From Diestel 2017, p. 15
    fn downset_of(&self, x_src: &N) -> HashSet<&N>;

    /// root of tree
    fn root(&self) -> &N;

    /// leaves of tree
    fn leaves(&self) -> HashSet<&N>;

    /// height of node
    fn height_of(&self, n: &N) -> i32;

    // nodes per level in python
    /// extract nodes based on height
    fn nodes_per_height(&self, height: i32) -> HashSet<&N>;

    /// <= comparison for nodes of tree
    fn less_than_or_equal(&self, first: &N, second: &N) -> bool;

    /// >= comparison for nodes of tree
    fn greater_than_or_equal(&self, first: &N, second: &N) -> bool;
}
