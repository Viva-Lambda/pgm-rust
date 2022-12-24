//! Set operation functions defined on graphs

use crate::graph::traits::edge::Edge;
use crate::graph::traits::graph::Graph as GraphTrait;
use crate::graph::traits::node::Node;
use crate::graph::types::graph::Graph;
use std::collections::HashSet;

/// intersection of edges
pub fn intersection_edge<'a, T: Edge>(a1: HashSet<&'a T>, a2: HashSet<&'a T>) -> HashSet<&'a T> {
    let mut inter = HashSet::new();
    for i in a1.intersection(&a2) {
        // instead of moving the reference we copy the reference
        inter.insert(i.clone());
    }
    inter
}
/// intersection of nodes
pub fn intersection_node<'a, T: Node>(a1: HashSet<&'a T>, a2: HashSet<&'a T>) -> HashSet<&'a T> {
    let mut inter = HashSet::new();
    for i in a1.intersection(&a2) {
        // instead of moving the reference we copy the reference
        inter.insert(i.clone());
    }
    inter
}
/// intersection of graph
pub fn intersection<'a, T: GraphTrait>(a1: &'a T, a2: &'a T) -> Graph {
    //
    let vs1 = a1.vertices();
    let vs2 = a2.vertices();

    let es1 = a1.edges();
    let es2 = a2.edges();
    let vs = intersection_node(vs1, vs2);
    let es = intersection_edge(es1, es2);
    Graph::from_edge_node_refs_set(es, vs)
}

/// union of edges
pub fn union_edge<'a, T: Edge>(a1: HashSet<&'a T>, a2: HashSet<&'a T>) -> HashSet<&'a T> {
    let mut inter = HashSet::new();
    for i in a1.union(&a2) {
        // instead of moving the reference we copy the reference
        inter.insert(i.clone());
    }
    inter
}
/// union of nodes
pub fn union_node<'a, T: Node>(a1: HashSet<&'a T>, a2: HashSet<&'a T>) -> HashSet<&'a T> {
    let mut inter = HashSet::new();
    for i in a1.union(&a2) {
        // instead of moving the reference we copy the reference
        inter.insert(i.clone());
    }
    inter
}
/// union of graph
pub fn union<'a, T: GraphTrait>(a1: &'a T, a2: &'a T) -> Graph {
    //
    let vs1 = a1.vertices();
    let vs2 = a2.vertices();

    let es1 = a1.edges();
    let es2 = a2.edges();
    let vs = union_node(vs1, vs2);
    let es = union_edge(es1, es2);
    Graph::from_edge_node_refs_set(es, vs)
}

/// difference
/// difference of edges
pub fn difference_edge<'a, T: Edge>(a1: HashSet<&'a T>, a2: HashSet<&'a T>) -> HashSet<&'a T> {
    let mut inter = HashSet::new();
    for i in a1.difference(&a2) {
        // instead of moving the reference we copy the reference
        inter.insert(i.clone());
    }
    inter
}
/// difference of nodes
pub fn difference_node<'a, T: Node>(a1: HashSet<&'a T>, a2: HashSet<&'a T>) -> HashSet<&'a T> {
    let mut inter = HashSet::new();
    for i in a1.difference(&a2) {
        // instead of moving the reference we copy the reference
        inter.insert(i.clone());
    }
    inter
}
/// difference of graph
pub fn difference<'a, T: GraphTrait>(a1: &'a T, a2: &'a T) -> Graph {
    //
    let vs1 = a1.vertices();
    let vs2 = a2.vertices();

    let es1 = a1.edges();
    let es2 = a2.edges();
    let vs = difference_node(vs1, vs2);
    let es = difference_edge(es1, es2);
    Graph::from_edge_node_refs_set(es, vs)
}

/// symmetric difference
pub fn symmetric_difference_edge<'a, T: Edge>(
    a1: HashSet<&'a T>,
    a2: HashSet<&'a T>,
) -> HashSet<&'a T> {
    let mut inter = HashSet::new();
    for i in a1.symmetric_difference(&a2) {
        // instead of moving the reference we copy the reference
        inter.insert(i.clone());
    }
    inter
}
/// symmetric difference of nodes
pub fn symmetric_difference_node<'a, T: Node>(
    a1: HashSet<&'a T>,
    a2: HashSet<&'a T>,
) -> HashSet<&'a T> {
    let mut inter = HashSet::new();
    for i in a1.symmetric_difference(&a2) {
        // instead of moving the reference we copy the reference
        inter.insert(i.clone());
    }
    inter
}
/// difference of graph
pub fn symmetric_difference<'a, T: GraphTrait>(a1: &'a T, a2: &'a T) -> Graph {
    //
    let vs1 = a1.vertices();
    let vs2 = a2.vertices();

    let es1 = a1.edges();
    let es2 = a2.edges();
    let vs = symmetric_difference_node(vs1, vs2);
    let es = symmetric_difference_edge(es1, es2);
    Graph::from_edge_node_refs_set(es, vs)
}

/// contains

/// contains of edges
pub fn contains_edge<'a, T: Edge>(a1: HashSet<&'a T>, a2: HashSet<&'a T>) -> bool {
    a2.is_subset(&a1)
}
/// contains of nodes
pub fn contains_node<'a, T: Node>(a1: HashSet<&'a T>, a2: HashSet<&'a T>) -> bool {
    a2.is_subset(&a1)
}
/// contains of graph
pub fn contains<'a, T: GraphTrait>(a1: &'a T, a2: &'a T) -> bool {
    //
    let vs1 = a1.vertices();
    let vs2 = a2.vertices();

    let es1 = a1.edges();
    let es2 = a2.edges();
    let has_node = contains_node(vs1, vs2);
    let has_edge = contains_edge(es1, es2);
    has_node && has_edge
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::types::edge::Edge;
    use crate::graph::types::edgetype::EdgeType;
    use crate::graph::types::graph::Graph;
    use crate::graph::types::node::Node;
    use std::collections::HashMap;
    use std::collections::HashSet;

    fn mk_node(n_id: &str) -> Node {
        Node::empty(n_id)
    }
    fn mk_nodes(ns: Vec<&str>) -> HashSet<Node> {
        let mut hs: HashSet<Node> = HashSet::new();
        for n in ns {
            hs.insert(mk_node(n));
        }
        hs
    }
    fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge {
        Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
    }
    fn mk_edges(es: Vec<Edge>) -> HashSet<Edge> {
        let mut hs = HashSet::new();
        for e in es {
            hs.insert(e);
        }
        hs
    }

    fn mk_edge_refs<'a>(es: &'a Vec<Edge>) -> HashSet<&'a Edge> {
        let mut hs = HashSet::new();
        for e in es {
            hs.insert(e);
        }
        hs
    }
    fn mk_node_refs<'a>(es: &'a Vec<Node>) -> HashSet<&'a Node> {
        let mut hs = HashSet::new();
        for e in es {
            hs.insert(e);
        }
        hs
    }
    fn mk_g1() -> Graph {
        let e1 = mk_uedge("n1", "n3", "e1");
        let e2 = mk_uedge("n2", "n3", "e2");
        let e3 = mk_uedge("n2", "n4", "e3");
        let nset = mk_nodes(vec!["n1", "n2", "n3", "n4", "n5"]);
        let h1 = HashMap::new();
        let h2 = mk_edges(vec![e1, e2, e3]);
        Graph::new("g1".to_string(), nset, h2, h1)
    }

    fn mk_g2() -> Graph {
        let e1 = mk_uedge("n1", "n3", "e1");
        let e2 = mk_uedge("n20", "n30", "e2");
        let e3 = mk_uedge("n20", "n40", "e3");
        let nset = mk_nodes(vec!["n1", "n2", "n3", "n20", "n30"]);
        let h1 = HashMap::new();
        let h2 = mk_edges(vec![e1, e2, e3]);
        Graph::new("g2".to_string(), nset, h2, h1)
    }

    #[test]
    fn test_intersection_edge() {
        let g1 = mk_g1();
        let g1es = g1.edges();
        let e1 = mk_uedge("n1", "n3", "e1");
        let e2 = mk_uedge("n20", "n30", "e2");
        let e3 = mk_uedge("n20", "n40", "e3");
        let evs = vec![e1.clone(), e2, e3];
        let es = mk_edge_refs(&evs);
        let einter = intersection_edge(g1es, es);
        let mut comp = HashSet::new();
        comp.insert(&e1);
        assert_eq!(einter, comp);
    }

    #[test]
    fn test_intersection_node() {
        let g1 = mk_g1();
        let g1ns = g1.vertices();
        let n1 = mk_node("n1");
        let n2 = mk_node("n20");
        let n3 = mk_node("n30");
        let nvs = vec![n1.clone(), n2, n3];
        let ns = mk_node_refs(&nvs);
        let ninter = intersection_node(g1ns, ns);
        let mut comp = HashSet::new();
        comp.insert(&n1);
        assert_eq!(ninter, comp);
    }

    #[test]
    fn test_intersection() {
        let g1 = mk_g1();
        let g2 = mk_g2();
        let g1interg2 = intersection(&g1, &g2);
        let inter_v = g1interg2.vertices();
        let inter_e = g1interg2.edges();
        let mut comp_e = HashSet::new();
        let e1 = mk_uedge("n1", "n3", "e1");
        comp_e.insert(&e1);
        let vs = vec![mk_node("n1"), mk_node("n2"), mk_node("n3")];
        let comp_v = mk_node_refs(&vs);
        assert_eq!(inter_v, comp_v);
        assert_eq!(inter_e, comp_e);
    }
}
