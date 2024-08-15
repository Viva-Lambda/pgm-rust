// edge trait

use crate::graph::traits::node::Node;

use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::types::edgetype::EdgeType;
use std::collections::HashMap;
use std::collections::HashSet;

/// Promotes anything that implements [GraphObject] trait to [Edge]
pub trait Edge<NodeType: Node>: GraphObject {
    // type NodeType: impl Node;
    /// start node of the edge
    fn start(&self) -> &NodeType;
    /// end node of the edge
    fn end(&self) -> &NodeType;
    /// type [Directed], [Undirected] of the edge
    fn has_type(&self) -> &EdgeType;

    /// constructor for the edge trait
    fn create(
        _: String,
        _: HashMap<String, Vec<String>>,
        _: NodeType,
        _: NodeType,
        _: EdgeType,
    ) -> Self;
}

/// Defines basic behaviour for containers of [Edge] a very thin wrapper
/// around HashSet<Edge>
pub trait EdgeSet<N: Node, E: Edge<N>> {
    /// access members of the container
    fn members(&self) -> HashSet<&E>;

    /// construct container from hash set
    fn create(_: HashSet<&E>) -> Self;
}
