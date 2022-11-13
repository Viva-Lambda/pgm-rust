//! functions that has a graph among its arguments that output a value

use crate::graph::ops::edge::boolops::is_endvertice;
use crate::graph::traits::graph::Graph;
use crate::graph::traits::graph_obj::GraphObject;
use std::collections::HashMap;
use std::collections::HashSet;
use std::option::Option;

/// create an edge list representation of graph

/// for each node we register all the edges
pub fn to_adjacencylist<'a, G: Graph>(g: &'a G) -> HashMap<&'a str, Option<HashSet<&'a str>>> {
    let mut elist: HashMap<&str, Option<HashSet<&str>>> = HashMap::new();
    for node in g.vertices() {
        let mut n_es: HashSet<&str> = HashSet::new();
        for edge in g.edges() {
            if is_endvertice(edge, node) {
                n_es.insert(edge.id());
            }
        }
        let nid = node.id();
        if n_es.is_empty() {
            elist.insert(nid, None);
        } else {
            elist.insert(nid, Some(n_es));
        }
    }
    elist
}

/// obtain graph object using its identifier
pub fn by_id<'a, G, T, F>(g: &'a G, id: &str, f: F) -> &'a T
where
    G: Graph,
    T: GraphObject,
    F: Fn(&'a G) -> HashSet<&'a T>,
{
    for h in f(g) {
        if h.id() == id {
            return h;
        }
    }
    panic!("{id} not contained in {g}");
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
    fn mk_g1() -> Graph {
        let e1 = mk_uedge("n1", "n3", "e1");
        let e2 = mk_uedge("n2", "n3", "e2");
        let e3 = mk_uedge("n2", "n4", "e3");
        let nset = mk_nodes(vec!["n1", "n2", "n3", "n4", "n5"]);
        let h1 = HashMap::new();
        let h2 = mk_edges(vec![e1, e2, e3]);
        Graph::new("g1".to_string(), nset, h2, h1)
    }

    fn mk_refset(es: Vec<&str>) -> HashSet<&str> {
        let mut ns: HashSet<&str> = HashSet::new();
        for e in es {
            ns.insert(e);
        }
        ns
    }

    #[test]
    fn test_to_adjacencylist() {
        let g = mk_g1();
        let alst = to_adjacencylist(&g);
        let n4_ns = mk_refset(vec!["e3"]);
        //
        let n3_ns = mk_refset(vec!["e1", "e2"]);
        //
        let n2_ns = mk_refset(vec!["e2", "e3"]);
        //
        let n1_ns = mk_refset(vec!["e1"]);
        //
        let mut comp = HashMap::new();
        comp.insert("n5", None);
        comp.insert("n4", Some(n4_ns));
        comp.insert("n3", Some(n3_ns));
        comp.insert("n2", Some(n2_ns));
        comp.insert("n1", Some(n1_ns));
        assert_eq!(comp, alst);
    }

    #[ignore]
    #[test]
    fn test_to_adjmat() {}

    #[ignore]
    #[test]
    fn test_get_subgraph_by_vertices() {}
}
