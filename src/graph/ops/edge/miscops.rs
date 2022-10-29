//! Functions that has an [Edge] among arguments that output various values.

use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::graph_obj::GraphObject;
use std::collections::HashSet;

/// extract node identifiers from a `e`
pub fn node_ids<E: EdgeTrait>(e: &E) -> HashSet<String> {
    let mut hset = HashSet::new();
    hset.insert(e.start().id().clone());
    hset.insert(e.end().id().clone());
    hset.clone()
}

#[cfg(test)]
mod tests {

    use super::*; // brings in the parent scope to current module scope
    use crate::graph::types::edge::Edge;
    use crate::graph::types::node::Node;
    use std::collections::HashMap;

    fn mk_uedge() -> Edge {
        let n1 = Node::new(String::from("m1"), HashMap::new());
        let n2 = Node::new(String::from("m2"), HashMap::new());
        let mut h1 = HashMap::new();
        h1.insert(String::from("my"), vec![String::from("data")]);
        Edge::undirected(String::from("uedge"), n1, n2, h1)
    }

    #[test]
    fn test_node_ids() {
        let e = mk_uedge();
        let ids = node_ids(&e);
        let mut h1 = HashSet::new();
        h1.insert("m1".to_string());
        h1.insert("m2".to_string());
        assert_eq!(ids, h1);
    }
}
