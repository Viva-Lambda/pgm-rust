// edge type

use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::types::edgetype::EdgeType;
use crate::graph::types::node::Node;
use std::collections::HashMap;
use std::fmt;

use std::hash::{Hash, Hasher};

/// Edge object.
/// Formally defined as set with two elements, see Diestel 2017, p. 2
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Edge {
    edge_id: String,
    edge_data: HashMap<String, Vec<String>>,
    edge_type: EdgeType,
    start_node: Node,
    end_node: Node,
}
impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let eid = &self.edge_id;
        let n1 = &self.start_node;
        let n2 = &self.end_node;
        let et = &self.edge_type;
        write!(
            f,
            "Edge[ id: {}, start: {}, end: {}, type: {} ]",
            eid, n1, n2, et
        )
    }
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.edge_id.hash(state);
    }
}

impl GraphObject for Edge {
    fn id(&self) -> &String {
        &self.edge_id
    }

    fn data(&self) -> &HashMap<String, Vec<String>> {
        &self.edge_data
    }
}

impl EdgeTrait for Edge {
    fn start(&self) -> &Node {
        &self.start_node
    }
    fn end(&self) -> &Node {
        &self.end_node
    }
    fn has_type(&self) -> &EdgeType {
        &self.edge_type
    }
}

impl Edge {
    /// edge constructor
    pub fn new(
        eid: String,
        snode: Node,
        enode: Node,
        etype: EdgeType,
        e_data: HashMap<String, Vec<String>>,
    ) -> Edge {
        Edge {
            edge_id: eid,
            start_node: snode,
            end_node: enode,
            edge_type: etype,
            edge_data: e_data,
        }
    }
    /// undirected edge constructor
    pub fn undirected(
        eid: String,
        snode: Node,
        enode: Node,
        e_data: HashMap<String, Vec<String>>,
    ) -> Edge {
        Edge {
            edge_id: eid,
            start_node: snode,
            end_node: enode,
            edge_type: EdgeType::Undirected,
            edge_data: e_data,
        }
    }
    /// directed edge constructor
    pub fn directed(
        eid: String,
        snode: Node,
        enode: Node,
        e_data: HashMap<String, Vec<String>>,
    ) -> Edge {
        Edge {
            edge_id: eid,
            start_node: snode,
            end_node: enode,
            edge_type: EdgeType::Directed,
            edge_data: e_data,
        }
    }
    /// a generic constructor for edge like objects with burrowing
    pub fn from_edgish_ref<T: EdgeTrait>(e: &T) -> Edge {
        Edge {
            edge_id: e.id().clone(),
            edge_data: e.data().clone(),
            start_node: e.start().clone(),
            end_node: e.end().clone(),
            edge_type: e.has_type().clone(),
        }
    }
    /// a generic constructor for edge like objects with move
    pub fn from_edgish<T: EdgeTrait>(e: T) -> Edge {
        Edge {
            edge_id: e.id().clone(),
            edge_data: e.data().clone(),
            start_node: e.start().clone(),
            end_node: e.end().clone(),
            edge_type: e.has_type().clone(),
        }
    }
    /// empty edge constructor.
    pub fn empty(
        edge_id: &str,
        edge_type: EdgeType,
        start_node_id: &str,
        end_node_id: &str,
    ) -> Edge {
        let n1 = Node::empty(start_node_id);
        let n2 = Node::empty(end_node_id);
        let h: HashMap<String, Vec<String>> = HashMap::new();
        Edge {
            edge_id: edge_id.to_string(),
            edge_type,
            start_node: n1,
            end_node: n2,
            edge_data: h,
        }
    }
}
#[cfg(test)]
mod tests {

    use super::*; // brings in the parent scope to current module scope

    fn mk_dedge() -> Edge {
        let n1 = Node::new(String::from("m1"), HashMap::new());
        let n2 = Node::new(String::from("m2"), HashMap::new());
        let mut h1 = HashMap::new();
        h1.insert(String::from("my"), vec![String::from("data")]);
        Edge::directed(String::from("medge"), n1, n2, h1)
    }
    fn mk_uedge() -> Edge {
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
