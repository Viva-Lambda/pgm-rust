// graph node

// call the GraphObject trait
pub use crate::graph::traits::graph_obj::GraphObject;

use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

// implements graph object trait
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    node_id: String,
    node_data: HashMap<String, Vec<String>>,
}

impl Node {
    pub fn new(nid: String, ndata: HashMap<String, Vec<String>>) -> Node {
        Node {
            node_id: nid,
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

#[cfg(test)]
mod tests {

    use super::*; // brings in the parent scope to current module scope

    #[test]
    fn test_id() {
        let my_node = Node {
            node_id: String::from("mnode"),
            node_data: HashMap::new(),
        };
        assert_eq!(my_node.id(), String::from("mnode"));
    }
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
        assert_eq!(my_node.data(), my_map2);
    }
}
