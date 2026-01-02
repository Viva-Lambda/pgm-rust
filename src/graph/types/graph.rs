//! A base graph which implements the Graph trait for doing graph theoretical
//! operations

use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::graph::Graph as GraphTrait;
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::traits::node::Node as NodeTrait;
use crate::graph::traits::utils::{from_borrowed_data, to_borrowed_data};

use crate::graph::traits::generic::default_with_hash_partial_eq_impl;

use crate::graph::traits::generic::{
    default_display_with_data_impl, default_getter_impl, default_hash_id_impl,
    default_idchanger_impl, default_identified_impl, default_loadchanger_impl, default_loaded_impl,
    default_named_impl, default_partial_eq_impl, default_setter_impl,
};

use crate::graph::traits::generic::{
    render_hashmap, IdChanger, Identified, LoadChanger, Loaded, Named,
};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use uuid::Uuid;

use std::hash::{Hash, Hasher};

/// Basic graph type which implements the relative [trait](GraphTrait)
/// Formally defined as a set with two members which are also sets,
/// see Diestel 2017, p. 2
#[derive(Debug, Clone)]
pub struct Graph<NodeType: NodeTrait, EdgeType: EdgeTrait<NodeType>> {
    /// graph identifier required for [GraphObject] trait
    _id: String,
    /// graph data required for [GraphObject] trait
    _data: HashMap<String, Vec<String>>,
    /// internal representation of graph data
    /// node set contains nodes that are not connected to any edges
    /// edge set contains edges
    gdata: (HashSet<NodeType>, HashSet<EdgeType>),
}

default_with_hash_partial_eq_impl!(Graph, <NodeType, EdgeType>, 
    NodeType: NodeTrait, EdgeType: EdgeTrait<NodeType>);

/// Graph objects display their identifier when serialized to string.
impl<T: NodeTrait, E: EdgeTrait<T>> fmt::Display for Graph<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nid = &self.id();
        write!(f, "<Graph id='{}'/>", nid)
    }
}



impl<T: NodeTrait, E: EdgeTrait<T> + Clone> GraphTrait<T, E> for Graph<T, E> {
    fn vertices(&self) -> HashSet<&T> {
        let mut hset: HashSet<&T> = HashSet::new();
        let (ns, es) = &self.gdata;
        for e in es {
            hset.insert(e.start());
            hset.insert(e.end());
        }
        for n in ns {
            hset.insert(n);
        }
        hset
    }
    fn edges(&self) -> HashSet<&E> {
        let mut hset: HashSet<&E> = HashSet::new();
        let (_, es) = &self.gdata;
        for e in es {
            hset.insert(e);
        }
        hset
    }
    fn create(
        graph_id: String,
        graph_data: HashMap<String, Vec<String>>,
        nodes: HashSet<T>,
        edges: HashSet<E>,
    ) -> Graph<T, E> {
        Graph::new(graph_id, graph_data, nodes, edges)
    }
    fn create_from_ref(
        graph_id: String,
        graph_data: HashMap<String, Vec<String>>,
        nodes: HashSet<&T>,
        edges: HashSet<&E>,
    ) -> Graph<T, E> {
        Graph::new_refs(graph_id, graph_data, nodes, edges)
    }
}

fn get_vertices<T: NodeTrait, E: EdgeTrait<T>>(
    nodes: HashSet<T>,
    edges: HashSet<E>,
) -> (HashSet<E>, HashSet<T>) {
    let mut nset: HashSet<&T> = HashSet::new();
    for e in &edges {
        nset.insert(e.start());
        nset.insert(e.end());
    }
    let mut mset: HashSet<T> = HashSet::new();
    for n in nodes {
        if nset.contains(&n) == false {
            mset.insert(n);
        }
    }
    (edges, mset)
}

fn get_vertices_from_refset<T: NodeTrait, E: EdgeTrait<T> + Clone>(
    nodes: HashSet<&T>,
    edges: HashSet<&E>,
) -> (HashSet<E>, HashSet<T>) {
    let mut nset: HashSet<&T> = HashSet::new();
    let mut eset: HashSet<E> = HashSet::new();
    for e in edges {
        nset.insert(e.start());
        nset.insert(e.end());
        eset.insert(e.clone());
    }
    let mut mset: HashSet<T> = HashSet::new();
    for n in nodes {
        if nset.contains(n) == false {
            mset.insert(n.clone());
        }
    }
    (eset, mset)
}

impl<T: NodeTrait, E: EdgeTrait<T> + Clone> Graph<T, E> {
    /// constructor for the [Graph] object
    pub fn new(
        graph_id: String,
        graph_data: HashMap<String, Vec<String>>,
        nodes: HashSet<T>,
        edges: HashSet<E>,
    ) -> Graph<T, E> {
        let (edges, mset) = get_vertices(nodes, edges);
        Graph {
            _id: graph_id,
            _data: graph_data,
            gdata: (mset, edges),
        }
    }
    /// constructor for the [Graph] object
    pub fn new_refs(
        graph_id: String,
        graph_data: HashMap<String, Vec<String>>,
        nodes: HashSet<&T>,
        edges: HashSet<&E>,
    ) -> Graph<T, E> {
        let (edges, mset) = get_vertices_from_refset(nodes, edges);
        Graph {
            _id: graph_id,
            _data: graph_data,
            gdata: (mset, edges),
        }
    }
    /// empty constructor.
    /// Creates an empty graph that has no edge and vertex.
    pub fn empty(graph_id: &str) -> Graph<T, E> {
        let mut g = Graph::null().set_id(graph_id);
        g
    }
    /// construct [Graph] from graph like object with borrowing
    pub fn from_graphish_ref<G: GraphTrait<T, E>>(g: &G) -> Graph<T, E> {
        let (edges, mset) = get_vertices_from_refset(g.vertices(), g.edges());
        Graph {
            _id: g.id().to_string(), // Changed: Use to_string() instead of clone()
            _data: from_borrowed_data(&g.data()), // Changed: Convert borrowed data to owned
            gdata: (mset, edges),
        }
    }
    /// construct [Graph] from graph like object with move
    pub fn from_graphish<G: GraphTrait<T, E>>(g: G) -> Graph<T, E> {
        let (edges, mset) = get_vertices_from_refset(g.vertices(), g.edges());
        Graph {
            _id: g.id().to_string(),
            _data: from_borrowed_data(&g.data()), // Changed: Convert borrowed data to owned
            gdata: (mset, edges),
        }
    }
    /// construct [Graph] from [Edge] set
    pub fn from_edgeset(edges: HashSet<E>) -> Graph<T, E> {
        Graph {
            _id: Uuid::new_v4().to_string(),
            _data: HashMap::new(),
            gdata: (HashSet::new(), edges),
        }
    }
    /// construct [Graph] from [Edge] and [Node] sets.
    pub fn from_edge_node_set(edges: HashSet<E>, nodes: HashSet<T>) -> Graph<T, E> {
        let (es, mset) = get_vertices(nodes, edges);
        Graph {
            _id: Uuid::new_v4().to_string(),
            _data: HashMap::new(),
            gdata: (mset, es),
        }
    }
    /// construct [Graph] from [Edge] and [Node] reference sets
    pub fn from_edge_node_refs_set(edges: HashSet<&E>, nodes: HashSet<&T>) -> Graph<T, E> {
        let (es, mset) = get_vertices_from_refset(nodes, edges);
        Graph {
            _id: Uuid::new_v4().to_string(),
            _data: HashMap::new(),
            gdata: (mset, es),
        }
    }
    /// construct [Graph] from [Edge] and [Node] sets.
    /// we filter `edges` based on `nodes` before initializing the [Graph].
    pub fn based_on_node_set(edges: HashSet<E>, nodes: HashSet<T>) -> Graph<T, E> {
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
            _id: Uuid::new_v4().to_string(),
            _data: HashMap::new(),
            gdata: (mset, es),
        }
    }
}
// Changed: Added missing Clone bound to E for GraphObject implementation
impl<T: NodeTrait, E: EdgeTrait<T> + Clone> GraphObject for Graph<T, E> {
    fn null() -> Graph<T, E> {
        let idstr = String::from("");
        Graph {
            _id: idstr,
            gdata: (HashSet::new(), HashSet::new()),
            _data: HashMap::new(),
        }
    }
}
#[cfg(test)]
mod tests {
}
