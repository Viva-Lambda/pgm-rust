// graph node

// call the GraphObject trait
pub use crate::graph::traits::graph_obj::GraphObject;
pub use crate::graph::traits::node::Node as NodeTrait;
use crate::graph::types::misc::GraphContent;
use crate::graph::types::misc::GraphObjects;

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
        Node {
            node_id: n.id().clone(),
            node_data: n.data().clone(),
        }
    }

    /// constructor for node like objects that implement node trait with move
    pub fn from_nodish<T: NodeTrait>(n: T) -> Node {
        Node {
            node_id: n.id().clone(),
            node_data: n.data().clone(),
        }
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
        write!(f, "Node[ id: {} ]", nid)
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node_id.hash(state);
    }
}

impl GraphObject for Node {
    fn id(&self) -> &String {
        &self.node_id
    }

    fn data(&self) -> &HashMap<String, Vec<String>> {
        &self.node_data
    }
}

impl NodeTrait for Node {}

pub type Vertices = GraphObjects<Node>;
pub type V<'a> = GraphContent<'a, Node>;

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
        let mut my_map2: HashMap<String, Vec<String>> = HashMap::new();
        let myv2 = vec![
            String::from("awesome"),
            String::from("string"),
            String::from("stuff"),
        ];
        my_map2.insert(String::from("my"), myv2);
        assert_eq!(my_node.data(), &my_map2);
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
