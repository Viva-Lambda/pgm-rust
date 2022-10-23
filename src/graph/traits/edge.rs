// edge trait

use crate::graph::types::node::Node;

use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::types::edgetype::EdgeType;

pub trait Edge: GraphObject {
    fn start(&self) -> &Node;
    fn end(&self) -> &Node;
    fn has_type(&self) -> EdgeType;
}
