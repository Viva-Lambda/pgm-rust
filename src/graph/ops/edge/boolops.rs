//! Functions that has an [Edge] among arguments that output a boolean value
use crate::graph::ops::edge::miscops::node_ids;
use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::node::Node as NodeTrait;

/// check if given `n` is start node of `e`
pub fn is_start<N, E>(e: &E, n: &N) -> bool
where
    N: NodeTrait,
    E: EdgeTrait<N>,
{
    e.start().id() == n.id()
}
/// check if given `n` is end node of `e`
pub fn is_end<N, E>(e: &E, n: &N) -> bool
where
    N: NodeTrait,
    E: EdgeTrait<N>,
{
    e.end().id() == n.id()
}

/// check if given `n` is belongs to `e`
pub fn is_endvertice<N, E>(e: &E, n: &N) -> bool
where
    N: NodeTrait,
    E: EdgeTrait<N>,
{
    let ids = node_ids(e);
    let nid: &String = &n.id();
    ids.contains(nid)
}

#[cfg(test)]
mod tests {

    use super::*;
    //
    use crate::graph::types::edge::Edge;
    use crate::graph::types::node::Node;
    use std::collections::HashMap; // brings in the parent scope to current module scope

    fn mk_uedge() -> Edge<Node> {
        let n1 = Node::new(String::from("m1"), HashMap::new());
        let n2 = Node::new(String::from("m2"), HashMap::new());
        let mut h1 = HashMap::new();
        h1.insert(String::from("my"), vec![String::from("data")]);
        Edge::undirected(String::from("uedge"), n1, n2, h1)
    }
    #[test]
    fn test_endvertice_true() {
        let e = mk_uedge();
        let n1 = Node::new(String::from("m1"), HashMap::new());
        assert!(is_endvertice(&e, &n1));
    }
    #[test]
    fn test_endvertice_false() {
        let e = mk_uedge();
        let n1 = Node::new(String::from("m3"), HashMap::new());
        assert!(!is_endvertice(&e, &n1)); //
    }
    #[test]
    fn test_is_start() {
        let e = mk_uedge();
        let n1 = Node::new(String::from("m1"), HashMap::new());
        assert!(is_start(&e, &n1));
    }
    #[test]
    fn test_is_end() {
        let e = mk_uedge();
        let n2 = Node::new(String::from("m2"), HashMap::new());
        assert!(is_end(&e, &n2));
    }
}
