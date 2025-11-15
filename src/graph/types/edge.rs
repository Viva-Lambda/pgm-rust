// edge type

use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::edge::EdgeSet as EdgeSetTrait;
use crate::graph::traits::graph_obj::GraphObject;

use crate::graph::traits::node::Node as NodeTrait;
use crate::graph::types::edgetype::EdgeType;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::marker::PhantomData;

use std::hash::{Hash, Hasher};

/// Edge info object.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EdgeInfo {
    id: String,
    data: HashMap<String, Vec<String>>,
    edge_type: EdgeType,
}

/// Edge object.
/// Formally defined as set with two elements, see Diestel 2017, p. 2
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Edge<T: NodeTrait> {
    info: EdgeInfo,
    start_node: T,
    end_node: T,
}

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
        let eid = &self.info.id;
        let n1 = &self.start_node;
        let n2 = &self.end_node;
        let et = &self.info.edge_type;
        write!(
            f,
            "<Edge id='{}' start='{}' end='{}' type='{}'/>",
            eid, n1, n2, et
        )
    }
}

impl<T: NodeTrait> Hash for Edge<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.info.id.hash(state);
        self.start_node.hash(state);
        self.end_node.hash(state)
    }
}

impl<T: NodeTrait> GraphObject for Edge<T> {
    fn id(&self) -> &String {
        &self.info.id
    }

    fn data(&self) -> &HashMap<String, Vec<String>> {
        &self.info.data
    }

    fn null() -> Edge<T> {
        let start = T::null();
        let end = T::null();
        let idstr = String::from("");
        let h: HashMap<String, Vec<String>> = HashMap::new();
        let info = EdgeInfo {
            id: idstr,
            edge_type: EdgeType::Undirected,
            data: h,
        };
        Edge {
            info,
            start_node: start,
            end_node: end,
        }
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
        &self.info.edge_type
    }
    fn create(
        eid: String,
        e_data: HashMap<String, Vec<String>>,
        snode: NodeType,
        enode: NodeType,
        etype: EdgeType,
    ) -> Edge<NodeType> {
        Edge::new(eid, e_data, snode, enode, etype)
    }
}

impl<T: NodeTrait> Edge<T> {
    /// edge constructor
    pub fn new(
        eid: String,
        e_data: HashMap<String, Vec<String>>,
        snode: T,
        enode: T,
        etype: EdgeType,
    ) -> Edge<T> {
        let info = EdgeInfo {
            id: eid,
            edge_type: etype,
            data: e_data,
        };
        Edge {
            info,
            start_node: snode,
            end_node: enode,
        }
    }
    /// construct edge from an edge info and nodes
    pub fn from_info(info: EdgeInfo, start_node: T, end_node: T) -> Edge<T> {
        Edge {
            info,
            start_node,
            end_node,
        }
    }
    /// undirected edge constructor
    pub fn undirected(
        eid: String,
        snode: T,
        enode: T,
        e_data: HashMap<String, Vec<String>>,
    ) -> Edge<T> {
        let info = EdgeInfo {
            id: eid,
            edge_type: EdgeType::Undirected,
            data: e_data,
        };
        Edge {
            info,
            start_node: snode,
            end_node: enode,
        }
    }
    /// directed edge constructor
    pub fn directed(
        eid: String,
        snode: T,
        enode: T,
        e_data: HashMap<String, Vec<String>>,
    ) -> Edge<T> {
        let info = EdgeInfo {
            id: eid,
            edge_type: EdgeType::Directed,
            data: e_data,
        };
        Edge {
            info,
            start_node: snode,
            end_node: enode,
        }
    }
    /// a generic constructor for edge like objects with burrowing
    pub fn from_edgish_ref<E: EdgeTrait<T>>(e: &E) -> Edge<T> {
        let info = EdgeInfo {
            id: e.id().to_string(),
            edge_type: e.has_type().clone(),
            data: e.data().clone(),
        };
        Edge {
            info,
            start_node: e.start().clone(),
            end_node: e.end().clone(),
        }
    }
    /// a generic constructor for edge like objects with move
    pub fn from_edgish<E: EdgeTrait<T>>(e: E) -> Edge<T> {
        let info = EdgeInfo {
            id: e.id().to_string(),
            edge_type: e.has_type().clone(),
            data: e.data().clone(),
        };
        Edge {
            info,
            start_node: e.start().clone(),
            end_node: e.end().clone(),
        }
    }
    /// empty edge constructor.
    pub fn empty(&self) -> Edge<T> {
        Edge::null()
    }
}
#[cfg(test)]
mod tests {

    use super::*; // brings in the parent scope to current module scope
    use crate::graph::types::node::Node;

    fn mk_uedge() -> Edge<Node> {
        let n1 = Node::new(String::from("m1"), HashMap::new());
        let n2 = Node::new(String::from("m2"), HashMap::new());
        let mut h1 = HashMap::new();
        h1.insert(String::from("my"), vec![String::from("data")]);
        Edge::undirected(String::from("uedge"), n1, n2, h1)
    }

    #[test]
    fn test_id() {
        let e = mk_uedge();
        assert_eq!(e.id(), &String::from("uedge"));
    }
    #[test]
    fn test_data() {
        let e = mk_uedge();
        let mut h1 = HashMap::new();
        h1.insert(String::from("my"), vec![String::from("data")]);
        assert_eq!(e.data(), &h1);
    }
    #[test]
    fn test_has_type() {
        let e = mk_uedge();
        assert_eq!(e.has_type(), &EdgeType::Undirected);
    }
    #[test]
    fn test_start() {
        let e = mk_uedge();
        assert_eq!(e.start(), &Node::new(String::from("m1"), HashMap::new()));
    }
    #[test]
    fn test_end() {
        let e = mk_uedge();
        assert_eq!(e.end(), &Node::new(String::from("m2"), HashMap::new()));
    }
    #[test]
    fn test_from_edgish_ref() {
        let e = mk_uedge();
        let n1 = Node::new(String::from("m1"), HashMap::new());
        let n2 = Node::new(String::from("m2"), HashMap::new());
        let mut h1 = HashMap::new();
        h1.insert(String::from("my"), vec![String::from("data")]);
        let e1 = Edge::undirected(String::from("uedge"), n1, n2, h1);
        let e2 = Edge::from_edgish_ref(&e);

        assert_eq!(e1, e2);
    }
    #[test]
    fn test_from_edgish() {
        let e = mk_uedge();
        let n1 = Node::new(String::from("m1"), HashMap::new());
        let n2 = Node::new(String::from("m2"), HashMap::new());
        let mut h1 = HashMap::new();
        h1.insert(String::from("my"), vec![String::from("data")]);
        let e1 = Edge::undirected(String::from("uedge"), n1, n2, h1);
        let e2 = Edge::from_edgish(e);

        assert_eq!(e1, e2);
    }
}
