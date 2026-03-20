//! A base graph which implements the Graph trait for doing graph theoretical
//! operations

use crate::errors::{PGMRustError, PGMRustResult};
use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::graph::Graph as GraphTrait;
use crate::graph::traits::graph_obj::GraphObject as GraphObjectTrait;
use crate::graph::traits::node::Node as NodeTrait;
use crate::graph::traits::path::Path as PathTrait;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::marker::PhantomData;
use crate::graph::traits::generic::default_with_hash_partial_eq_impl;

use crate::graph::traits::generic::{ // required for main macro
    default_getter_impl, default_hash_id_impl,
    default_idchanger_impl, default_identified_impl, default_loadchanger_impl, default_loaded_impl,
    default_named_impl, default_partial_eq_impl, default_setter_impl,
};

use crate::graph::traits::generic::Identified;

/// Output nodes of the argument edges with different groupings
fn get_end_vertices_and_nodes<N, E>(edges: &HashSet<E>) -> PGMRustResult<(Vec<N>, HashSet<N>, (N, N))>
where
    N: NodeTrait,
    E: EdgeTrait<N>,
{
    if edges.is_empty() {
        return Err(PGMRustError::EmptyEdgeSet);
    }

    let mut degree_map: HashMap<N, usize> = HashMap::new();
    let mut node_set: HashSet<N> = HashSet::new();

    for e in edges {
        let u = e.start().clone();
        let v = e.end().clone();

        *degree_map.entry(u.clone()).or_insert(0) += 1;
        *degree_map.entry(v.clone()).or_insert(0) += 1;

        node_set.insert(u);
        node_set.insert(v);
    }

    // Find nodes with degree 1 (endpoints)
    let endpoints: Vec<N> = degree_map
        .into_iter()
        .filter(|(_, count)| *count == 1)
        .map(|(node, _)| node)
        .collect();

    if endpoints.len() != 2 {
        return Err(PGMRustError::NotASimplePath(endpoints.len()));
    }

    // For the return tuple (node_lst, node_set, (start, end))
    let node_lst: Vec<N> = node_set.iter().cloned().collect();
    let start_end = (endpoints[0].clone(), endpoints[1].clone());

    Ok((node_lst, node_set, start_end))
}

fn order_edges<N, E>(start_node: &N, edge_set: HashSet<E>) -> PGMRustResult<Vec<E>>
where
    N: NodeTrait + PartialEq + Clone,
    E: EdgeTrait<N> + Clone,
{
    let mut ordered = Vec::new();
    let mut remaining_edges: Vec<E> = edge_set.into_iter().collect();
    let mut current_node = start_node.clone();

    while !remaining_edges.is_empty() {
        // Find the index of the edge that connects to the current node
        if let Some(pos) = remaining_edges.iter().position(|e| *e.start() == current_node || *e.end() == current_node) {
            let edge = remaining_edges.remove(pos);

            // Determine the "next" node in the sequence
            current_node = if *edge.start() == current_node {
                edge.end().clone()
            } else {
                edge.start().clone()
            };

            ordered.push(edge);
        } else {
            return Err(PGMRustError::DisconnectedPath(remaining_edges.len()));
        }
    }
    Ok(ordered)
}

/// path is essentially a graph
/// path object as defined in Diestel 2017, p. 6
#[derive(Debug, Clone)]
pub struct Path<N: NodeTrait, E: EdgeTrait<N>> {
    /// edges of the path graph
    gdata: Vec<E>,
    /// graph identifier required for [GraphObject] trait
    _id: String,
    /// graph data required for [GraphObject] trait
    _data: HashMap<String, Vec<String>>,
    _node_type: PhantomData<N>,
}

default_with_hash_partial_eq_impl!(Path, <NodeType, EdgeType>, 
    NodeType: NodeTrait, EdgeType: EdgeTrait<NodeType>);


/// Path objects display their identifier when serialized to string.
impl<N: NodeTrait, E: EdgeTrait<N>> fmt::Display for Path<N, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nid = &self.id();
        write!(f, "<Path id='{}'>", nid)
    }
}

impl<T: NodeTrait, E: EdgeTrait<T>> GraphObjectTrait
    for Path<T, E>
{
    fn null() -> Path<T, E> {
        let idstr = String::from("");
        Path {
            _id: idstr,
            gdata: Vec::new(),
            _data: HashMap::new(),
            _node_type: PhantomData,
        }
    }

}

impl<T: NodeTrait, E: EdgeTrait<T> + Clone> GraphTrait<T, E>
    for Path<T, E>
{
    fn vertices(&self) -> HashSet<&T> {

    let mut nodes: HashSet<&T> = HashSet::new();

    for e in &self.gdata {
        let e_start: &T = e.start();
        let e_end: &T = e.end();
        nodes.insert(e_start);
        nodes.insert(e_end);
    }
    nodes
    }
    fn edges(&self) -> HashSet<&E> {
        self.gdata.iter().collect()
    }
    fn create(
        graph_id: String,
        graph_data: HashMap<String, Vec<String>>,
        _nodes: HashSet<T>,
        edges: HashSet<E>,
    ) -> PGMRustResult<Path<T, E>> {
        let (_, _, (start, _end)) = get_end_vertices_and_nodes::<T, E>(&edges)?;
        let ordered = order_edges::<T, E>(&start, edges)?;
        Ok(Path {
            _id: graph_id,
            _data: graph_data,
            gdata: ordered,
            _node_type: PhantomData,
        })
    }
    fn create_from_ref(
        graph_id: String,
        graph_data: HashMap<String, Vec<String>>,
        _nodes: HashSet<&T>,
        edges: HashSet<&E>,
    ) -> PGMRustResult<Path<T, E>> {
        let edges_: HashSet<E> = edges.iter().map(|&x| x.clone()).collect();
        let (_, _, (start, _end)) = get_end_vertices_and_nodes::<T, E>(&edges_)?;
        let ordered = order_edges::<T, E>(&start, edges_)?;
        Ok(Path {
            _id: graph_id,
            _data: graph_data,
            gdata: ordered,
            _node_type: PhantomData,
        })
    }
}

impl<T: NodeTrait, E: EdgeTrait<T> + Clone> PathTrait<T, E>
    for Path<T, E>
{
    /// number of edges inside the path, see Diestel 2017, p. 6
    fn length(&self) -> usize {
        self.gdata.len()
    }

    /// end nodes of path
    fn endvertices(&self) -> PGMRustResult<(&T, &T)> {
        if self.gdata.is_empty() {
            return Err(PGMRustError::EmptyPath);
        }

        if self.gdata.len() == 1 {
            // Path of length 1: the edge is (u, v)
            return Ok((self.gdata[0].start(), self.gdata[0].end()));
        }

        // For length > 1, find the nodes in the first and last edges
        // that are NOT shared with their neighbors.
        let e_first = &self.gdata[0];
        let e_second = &self.gdata[1];
        let e_last = &self.gdata[self.gdata.len() - 1];
        let e_penultimate = &self.gdata[self.gdata.len() - 2];

        // The start node is the one in e_first that isn't in e_second
        let start = if e_first.start() == e_second.start() || e_first.start() == e_second.end() {
            e_first.end()
        } else {
            e_first.start()
        };

        // The end node is the one in e_last that isn't in e_penultimate
        let end = if e_last.start() == e_penultimate.start() || e_last.start() == e_penultimate.end() {
            e_last.end()
        } else {
            e_last.start()
        };

        Ok((start, end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::traits::graph::Graph as GraphTrait;
    use crate::graph::traits::path::Path as PathTrait;
    use crate::graph::types::edge::Edge;
    use crate::graph::types::node::Node;

    fn node(id: &str) -> Node {
        Node::from_id(id)
    }

    fn uedge(id: &str, s: &str, e: &str) -> Edge<Node> {
        Edge::undirected(id.to_string(), node(s), node(e), HashMap::new())
    }

    // --- construction ---

    #[test]
    fn create_single_edge_path() {
        let e1 = uedge("e1", "n1", "n2");
        let mut edges = HashSet::new();
        edges.insert(e1);
        let p: Path<Node, Edge<Node>> =
            GraphTrait::create("p".to_string(), HashMap::new(), HashSet::new(), edges).unwrap();
        assert_eq!(p.length(), 1);
    }

    #[test]
    fn create_two_edge_path() {
        let e1 = uedge("e1", "n1", "n2");
        let e2 = uedge("e2", "n2", "n3");
        let mut edges = HashSet::new();
        edges.insert(e1);
        edges.insert(e2);
        let p: Path<Node, Edge<Node>> =
            GraphTrait::create("p".to_string(), HashMap::new(), HashSet::new(), edges).unwrap();
        assert_eq!(p.length(), 2);
    }

    #[test]
    fn create_three_edge_path() {
        let e1 = uedge("e1", "n1", "n2");
        let e2 = uedge("e2", "n2", "n3");
        let e3 = uedge("e3", "n3", "n4");
        let mut edges = HashSet::new();
        edges.insert(e1);
        edges.insert(e2);
        edges.insert(e3);
        let p: Path<Node, Edge<Node>> =
            GraphTrait::create("p".to_string(), HashMap::new(), HashSet::new(), edges).unwrap();
        assert_eq!(p.length(), 3);
    }

    #[test]
    fn create_from_ref_path() {
        let e1 = uedge("e1", "n1", "n2");
        let e2 = uedge("e2", "n2", "n3");
        let mut edge_refs = HashSet::new();
        edge_refs.insert(&e1);
        edge_refs.insert(&e2);
        let p: Path<Node, Edge<Node>> =
            GraphTrait::create_from_ref("p".to_string(), HashMap::new(), HashSet::new(), edge_refs).unwrap();
        assert_eq!(p.length(), 2);
    }

    // --- endvertices ---

    #[test]
    fn endvertices_single_edge() {
        let e1 = uedge("e1", "a", "b");
        let mut edges = HashSet::new();
        edges.insert(e1);
        let p: Path<Node, Edge<Node>> =
            GraphTrait::create("p".to_string(), HashMap::new(), HashSet::new(), edges).unwrap();
        let (start, end) = p.endvertices().unwrap();
        let ids: HashSet<&str> = [start.id(), end.id()].into();
        assert!(ids.contains("a"));
        assert!(ids.contains("b"));
    }

    #[test]
    fn endvertices_three_edge_path() {
        let e1 = uedge("e1", "n1", "n2");
        let e2 = uedge("e2", "n2", "n3");
        let e3 = uedge("e3", "n3", "n4");
        let mut edges = HashSet::new();
        edges.insert(e1);
        edges.insert(e2);
        edges.insert(e3);
        let p: Path<Node, Edge<Node>> =
            GraphTrait::create("p".to_string(), HashMap::new(), HashSet::new(), edges).unwrap();
        let (start, end) = p.endvertices().unwrap();
        let ids: HashSet<&str> = [start.id(), end.id()].into();
        assert!(ids.contains("n1"));
        assert!(ids.contains("n4"));
    }

    // --- vertices ---

    #[test]
    fn vertices_contains_all_nodes() {
        let e1 = uedge("e1", "n1", "n2");
        let e2 = uedge("e2", "n2", "n3");
        let mut edges = HashSet::new();
        edges.insert(e1);
        edges.insert(e2);
        let p: Path<Node, Edge<Node>> =
            GraphTrait::create("p".to_string(), HashMap::new(), HashSet::new(), edges).unwrap();
        let vs: HashSet<&str> = p.vertices().iter().map(|n| n.id()).collect();
        assert_eq!(vs.len(), 3);
        assert!(vs.contains("n1"));
        assert!(vs.contains("n2"));
        assert!(vs.contains("n3"));
    }

    // --- edges ---

    #[test]
    fn edges_count_matches_length() {
        let e1 = uedge("e1", "n1", "n2");
        let e2 = uedge("e2", "n2", "n3");
        let e3 = uedge("e3", "n3", "n4");
        let mut edges = HashSet::new();
        edges.insert(e1);
        edges.insert(e2);
        edges.insert(e3);
        let p: Path<Node, Edge<Node>> =
            GraphTrait::create("p".to_string(), HashMap::new(), HashSet::new(), edges).unwrap();
        assert_eq!(p.edges().len(), p.length());
    }

    // --- error cases ---

    #[test]
    fn create_errors_on_empty_edge_set() {
        use crate::errors::PGMRustError;
        let result: Result<Path<Node, Edge<Node>>, _> =
            GraphTrait::create("p".to_string(), HashMap::new(), HashSet::new(), HashSet::new());
        assert!(matches!(result, Err(PGMRustError::EmptyEdgeSet)));
    }

    #[test]
    fn endvertices_errors_on_null_path() {
        use crate::errors::PGMRustError;
        let p: Path<Node, Edge<Node>> = GraphObjectTrait::null();
        assert!(matches!(p.endvertices(), Err(PGMRustError::EmptyPath)));
    }

    /// A valid path (n1-n2-n3) combined with a disconnected cycle (n4-n5-n4)
    /// has exactly 2 degree-1 nodes (n1 and n3), so it passes the endpoint check,
    /// but the cycle is unreachable from n1 and must be caught by order_edges.
    #[test]
    fn create_errors_on_disconnected_edge_set() {
        use crate::errors::PGMRustError;
        let mut edges = HashSet::new();
        edges.insert(uedge("e1", "n1", "n2"));
        edges.insert(uedge("e2", "n2", "n3"));
        // isolated cycle: n4-n5, n5-n4 — all degree-2, so endpoint count stays at 2
        edges.insert(uedge("e3", "n4", "n5"));
        edges.insert(uedge("e4", "n5", "n4"));
        let result: Result<Path<Node, Edge<Node>>, _> =
            GraphTrait::create("p".to_string(), HashMap::new(), HashSet::new(), edges);
        assert!(matches!(result, Err(PGMRustError::DisconnectedPath(_))));
    }
}
