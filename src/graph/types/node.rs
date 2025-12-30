// graph node

// call the GraphObject trait
use crate::graph::traits::generic::{
    default_all_impl, default_display_with_data_impl, default_getter_impl, default_hash_id_impl,
    default_idchanger_impl, default_identified_impl, default_loadchanger_impl, default_loaded_impl,
    default_named_impl, default_partial_eq_impl, default_setter_impl,
};

use crate::graph::traits::generic::{IdChanger, Identified, LoadChanger, Loaded, Named};
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::traits::node::Node as NodeTrait;
use crate::graph::traits::node::VertexSet as VertexSetTrait;

// call the utilities
use crate::graph::traits::utils::from_borrowed_data;
use crate::graph::traits::utils::to_borrowed_data;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Node object.
/// Formally defined as a member/point/vertex of a graph, see Diestel 2017, p.2
#[derive(Debug, Clone)]
pub struct Node {
    _id: String,
    _data: HashMap<String, Vec<String>>,
}

default_all_impl!(Node);

/// Short hand for set of nodes
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Vertices<N: NodeTrait> {
    /// node set field
    pub vertex_set: HashSet<N>,
}

impl<N: NodeTrait> VertexSetTrait<N> for Vertices<N> {
    fn members(&self) -> HashSet<&N> {
        let mut ms: HashSet<&N> = HashSet::new();
        for v in &self.vertex_set {
            ms.insert(v);
        }
        ms
    }
    fn create(vs: HashSet<&N>) -> Self {
        let mut ms: HashSet<N> = HashSet::new();
        for v in &vs {
            let m: &N = v;
            ms.insert(m.clone());
        }
        Vertices { vertex_set: ms }
    }
}

impl Node {
    /// constructor for Node object
    pub fn new(nid: String, ndata: HashMap<String, Vec<String>>) -> Node {
        Node {
            _id: nid,
            _data: ndata,
        }
    }
    /// constructor for node like objects that implement node trait with borrowing
    pub fn from_nodish_ref<T: NodeTrait>(n: &T) -> Node {
        let ndata = n.data();
        let data = from_borrowed_data(&ndata);
        Node {
            _id: n.id().to_string(),
            _data: data,
        }
    }

    /// constructor for node like objects that implement node trait with move
    pub fn from_nodish<T: NodeTrait>(n: T) -> Node {
        Node::from_nodish_ref(&n)
    }
    /// empty constructor
    pub fn from_id(nid: &str) -> Node {
        let ndata: HashMap<String, Vec<String>> = HashMap::new();
        Node {
            _id: nid.to_string(),
            _data: ndata,
        }
    }
}

impl GraphObject for Node {
    fn null() -> Node {
        let nid = String::from("");
        Node::from_id(&nid)
    }
}

impl NodeTrait for Node {
    fn create(nid: String, ndata: HashMap<String, Vec<String>>) -> Node {
        Node::new(nid, ndata)
    }
}

#[cfg(test)]
mod tests {}
