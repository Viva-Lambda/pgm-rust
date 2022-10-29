// edge trait

use crate::graph::types::node::Node;

use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::types::edgetype::EdgeType;

/// Promotes anything that implements [GraphObject] trait to [Edge]
pub trait Edge: GraphObject {
    /// start node of the edge
    fn start(&self) -> &Node;
    /// end node of the edge
    fn end(&self) -> &Node;
    /// type [Directed], [Undirected] of the edge
    fn has_type(&self) -> EdgeType;
}
