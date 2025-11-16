// graph node

// call the GraphObject trait
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::traits::node::Node as NodeTrait;
use crate::graph::traits::node::VertexSet as VertexSetTrait;

// call the utilities
use crate::graph::types::utils::from_borrowed_data;
use crate::graph::types::utils::to_borrowed_data;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Node object.
/// Formally defined as a member/point/vertex of a graph, see Diestel 2017, p.2
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    node_id: String,
    node_data: HashMap<String, Vec<String>>,
}

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
            node_id: nid,
            node_data: ndata,
        }
    }
    /// constructor for node like objects that implement node trait with borrowing
    pub fn from_nodish_ref<T: NodeTrait>(n: &T) -> Node {
        let ndata = n.data();
        let data = from_borrowed_data(&ndata);
        Node {
            node_id: n.id().to_string(),
            node_data: data,
        }
    }

    /// constructor for node like objects that implement node trait with move
    pub fn from_nodish<T: NodeTrait>(n: T) -> Node {
        Node::from_nodish_ref(&n)
    }
    /// empty constructor
    pub fn empty(nid: &str) -> Node {
        let ndata: HashMap<String, Vec<String>> = HashMap::new();
        Node {
            node_id: nid.to_string(),
            node_data: ndata,
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nid = &self.node_id;
        write!(f, "<Node id='{}'/>", nid)
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node_id.hash(state);
    }
}

impl GraphObject for Node {
    fn id(&self) -> &str {
        &self.node_id
    }

    fn data(&self) -> HashMap<&str, Vec<&str>> {
        let data = to_borrowed_data(&self.node_data);
        data
    }

    fn null() -> Node {
        let nid = String::from("");
        Node::empty(&nid)
    }

    fn set_id(&self, idstr: &str) -> Self {
        let mut n = Node::null();
        n.node_id = idstr.to_string();
        n.node_data = self.node_data.clone();
        n
    }

    fn set_data(&self, data: HashMap<&str, Vec<&str>>) -> Self {
        let mut n = Node::null();
        n.node_id = self.node_id.clone();
        n.node_data = from_borrowed_data(&data);
        n
    }
}

impl NodeTrait for Node {
    fn create(nid: String, ndata: HashMap<String, Vec<String>>) -> Node {
        Node::new(nid, ndata)
    }
}

#[cfg(test)]
mod tests {

    use super::*; // brings in the parent scope to current module scope

    #[test]
    fn test_id() {
        let my_node = Node {
            node_id: String::from("mnode"),
            node_data: HashMap::new(),
        };
        assert_eq!(my_node.id(), &String::from("mnode"));
    }
    #[test]
    fn test_data() {
        let mut my_map: HashMap<String, Vec<String>> = HashMap::new();
        let myv = vec![
            String::from("awesome"),
            String::from("string"),
            String::from("stuff"),
        ];
        my_map.insert(String::from("my"), myv);
        let my_node = Node {
            node_id: String::from("mnode"),
            node_data: my_map,
        };
        let mut my_map2: HashMap<&str, Vec<&str>> = HashMap::new();
        let myv2 = vec!["awesome", "string", "stuff"];
        my_map2.insert("my", myv2);

        assert_eq!(my_node.data(), my_map2);
    }

    #[test]
    fn test_from_nodish_ref() {
        let mut my_map: HashMap<String, Vec<String>> = HashMap::new();
        let myv = vec![
            String::from("awesome"),
            String::from("string"),
            String::from("stuff"),
        ];
        my_map.insert(String::from("my"), myv);
        let my_node = Node {
            node_id: String::from("mnode"),
            node_data: my_map,
        };
        let n2 = Node::from_nodish_ref(&my_node);
        assert_eq!(my_node, n2);
    }
    #[test]
    fn test_from_nodish() {
        let mut my_map: HashMap<String, Vec<String>> = HashMap::new();
        let myv = vec![
            String::from("awesome"),
            String::from("string"),
            String::from("stuff"),
        ];
        my_map.insert(String::from("my"), myv);
        let my_node = Node {
            node_id: String::from("mnode"),
            node_data: my_map,
        };
        let n1 = my_node.clone();
        let n2 = Node::from_nodish(my_node);
        assert_eq!(n1, n2);
    }
}
