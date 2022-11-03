//! A base graph which implements the Graph trait for doing graph theoretical
//! operations

use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::graph::Graph as GraphTrait;
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::types::edge::Edge;
use crate::graph::types::edge::EdgeInfo;
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
    /// internal representation of graph data
    /// node set contains nodes that are not connected to any edges
    /// edge set contains edges
    gdata: (HashSet<Edge>, HashSet<Node>),
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
fn get_vertices(nodes: HashSet<Node>, edges: HashSet<Edge>) -> (HashSet<Edge>, HashSet<Node>) {
    let mut nset: HashSet<&Node> = HashSet::new();
    for e in &edges {
        nset.insert(e.start());
        nset.insert(e.end());
    }
    let mut mset: HashSet<Node> = HashSet::new();
    for n in nodes {
        if nset.contains(&n) == false {
            mset.insert(n);
        }
    }
    (edges, mset)
}

fn get_vertices_from_refset(
    nodes: HashSet<&Node>,
    edges: HashSet<&Edge>,
) -> (HashSet<Edge>, HashSet<Node>) {
    let mut nset: HashSet<&Node> = HashSet::new();
    let mut eset: HashSet<Edge> = HashSet::new();
    for e in edges {
        nset.insert(e.start());
        nset.insert(e.end());
        eset.insert(e.clone());
    }
    let mut mset: HashSet<Node> = HashSet::new();
    for n in nodes {
        if nset.contains(n) == false {
            mset.insert(n.clone());
        }
    }
    (eset, mset)
}

impl Graph {
    /// constructor for the [Graph] object
    pub fn new(
        graph_id: String,
        nodes: HashSet<Node>,
        edges: HashSet<Edge>,
        graph_data: HashMap<String, Vec<String>>,
    ) -> Graph {
        let (edges, mset) = get_vertices(nodes, edges);
        Graph {
            graph_id,
            gdata: (edges, mset),
            graph_data,
        }
    }
    /// empty constructor.
    /// Creates an empty graph that has no edge and vertex.
    pub fn empty(graph_id: &str) -> Graph {
        Graph {
            graph_id: graph_id.to_string(),
            gdata: (HashSet::new(), HashSet::new()),
            graph_data: HashMap::new(),
        }
    }
    /// construct [Graph] from graph like object with borrowing
    pub fn from_graphish_ref<T: GraphTrait>(g: &T) -> Graph {
        let (edges, mset) = get_vertices_from_refset(g.vertices(), g.edges());
        Graph {
            graph_id: g.id().clone(),
            graph_data: g.data().clone(),
            gdata: (edges, mset),
        }
    }
    /// construct [Graph] from graph like object with move
    pub fn from_graphish<T: GraphTrait>(g: T) -> Graph {
        let (edges, mset) = get_vertices_from_refset(g.vertices(), g.edges());
        Graph {
            graph_id: g.id().to_string(),
            graph_data: g.data().clone(),
            gdata: (edges, mset),
        }
    }
    /// construct [Graph] from [Edge] set
    pub fn from_edgeset(edges: HashSet<Edge>) -> Graph {
        Graph {
            graph_id: Uuid::new_v4().to_string(),
            graph_data: HashMap::new(),
            gdata: (edges, HashSet::new()),
        }
    }
    /// construct [Graph] from [Edge] and [Node] sets.
    pub fn from_edge_node_set(edges: HashSet<Edge>, nodes: HashSet<Node>) -> Graph {
        let (es, mset) = get_vertices(nodes, edges);
        Graph {
            graph_id: Uuid::new_v4().to_string(),
            graph_data: HashMap::new(),
            gdata: (es, mset),
        }
    }
    /// construct [Graph] from [Edge] and [Node] sets.
    /// we filter `edges` based on `nodes` before initializing the [Graph].
    pub fn based_on_node_set(edges: HashSet<Edge>, nodes: HashSet<Node>) -> Graph {
        let mut medges = HashSet::new();
        for edge in edges {
            let c1 = nodes.contains(edge.start());
            let c2 = nodes.contains(edge.end());
            if c1 && c2 {
                medges.insert(edge.clone());
            }
        }
        let (es, mset) = get_vertices(nodes, medges);

        Graph {
            graph_id: Uuid::new_v4().to_string(),
            graph_data: HashMap::new(),
            gdata: (es, mset),
        }
    }
}
impl GraphTrait for Graph {
    fn vertices(&self) -> HashSet<&Node> {
        let mut hset: HashSet<&Node> = HashSet::new();
        let (es, ns) = &self.gdata;
        for e in es {
            hset.insert(e.start());
            hset.insert(e.end());
        }
        for n in ns {
            hset.insert(n);
        }
        hset
    }
    fn edges(&self) -> HashSet<&Edge> {
        let mut hset: HashSet<&Edge> = HashSet::new();
        let (es, _) = &self.gdata;
        for e in es {
            hset.insert(e);
        }
        hset
    }
}

#[cfg(test)]
mod tests {

    use super::*; // brings in the parent scope to current module scope

    // mk node
    fn mk_node(n_id: &str) -> Node {
        Node::new(n_id.to_string(), HashMap::new())
    }

    fn mk_nodes(ns: Vec<&str>) -> HashSet<Node> {
        let mut h: HashSet<Node> = HashSet::new();
        for n in ns {
            h.insert(mk_node(n));
        }
        h
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
        let nodes = mk_nodes(vec!["n1", "n2", "n3", "n4"]);
        let mut edges = HashSet::new();
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
        let ns = mk_nodes(vec!["n1", "n2", "n3", "n4"]);
        for n in &ns {
            nodes.insert(n);
        }
        //
        assert_eq!(nodes, vs);
    }

    #[test]
    fn test_edges() {
        let g = mk_g("g1");
        let es = g.edges();
        //
        let mut edges = HashSet::new();
        let e1 = mk_uedge("n1", "n2", "e1");
        let e2 = mk_uedge("n2", "n3", "e2");
        edges.insert(&e1);
        edges.insert(&e2);
        //
        assert_eq!(edges, es);
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
        let mut edges: HashSet<&Edge> = HashSet::new();
        let e1 = mk_uedge("n1", "n2", "e1");
        let e2 = mk_uedge("n2", "n3", "e2");
        edges.insert(&e1);
        edges.insert(&e2);

        let mut nodes = HashSet::new();
        let n1 = mk_node("n1");
        let n2 = mk_node("n2");
        let n3 = mk_node("n3");
        nodes.insert(&n1);
        nodes.insert(&n2);
        nodes.insert(&n3);
        assert_eq!(g1.vertices(), nodes);
        assert_eq!(g1.edges(), edges);
    }

    #[test]
    fn test_from_edge_node_set() {
        let mut nodes: HashSet<&Node> = HashSet::new();
        let mut edges = HashSet::new();
        let ns = mk_nodes(vec!["n1", "n2", "n3", "n4"]);
        for n in &ns {
            nodes.insert(n);
        }
        let e1 = mk_uedge("n1", "n2", "e1");
        let e2 = mk_uedge("n2", "n3", "e2");
        edges.insert(&e1);
        edges.insert(&e2);
        let gedges: HashSet<Edge> = HashSet::from([e1.clone(), e2.clone()]);

        let gnodes: HashSet<Node> = ns.clone();

        let g = Graph::from_edge_node_set(gedges, gnodes);
        assert_eq!(g.vertices(), nodes);
        assert_eq!(g.edges(), edges);
    }

    #[test]
    fn test_based_on_node_set() {
        let nodes = mk_nodes(vec!["n2", "n3"]);
        let mut edges = HashSet::new();
        let mut mnodes: HashSet<&Node> = HashSet::new();
        for n in &nodes {
            mnodes.insert(n);
        }
        edges.insert(mk_uedge("n1", "n2", "e1"));
        edges.insert(mk_uedge("n2", "n3", "e2"));
        let g = Graph::based_on_node_set(edges, nodes.clone());
        assert_eq!(g.vertices(), mnodes);
        let mut es = HashSet::new();
        let e1 = mk_uedge("n2", "n3", "e2");
        es.insert(&e1);
        assert_eq!(g.edges(), es);
    }
}
