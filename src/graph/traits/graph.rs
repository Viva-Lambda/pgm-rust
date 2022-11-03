// graph trait
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::types::edge::Edge;
use crate::graph::types::node::Node;
use std::collections::HashSet;

/// Promotes an object to being a graph.
/// This trait is the gateway for using all the graph related operations in
/// the library
pub trait Graph: GraphObject {
    /// outputs a [Node] set.
    /// a [Node] can constructed anything that implements the Node trait
    fn vertices(&self) -> HashSet<&Node>;

    /// outputs an [Edge] set.
    /// an [Edge] can constructed anything that implements the Edge trait
    fn edges(&self) -> HashSet<&Edge>;
}
//
