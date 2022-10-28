//! functions that has a graph among its arguments that output a boolean value
use crate::graph::ops::edge::boolops::is_endvertice;
use crate::graph::ops::edge::miscops::node_ids;
use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::graph::Graph;
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::traits::node::Node;
use std::collections::HashSet;

pub fn is_empty<G: Graph>(g: &G) -> bool {
    g.vertices().is_empty()
}

/// check if given graph object is in graph

pub fn is_in<G, T>(g: &G, element: &T) -> bool
where
    G: Graph,
    T: GraphObject,
{
    let eid = element.id();
    let mut ns = HashSet::new();
    for e in g.edges() {
        if e.id() == eid {
            return true;
        }
        let estart = e.start().clone();
        if estart.id() == eid {
            return true;
        }
        ns.insert(estart);
        let eend = e.end().clone();
        if eend.id() == eid {
            return true;
        }
        ns.insert(eend);
    }
    for n in g.vertices().difference(&ns) {
        if n.id() == eid {
            return true;
        }
    }
    false
}

/// Check if two edges are adjacent
/// - e1 an edge like object
/// - e2 an edge like object
/// - g a graph like object
pub fn is_adjacent_of<G, E>(g: &G, e1: &E, e2: &E) -> bool
where
    G: Graph,
    E: EdgeTrait,
{
    if !is_in(g, e1) {
        panic!("{e1} not in {g}");
    }
    if !is_in(g, e2) {
        panic!("{e2} not in {g}");
    }
    let e1_ns = node_ids(e1);
    let e2_ns = node_ids(e2);
    let common: HashSet<_> = e1_ns.intersection(&e2_ns).collect();
    !common.is_empty()
}

/// Check if a node and edge is incident
/// - n a node like object
/// - e an edge like object
/// - g a graph like object
pub fn is_node_incident<G, E, N>(g: &G, e: &E, n: &N) -> bool
where
    G: Graph,
    E: EdgeTrait,
    N: Node,
{
    if !is_in(g, e) {
        panic!("{e} not in {g}");
    }
    if !is_in(g, n) {
        panic!("{n} not in {g}");
    }
    is_endvertice(e, n)
}

#[cfg(test)]
mod tests {

    use super::*;
    //
    use crate::graph::types::edge::Edge;
    use crate::graph::types::graph::Graph;
    use crate::graph::types::node::Node;
    use std::collections::HashMap;

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
    fn mk_g1() -> Graph {
        let e1 = mk_uedge("n1", "n2", "e1");
        let e2 = mk_uedge("n2", "n3", "e1");
        let mut nset = HashSet::new();
        nset.insert(e1.start().clone());
        nset.insert(e1.end().clone());
        nset.insert(e2.start().clone());
        nset.insert(e2.end().clone());
        let mut h1 = HashMap::new();
        h1.insert(String::from("my"), vec![String::from("data")]);
        let mut h2 = HashSet::new();
        h2.insert(e1);
        h2.insert(e2);
        Graph::new("g1".to_string(), nset, h2, h1)
    }
    #[test]
    fn test_is_empty_true() {
        let edges = HashSet::new();
        let g = Graph::from_edgeset(edges);
        assert!(is_empty(&g));
    }
    #[test]
    fn test_is_empty_false() {
        let mut edges = HashSet::new();
        edges.insert(mk_uedge("n1", "n2", "e1"));
        let g = Graph::from_edgeset(edges);
        assert!(!is_empty(&g));
    }

    #[test]
    fn test_is_in_true() {
        let g1 = mk_g1();
        let n1 = mk_node("n1");
        assert!(is_in(&g1, &n1));
    }
    #[test]
    fn test_is_in_false() {
        let g1 = mk_g1();
        let n1 = mk_node("n55");
        assert!(!is_in(&g1, &n1));
    }

    #[ignore]
    #[test]
    fn test_is_adjacent_of() {}

    #[ignore]
    #[test]
    fn test_is_node_incident() {}

    #[ignore]
    #[test]
    fn test_is_neighbor_of() {}
}
