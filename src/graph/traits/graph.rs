// graph trait
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::types::edge::Edge;
use crate::graph::types::node::Node;
use std::collections::HashSet;

pub trait Graph: GraphObject {
    fn vertices(&self) -> &HashSet<Node>;
    fn edges(&self) -> &HashSet<Edge>;
}
//
