//! A base graph which implements the Graph trait for doing graph theoretical
//! operations

use crate::graph::traits::graph::Graph as GraphTrait;
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::types::edge::Edge;
use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::types::node::Node;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use uuid::Uuid;

use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq)]
struct Graph {
    /// graph identifier required for GraphObject trait
    graph_id: String,
    /// graph data required for GraphObject trait
    graph_data: HashMap<String, Vec<String>>,
    /// edges belonging to the graph
    edges: HashSet<Edge>,
    /// node belonging to the graph
    nodes: HashSet<Node>,
}

impl Hash for Graph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.graph_id.hash(state);
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nid = &self.graph_id;
        write!(f, "Graph[ id: {} ]", nid)
    }
}

impl GraphObject for Graph {
    fn id(&self) -> &String {
        &self.graph_id
    }

    fn data(&self) -> &HashMap<String, Vec<String>> {
        &self.graph_data
    }
}
impl GraphTrait for Graph {
    fn vertices(&self) -> &HashSet<Node> {
        &self.nodes
    }
    fn edges(&self) -> &HashSet<Edge> {
        &self.edges
    }
}

impl Graph {
    pub fn new(
        graph_id: String,
        nodes: HashSet<Node>,
        edges: HashSet<Edge>,
        graph_data: HashMap<String, Vec<String>>,
    ) -> Graph {
        Graph {
            graph_id,
            nodes,
            edges,
            graph_data,
        }
    }
    pub fn from_graph_ref<T: GraphTrait>(g: &T) -> Graph {
        Graph {
            graph_id: g.id().clone(),
            graph_data: g.data().clone(),
            nodes: g.vertices().clone(),
            edges: g.edges().clone(),
        }
    }
    pub fn from_graph<T: GraphTrait>(g: T) -> Graph {
        Graph {
            graph_id: g.id().clone(),
            graph_data: g.data().clone(),
            nodes: g.vertices().clone(),
            edges: g.edges().clone(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.vertices().is_empty()
    }
    pub fn from_edgeset(edges: HashSet<Edge>) -> Graph {
        let mut nodes = HashSet::new();
        for edge in &edges {
            nodes.insert(edge.start().clone());
            nodes.insert(edge.end().clone());
        }
        Graph {
            graph_id: Uuid::new_v4().to_string(),
            graph_data: HashMap::new(),
            nodes,
            edges,
        }
    }
}
