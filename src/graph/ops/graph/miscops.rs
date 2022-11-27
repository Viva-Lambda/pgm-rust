//! functions that has a graph among its arguments that output a value

use crate::graph::ops::edge::boolops::is_endvertice;
use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::graph::Graph;
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::traits::node::Node as NodeTrait;
use crate::graph::types::edge::Edge;
use crate::graph::types::node::Node;
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

/// Obtain the adjacency matrix of the graph
/// # Description
/// Adjacency matrix contains information about the adjacency of vertices.
/// Its keys are vertex identifiers, and values are booleans.
/// # Args
/// - g: something that implements [Graph] trait.
/// # Example
/// ```
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::types::graph::Graph;
/// use pgm_rust::graph::ops::graph::miscops::to_adjmat;
/// use pgm_rust::graph::traits::graph_obj::GraphObject;
/// use pgm_rust::graph::types::node::Node;
/// use std::collections::HashMap;
/// use std::collections::HashSet;
/// fn mk_node(n_id: &str) -> Node {Node::empty(n_id)}
///
/// fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge {
///     Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
/// }
/// fn mk_edges(es: Vec<Edge>) -> HashSet<Edge> {
///     let mut hs = HashSet::new();
///     for e in es {
///         hs.insert(e);
///     }
///     hs
/// }
///
/// fn mk_nodes(ns: Vec<&str>) -> HashSet<Node> {
///     let mut hs: HashSet<Node> = HashSet::new();
///     for n in ns {
///         hs.insert(mk_node(n));
///     }
///     hs
/// }
///
/// let a = mk_node("a");
/// let b = mk_node("b");
/// let f = mk_node("f");
/// let e = mk_node("e");
/// let ae = mk_uedge("a", "e", "ae");
/// let af = mk_uedge("a", "f", "af");
/// let ef = mk_uedge("e", "f", "ef");
/// let nset = mk_nodes(vec!["a", "b", "f", "e"]);
/// let h1 = HashMap::new();
/// let h2 = mk_edges(vec![ae, af, ef]);
/// let g1 = Graph::new("g1".to_string(), nset, h2, h1);
/// let mut comp = HashMap::new();
/// comp.insert((b.id(), b.id()), false);
/// comp.insert((b.id(), e.id()), false);
/// comp.insert((b.id(), f.id()), false);
/// comp.insert((b.id(), a.id()), false);
/// comp.insert((e.id(), b.id()), false);
/// comp.insert((e.id(), e.id()), false);
/// comp.insert((e.id(), f.id()), true);
/// comp.insert((e.id(), a.id()), true);
/// comp.insert((f.id(), b.id()), false);
/// comp.insert((f.id(), e.id()), true);
/// comp.insert((f.id(), f.id()), false);
/// comp.insert((f.id(), a.id()), true);
/// comp.insert((a.id(), b.id()), false);
/// comp.insert((a.id(), e.id()), true);
/// comp.insert((a.id(), f.id()), true);
/// comp.insert((a.id(), a.id()), false);
/// let amat = to_adjmat(&g1);
/// amat == comp; // true
/// ```
pub fn to_adjmat<'a, G: Graph>(g: &'a G) -> HashMap<(&'a String, &'a String), bool> {
    //
    let mut adjmat = HashMap::new();
    for e in g.edges() {
        let n1 = e.start();
        let n2 = e.end();
        let n1_id = n1.id();
        let n2_id = n2.id();
        adjmat.insert((n1_id, n2_id), true);
        adjmat.insert((n2_id, n1_id), true);
    }
    for n1 in g.vertices() {
        for n2 in g.vertices() {
            let n1_id = n1.id();
            let n2_id = n2.id();
            if !adjmat.contains_key(&(n1_id, n2_id)) {
                adjmat.insert((n1_id, n2_id), false);
                adjmat.insert((n2_id, n1_id), false);
            }
        }
    }
    adjmat
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

/// Get subgraph using given vertices
/// # Description
/// We extract the subgraph using the provided node set.
///
/// # Args
/// - g: something that implements [Graph] trait.
/// - ns: a set of things that implement [Node] trait
/// - edge_policy: defines how to handle edges given a node. By default, we
/// conserve edges whose incident nodes are a subset of `ns`
pub fn get_subgraph_by_vertices<'a, G, N, F>(
    g: &'a G,
    ns: HashSet<&N>,
    edge_policy: Option<F>,
) -> (HashSet<&'a Node>, HashSet<&'a Edge>)
where
    G: Graph,
    N: NodeTrait,
    F: Fn(&'a Edge, &HashSet<&N>) -> bool,
{
    let policy = |e: &'a Edge, vs: &HashSet<&N>| -> bool {
        match &edge_policy {
            Some(p) => p(e, vs),
            None => {
                let n1 = e.start();
                let n2 = e.end();
                let mut n1_c = false;
                let mut n2_c = false;
                for v in vs {
                    let vid = v.id();
                    if vid == n1.id() {
                        n1_c = true;
                    }
                    if vid == n2.id() {
                        n2_c = true;
                    }
                }
                n1_c && n2_c
            }
        }
    };
    let mut eset = HashSet::new();
    for e in g.edges() {
        if policy(e, &ns) {
            eset.insert(e);
        }
    }
    let vset: HashSet<&String> = ns.iter().map(|&n| -> &String { n.id() }).collect();
    let mut nset = HashSet::new();
    for n in g.vertices() {
        if vset.contains(n.id()) {
            nset.insert(n);
        }
    }
    (nset, eset)
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

    #[test]
    fn test_to_adjmat() {
        let a = mk_node("a");
        let b = mk_node("b");
        let f = mk_node("f");
        let e = mk_node("e");
        let ae = mk_uedge("a", "e", "ae");
        let af = mk_uedge("a", "f", "af");
        let ef = mk_uedge("e", "f", "ef");
        let nset = mk_nodes(vec!["a", "b", "f", "e"]);
        let h1 = HashMap::new();
        let h2 = mk_edges(vec![ae, af, ef]);
        let g1 = Graph::new("g1".to_string(), nset, h2, h1);
        let mut comp = HashMap::new();
        comp.insert((b.id(), b.id()), false);
        comp.insert((b.id(), e.id()), false);
        comp.insert((b.id(), f.id()), false);
        comp.insert((b.id(), a.id()), false);
        comp.insert((e.id(), b.id()), false);
        comp.insert((e.id(), e.id()), false);
        comp.insert((e.id(), f.id()), true);
        comp.insert((e.id(), a.id()), true);
        comp.insert((f.id(), b.id()), false);
        comp.insert((f.id(), e.id()), true);
        comp.insert((f.id(), f.id()), false);
        comp.insert((f.id(), a.id()), true);
        comp.insert((a.id(), b.id()), false);
        comp.insert((a.id(), e.id()), true);
        comp.insert((a.id(), f.id()), true);
        comp.insert((a.id(), a.id()), false);
        let amat = to_adjmat(&g1);
        assert_eq!(amat, comp);
    }

    #[test]
    fn test_get_subgraph_by_vertices_default_edge_policy() {
        let g1 = mk_g1();
        let n1 = mk_node("n1");
        let n2 = mk_node("n2");
        let n4 = mk_node("n4");
        let mut nrefset = HashSet::new();
        nrefset.insert(&n1);
        nrefset.insert(&n2);
        nrefset.insert(&n4);
        let nrefset2 = nrefset.clone();
        let mut erefset = HashSet::new();
        let e1 = mk_uedge("n2", "n4", "e3");
        erefset.insert(&e1);
        // let opt: Option<dyn Fn(&Edge, &HashSet<&Node>) -> bool> = None;
        let opt: Option<Box<dyn Fn(&Edge, &HashSet<&Node>) -> bool>> = None;
        // let opt = None;
        let result: (HashSet<&Node>, HashSet<&Edge>) = get_subgraph_by_vertices(&g1, nrefset, opt);
        let (nodes, edges) = result;
        assert_eq!(nodes, nrefset2);

        //
        assert_eq!(edges, erefset);
    }

    #[test]
    fn test_get_subgraph_by_vertices_inclusive_edge_policy() {
        let g1 = mk_g1();
        let n1 = mk_node("n1");
        let n3 = mk_node("n3");
        let mut nrefset = HashSet::new();
        nrefset.insert(&n1);
        nrefset.insert(&n3);
        let nrefset2 = nrefset.clone();
        let mut erefset = HashSet::new();
        let e1 = mk_uedge("n1", "n3", "e1");
        let e2 = mk_uedge("n2", "n3", "e2");
        erefset.insert(&e1);
        erefset.insert(&e2);
        // let opt: Option<dyn Fn(&Edge, &HashSet<&Node>) -> bool> = None;
        let policy = |e: &Edge, vs: &HashSet<&Node>| -> bool {
            let n1 = e.start();
            let n2 = e.end();
            let mut n1_c = false;
            let mut n2_c = false;
            for v in vs {
                let vid = v.id();
                if vid == n1.id() {
                    n1_c = true;
                }
                if vid == n2.id() {
                    n2_c = true;
                }
            }
            n1_c || n2_c
        };

        let opt = Some(policy);
        // let opt = None;
        let result: (HashSet<&Node>, HashSet<&Edge>) = get_subgraph_by_vertices(&g1, nrefset, opt);
        let (nodes, edges) = result;
        assert_eq!(nodes, nrefset2);

        //
        assert_eq!(edges, erefset);
    }
}
