// edge type

use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::edge::EdgeSet as EdgeSetTrait;
use crate::graph::traits::generic::{
    default_display_with_data_impl, default_getter_impl, default_hash_id_impl,
    default_idchanger_impl, default_identified_impl, default_loadchanger_impl, default_loaded_impl,
    default_named_impl, default_partial_eq_impl, default_setter_impl,
};
use crate::graph::traits::graph_obj::GraphObject;

use crate::graph::traits::generic::{
    render_hashmap, IdChanger, Identified, LoadChanger, Loaded, Named,
};

use crate::graph::traits::generic::default_with_hash_partial_eq_impl;
use crate::graph::traits::node::Node as NodeTrait;
use crate::graph::traits::utils::from_borrowed_data;
use crate::graph::traits::utils::to_borrowed_data;
use crate::graph::types::edgetype::EdgeType;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::marker::PhantomData;

/// Edge object.
/// Formally defined as set with two elements, see Diestel 2017, p. 2
#[derive(Debug, Clone)]
pub struct Edge<T: NodeTrait> {
    _id: String,
    _data: HashMap<String, Vec<String>>,
    edge_type: EdgeType,
    start_node: T,
    end_node: T,
}

default_with_hash_partial_eq_impl!(Edge, <T>, T: NodeTrait + Identified);

/// short hand for edge set
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Edges<N: NodeTrait, E: EdgeTrait<N>> {
    /// edge set content
    pub edge_set: HashSet<E>,
    node_type: PhantomData<N>,
}

impl<N: NodeTrait, E: EdgeTrait<N> + Clone> EdgeSetTrait<N, E> for Edges<N, E> {
    fn members(&self) -> HashSet<&E> {
        let mut es: HashSet<&E> = HashSet::new();
        for e in &self.edge_set {
            es.insert(&e);
        }
        es
    }
    fn create(hset: HashSet<&E>) -> Self {
        let mut es: HashSet<E> = HashSet::new();
        for e in hset {
            es.insert(e.clone());
        }
        Edges {
            edge_set: es,
            node_type: PhantomData,
        }
    }
}

impl<T: NodeTrait> fmt::Display for Edge<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n1 = &self.start().to_string();
        let n2 = &self.end().to_string();
        let et = &self.edge_type.to_string();
        let id_s = &self.id().to_string();
        let data_s = render_hashmap(&self.data());
        write!(
            f,
            "<Edge id='{}' type='{}'>\n<start>{}</start>\n<end>{}</end>\n{}</Edge>",
            id_s, et, n1, n2, data_s
        )
    }
}

impl<T: NodeTrait> GraphObject for Edge<T> {
    fn null() -> Edge<T> {
        let s = T::null();
        let e = T::null();
        Edge {
            _id: String::from(""),
            _data: HashMap::new(),
            edge_type: EdgeType::Undirected,
            start_node: s,
            end_node: e,
        }
    }
}

impl<T: NodeTrait> Edge<T> {
    /// edge constructor
    pub fn new(
        eid: String,
        e_data: HashMap<String, Vec<String>>,
        etype: EdgeType,
        snode: T,
        enode: T,
    ) -> Edge<T> {
        Edge {
            _id: eid,
            _data: e_data,
            edge_type: etype,
            start_node: snode,
            end_node: enode,
        }
    }

    /// undirected edge constructor
    pub fn undirected(
        eid: String,
        snode: T,
        enode: T,
        e_data: HashMap<String, Vec<String>>,
    ) -> Edge<T> {
        Edge::new(eid, e_data, EdgeType::Undirected, snode, enode)
    }
    /// directed edge constructor
    pub fn directed(
        eid: String,
        snode: T,
        enode: T,
        e_data: HashMap<String, Vec<String>>,
    ) -> Edge<T> {
        Edge::new(eid, e_data, EdgeType::Directed, snode, enode)
    }
    /// a generic constructor for edge like objects with burrowing
    pub fn from_edgish_ref<E: EdgeTrait<T>>(e: &E) -> Edge<T> {
        let eid = e.id().to_string();
        let e_data = from_borrowed_data(&e.data());
        let etype = e.has_type().clone();
        let snode = e.start().clone();
        let enode = e.end().clone();
        Edge::new(eid, e_data, etype, snode, enode)
    }
    /// a generic constructor for edge like objects with move
    pub fn from_edgish<E: EdgeTrait<T>>(e: E) -> Edge<T> {
        Edge::from_edgish_ref(&e)
    }
    /// empty edge constructor.
    pub fn empty() -> Edge<T> {
        Edge::null()
    }

    /// construct edge using identifiers
    pub fn from_ids(eid: &str, etype: EdgeType, start_id: &str, end_id: &str) -> Edge<T> {
        let mut e = Edge::null().set_id(eid);
        e.edge_type = etype;
        let start_n = T::null().set_id(start_id);
        e.start_node = start_n;
        let end_n = T::null().set_id(end_id);
        e.end_node = end_n;
        e
    }
}

impl<NodeType: NodeTrait> EdgeTrait<NodeType> for Edge<NodeType> {
    fn start(&self) -> &NodeType {
        &self.start_node
    }
    fn end(&self) -> &NodeType {
        &self.end_node
    }
    fn has_type(&self) -> &EdgeType {
        &self.edge_type
    }
    fn create(
        eid: String,
        e_data: HashMap<String, Vec<String>>,
        snode: NodeType,
        enode: NodeType,
        etype: EdgeType,
    ) -> Edge<NodeType> {
        Edge::new(eid, e_data, etype, snode, enode)
    }
}
#[cfg(test)]
mod tests {}
