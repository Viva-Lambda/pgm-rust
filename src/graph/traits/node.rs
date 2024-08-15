// node trait

use crate::graph::traits::graph_obj::GraphObject;
use std::collections::HashMap;
use std::collections::HashSet;

/// Promotes anything that implements a [GraphObject] to being a [Node]
pub trait Node: GraphObject + Clone {
    /// factory function for node
    fn create(_: String, _: HashMap<String, Vec<String>>) -> Self;
}

/// Defines basic behaviour for containers of [Node] a very thin wrapper
/// around HashSet<Node>
pub trait VertexSet<N: Node> {
    /// access members of the container
    fn members(&self) -> HashSet<&N>;

    /// construct container from hash set
    fn create(_: HashSet<&N>) -> Self;
}
