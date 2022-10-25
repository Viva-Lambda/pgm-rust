//! functions that has a graph among its arguments that output a boolean value
use crate::graph::traits::graph::Graph as GraphTrait;
use crate::graph::types::graph::Graph;

pub fn is_empty(g: &Graph) -> bool {
    g.vertices().is_empty()
}

#[cfg(test)]
mod tests {

    use super::*;
    //
    use crate::graph::types::edge::Edge;
    use crate::graph::types::node::Node;
    use std::collections::HashMap;
    use std::collections::HashSet;

    fn mk_node(n_id: &str) -> Node {
        Node::new(n_id.to_string(), HashMap::new())
    }
    fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge {
        let n1 = mk_node(n1_id);
        let n2 = mk_node(n2_id);
        let mut h1 = HashMap::new();
        h1.insert(String::from("my"), vec![String::from("data")]);
        Edge::undirected(e_id.to_string(), n1, n2, h1)
    }
    fn test_is_empty_true() {
        let edges = HashSet::new();
        let g = Graph::from_edgeset(edges);
        assert!(is_empty(&g));
    }
    fn test_is_empty_false() {
        let mut edges = HashSet::new();
        edges.insert(mk_uedge("n1", "n2", "e1"));
        let g = Graph::from_edgeset(edges);
        assert!(is_empty(&g));
    }
}
