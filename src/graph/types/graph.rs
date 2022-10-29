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
/// Formally defined as a set with two members which are also sets,
/// see Diestel 2017, p. 2
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
    /// constructor for the [Graph] object
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
    /// empty constructor.
    /// Creates an empty graph that has no edge and vertex.
    pub fn empty(graph_id: &str) -> Graph {
        Graph {
            graph_id: graph_id.to_string(),
            edges: HashSet::new(),
            nodes: HashSet::new(),
            graph_data: HashMap::new(),
        }
    }
    /// construct [Graph] from graph like object with borrowing
    pub fn from_graphish_ref<T: GraphTrait>(g: &T) -> Graph {
        Graph {
            graph_id: g.id().clone(),
            graph_data: g.data().clone(),
            nodes: g.vertices().clone(),
            edges: g.edges().clone(),
        }
    }
    /// construct [Graph] from graph like object with move
    pub fn from_graphish<T: GraphTrait>(g: T) -> Graph {
        Graph {
            graph_id: g.id().clone(),
            graph_data: g.data().clone(),
            nodes: g.vertices().clone(),
            edges: g.edges().clone(),
        }
    }
    /// construct [Graph] from [Edge] set
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
    /// construct [Graph] from [Edge] and [Node] sets.
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
    /// construct [Graph] from [Edge] and [Node] sets.
    /// we filter `edges` based on `nodes` before initializing the [Graph].
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

#[cfg(test)]
mod tests {

    use super::*; // brings in the parent scope to current module scope

    // mk node
    fn mk_node(n_id: &str) -> Node {
        Node::new(n_id.to_string(), HashMap::new())
    }

    // mk edge
    fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge {
        let n1 = mk_node(n1_id);
        let n2 = mk_node(n2_id);
        let mut h1 = HashMap::new();
        h1.insert(String::from("my"), vec![String::from("data")]);
        Edge::undirected(e_id.to_string(), n1, n2, h1)
    }

    // make graph
    fn mk_g(g_id: &str) -> Graph {
        let mut nodes = HashSet::new();
        let mut edges = HashSet::new();
        nodes.insert(mk_node("n1"));
        nodes.insert(mk_node("n2"));
        nodes.insert(mk_node("n3"));
        nodes.insert(mk_node("n4"));
        edges.insert(mk_uedge("n1", "n2", "e1"));
        edges.insert(mk_uedge("n2", "n3", "e2"));
        Graph::new(g_id.to_string(), nodes, edges, HashMap::new())
    }

    //
    #[test]
    fn test_vertices() {
        let g = mk_g("g1");
        let vs = g.vertices();
        //
        let mut nodes = HashSet::new();
        nodes.insert(mk_node("n1"));
        nodes.insert(mk_node("n2"));
        nodes.insert(mk_node("n3"));
        nodes.insert(mk_node("n4"));
        //
        assert_eq!(&nodes, vs);
    }

    #[test]
    fn test_edges() {
        let g = mk_g("g1");
        let es = g.edges();
        //
        let mut edges = HashSet::new();
        edges.insert(mk_uedge("n1", "n2", "e1"));
        edges.insert(mk_uedge("n2", "n3", "e2"));
        //
        assert_eq!(&edges, es);
    }
    #[test]
    fn test_from_graphish_ref() {
        let g1 = mk_g("g1");
        let g2 = mk_g("g1");
        let g = Graph::from_graphish_ref(&g1);
        assert_eq!(g, g2);
    }

    #[test]
    fn test_from_graphish() {
        let g1 = mk_g("g1");
        let g2 = mk_g("g1");
        let g = Graph::from_graphish(g1);
        assert_eq!(g, g2);
    }

    #[test]
    fn test_from_edgeset() {
        let mut edges = HashSet::new();
        edges.insert(mk_uedge("n1", "n2", "e1"));
        edges.insert(mk_uedge("n2", "n3", "e2"));
        let g1 = Graph::from_edgeset(edges.clone());

        let mut nodes = HashSet::new();
        nodes.insert(mk_node("n1"));
        nodes.insert(mk_node("n2"));
        nodes.insert(mk_node("n3"));
        assert_eq!(g1.vertices(), &nodes);
        assert_eq!(g1.edges(), &edges);
    }

    #[test]
    fn test_from_edge_node_set() {
        let mut nodes = HashSet::new();
        let mut edges = HashSet::new();
        nodes.insert(mk_node("n1"));
        nodes.insert(mk_node("n2"));
        nodes.insert(mk_node("n3"));
        nodes.insert(mk_node("n4"));
        edges.insert(mk_uedge("n1", "n2", "e1"));
        edges.insert(mk_uedge("n2", "n3", "e2"));
        let g = Graph::from_edge_node_set(edges.clone(), nodes.clone());
        assert_eq!(g.vertices(), &nodes);
        assert_eq!(g.edges(), &edges);
    }

    #[test]
    fn test_based_on_node_set() {
        let mut nodes = HashSet::new();
        let mut edges = HashSet::new();
        nodes.insert(mk_node("n2"));
        nodes.insert(mk_node("n3"));
        edges.insert(mk_uedge("n1", "n2", "e1"));
        edges.insert(mk_uedge("n2", "n3", "e2"));
        let g = Graph::based_on_node_set(edges.clone(), nodes.clone());
        assert_eq!(g.vertices(), &nodes);
        let mut es = HashSet::new();
        es.insert(mk_uedge("n2", "n3", "e2"));
        assert_eq!(g.edges(), &es);
    }
}
