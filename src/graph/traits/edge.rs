// edge trait

use crate::graph::traits::node::Node;

use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::types::edgetype::EdgeType;
use std::collections::HashMap;

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
