// base graph module

use crate::graph::graphtype::edge::Edge;
use crate::graph::graphtype::node::Node;
use crate::graph::graphtype::obj::GraphObject;
use std::collections::HashSet;

use std::hash::{Hash, Hasher};

#[derive(Debug, Hash, Eq, PartialEq)]
struct BaseGraph {
    graph_id: String,
    graph_data: HashMap<String, Vec<String>>,
    edges: HashSet<Edge>,
    nodes: HashSet<Node>,
}

impl Hash for BaseGraph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.graph_id.hash(state);
    }
}

impl GraphObject for BaseGraph {
    fn id(&self) -> String {
        self.edge_id.clone()
    }

    fn data(&self) -> &HashMap<String, Vec<String>> {
        &self.graph_data
    }
}

impl BaseGraph {
}
