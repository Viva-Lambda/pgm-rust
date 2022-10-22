// edge type

use crate::graph::graphtype::edgetype::EdgeType;
use crate::graph::graphtype::node::Node;
use crate::graph::graphtype::obj::GraphObject;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq)]
struct Edge {
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

impl Edge {
    pub fn start(&self) -> &Node {
        &self.start_node
    }
    pub fn end(&self) -> &Node {
        &self.end_node
    }
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
    pub fn is_start(&self, n: &Node) -> bool {
        self.start().id() == n.id()
    }
    pub fn is_end(&self, n: &Node) -> bool {
        self.end().id() == n.id()
    }
    pub fn has_type(&self) -> EdgeType {
        self.edge_type.clone()
    }
    pub fn node_ids(&self) -> HashSet<String> {
        let mut hset = HashSet::new();
        hset.insert(self.start().id().clone());
        hset.insert(self.end().id().clone());
        hset.clone()
    }
    pub fn is_endvertice(&self, n: &Node) -> bool {
        let ids = self.node_ids();
        let nid: &String = &n.id();
        ids.contains(nid)
    }
    pub fn get_other(&self, n: &Node) -> &Node {
        let nid: &String = n.id();
        let start = self.start();
        let sid = start.id();
        let end = self.end();
        let eid = end.id();
        if sid == nid {
            self.end()
        } else if sid == eid {
            self.start()
        } else {
            let this_edge = dbg!(self);
            let this_node = dbg!(n);
            panic!("{n} does not belong to this {self}");
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
        assert_eq!(e.id(), String::from("uedge"));
    }
    fn test_data() {
        let e = mk_uedge();
        let mut h1 = HashMap::new();
        h1.insert(String::from("my"), vec![String::from("data")]);
        assert_eq!(e.data(), h1);
    }
    fn test_has_type() {
        let e = mk_uedge();
        assert_eq!(e.has_type(), EdgeType::Undirected);
    }
    fn test_start() {
        let e = mk_uedge();
        assert_eq!(e.start(), Node::new(String::from("m1"), HashMap::new()));
    }
    fn test_end() {
        let e = mk_uedge();
        assert_eq!(e.end(), Node::new(String::from("m2"), HashMap::new()));
    }
    fn test_node_ids() {
        let e = mk_uedge();
        let ids = e.node_ids();
        let mut h1 = HashSet::new();
        h1.insert("m1".to_string());
        h1.insert("m2".to_string());
        assert_eq!(ids, h1);
    }
    fn test_endvertice_true() {
        let e = mk_uedge();
        let n1 = Node::new(String::from("m1"), HashMap::new());
        assert!(e.is_endvertice(&n1));
    }
    fn test_endvertice_false() {
        let e = mk_uedge();
        let n1 = Node::new(String::from("m3"), HashMap::new());
        assert!(!e.is_endvertice(&n1)); //
    }
}
