//! A base graph which implements the Graph trait for doing graph theoretical
//! operations

use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::graph::Graph as GraphTrait;
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::types::edge::Edge;
use crate::graph::types::node::Node;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use uuid::Uuid;

use std::hash::{Hash, Hasher};

/// Basic graph type which implements the relative [trait](GraphTrait)
#[derive(Debug, Eq, PartialEq)]
pub struct Graph {
    /// graph identifier required for [GraphObject] trait
    graph_id: String,
    /// graph data required for [GraphObject] trait
    graph_data: HashMap<String, Vec<String>>,
    /// edges belonging to the graph
    edges: HashSet<Edge>,
    /// node belonging to the graph
    nodes: HashSet<Node>,
}

/// Graph objects are hashed using their identifiers
impl Hash for Graph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.graph_id.hash(state);
    }
}

/// Graph objects display their identifier when serialized to string.
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
    pub fn from_graphish_ref<T: GraphTrait>(g: &T) -> Graph {
        Graph {
            graph_id: g.id().clone(),
            graph_data: g.data().clone(),
            nodes: g.vertices().clone(),
            edges: g.edges().clone(),
        }
    }
    pub fn from_graphish<T: GraphTrait>(g: T) -> Graph {
        Graph {
            graph_id: g.id().clone(),
            graph_data: g.data().clone(),
            nodes: g.vertices().clone(),
            edges: g.edges().clone(),
        }
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
    pub fn from_edge_node_set(edges: HashSet<Edge>, nodes: HashSet<Node>) -> Graph {
        let mut mnodes = nodes;
        for edge in &edges {
            mnodes.insert(edge.start().clone());
            mnodes.insert(edge.end().clone());
        }
        Graph {
            graph_id: Uuid::new_v4().to_string(),
            graph_data: HashMap::new(),
            nodes: mnodes,
            edges,
        }
    }
    /// we filter `edges` based on `nodes` before initializing the [Graph]
    pub fn based_on_node_set(edges: HashSet<Edge>, nodes: HashSet<Node>) -> Graph {
        let mut medges = HashSet::new();
        for edge in &edges {
            let c1 = nodes.contains(edge.start());
            let c2 = nodes.contains(edge.end());
            if c1 && c2 {
                medges.insert(edge.clone());
            }
        }
        Graph {
            graph_id: Uuid::new_v4().to_string(),
            graph_data: HashMap::new(),
            nodes,
            edges: medges,
        }
    }
}
