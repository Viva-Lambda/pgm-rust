//! A base graph which implements the Graph trait for doing graph theoretical
//! operations

use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::edge::EdgeSet as EdgeSetTrait;
use crate::graph::traits::utils::{from_borrowed_data, to_borrowed_data};
use crate::graph::traits::graph::Graph as GraphTrait;
use crate::graph::traits::graph_obj::GraphObject as GraphObjectTrait;
use crate::graph::traits::node::Node as NodeTrait;
use crate::graph::traits::node::VertexSet as VertexSetTrait;
use crate::graph::traits::path::Path as PathTrait;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use crate::graph::traits::generic::default_with_hash_partial_eq_impl;

use crate::graph::traits::generic::{ // required for main macro
    default_getter_impl, default_hash_id_impl,
    default_idchanger_impl, default_identified_impl, default_loadchanger_impl, default_loaded_impl,
    default_named_impl, default_partial_eq_impl, default_setter_impl,
};

use crate::graph::traits::generic::{
    IdChanger, Identified, Loaded,
};

/// checks if containers has two members or less
fn has_two_or_less<N: NodeTrait>(nodes: &Vec<&N>) {
    let c1 = nodes.len() <= 2;
    if !c1 {
        panic!("nodes have more than 2 elements")
    } else {
    }
}

fn extract_two_nodes<N: NodeTrait>(nodes: &Vec<&N>) -> (N, N) {
    let arr: [&N; 2] = nodes[0..2].try_into().unwrap();
    (arr[0].clone(), arr[1].clone())
}

/// Output nodes of the argument edges with different groupings
fn get_end_vertices_and_nodes<N, E>(edges: &HashSet<E>) -> (Vec<N>, HashSet<N>, (N, N))
where
    N: NodeTrait + std::hash::Hash + Eq + Clone,
    E: EdgeTrait<N>,
{
    if edges.is_empty() { panic!("Empty edge set"); }

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
        panic!("The provided edges do not form a simple path (endpoints found: {})", endpoints.len());
    }

    // For the return tuple (node_lst, node_set, (start, end))
    let node_lst: Vec<N> = node_set.iter().cloned().collect();
    let start_end = (endpoints[0].clone(), endpoints[1].clone());

    (node_lst, node_set, start_end)
}

fn order_edges<N, E>(start_node: &N, edge_set: HashSet<E>) -> Vec<E>
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
            break; // Should not happen if degree check passed and graph is connected
        }
    }
    ordered
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
        nodes: HashSet<T>,
        edges: HashSet<E>,
    ) -> Path<T, E> {
        let group = get_end_vertices_and_nodes::<T, E>(&edges);
        let (_, _, (start, end)) = group;
        let ordered = order_edges::<T, E>(&start, edges);
        Path {
            _id: graph_id,
            _data: graph_data,
            gdata: ordered
            ,_node_type: PhantomData,
        }
    }
    fn create_from_ref(
        graph_id: String,
        graph_data: HashMap<String, Vec<String>>,
        nodes: HashSet<&T>,
        edges: HashSet<&E>,
    ) -> Path<T, E> {
        let edges_: HashSet<E> = edges.iter().map(|&x|x.clone()).collect(); 
        let group = get_end_vertices_and_nodes::<T, E>(&edges_);
        let (_, _, (start, end)) = group;
        let ordered = order_edges::<T, E>(&start, edges_);
        Path {
            _id: graph_id,
            _data: graph_data,
            gdata: ordered
            ,_node_type: PhantomData,
        }
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
    fn endvertices(&self) -> (&T, &T) {
    if self.gdata.is_empty() {
        panic!("Diestel defines a path as non-empty; length 0 path not supported here.");
    }

    if self.gdata.len() == 1 {
        // Path of length 1: the edge is (u, v)
        return (self.gdata[0].start(), self.gdata[0].end());
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

    (start, end)
}
}

#[cfg(test)]
mod tests {

}
