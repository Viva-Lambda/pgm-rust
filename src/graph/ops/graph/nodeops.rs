use crate::graph::ops::edge::boolops::is_endvertice;
use crate::graph::ops::edge::nodeops::get_other;
use crate::graph::ops::graph::boolops::is_in;
use crate::graph::ops::graph::miscops::by_id;
use crate::graph::traits::edge::Edge as EdgeTrait;
///
use crate::graph::traits::graph::Graph as GraphTrait;
use crate::graph::traits::node::Node as NodeTrait;
use std::collections::HashSet;

/// Find the neighbors of a given node.
/// # Description
/// Given a nodish object in a graphish object, find neighboring nodes to
/// nodish object. For the definition of neighbor, see Diestel, p. 3.
///
/// # Args
/// - n: something that implements [NodeTrait] trait
/// - g: something that implements [Graph] trait
/// - returns: a set of nodes that are neighbors of `n`
/// # Example
/// ```
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::types::graph::Graph;
/// use pgm_rust::graph::types::node::Node;
/// use pgm_rust::graph::ops::graph::nodeops::neighbors_of;
/// use std::collections::HashMap;
/// use std::collections::HashSet;
///
/// fn mk_node(n_id: &str) -> Node {
///     Node::empty(n_id)
/// }
/// fn mk_nodes(ns: Vec<&str>) -> HashSet<Node> {
///     let mut hs: HashSet<Node> = HashSet::new();
///     for n in ns {
///         hs.insert(mk_node(n));
///     }
///     hs
/// }
/// fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge<Node> {
///     Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
/// }
/// fn mk_edges(es: Vec<Edge<Node>>) -> HashSet<Edge<Node>> {
///     let mut hs = HashSet::new();
///     for e in es {
///         hs.insert(e);
///     }
///     hs
/// }
/// fn mk_g1() -> Graph<Node, Edge<Node>> {
///     let e1 = mk_uedge("n1", "n3", "e1");
///     let e2 = mk_uedge("n2", "n3", "e2");
///     let e3 = mk_uedge("n2", "n4", "e3");
///     let nset = mk_nodes(vec!["n1", "n2", "n3", "n4"]);
///     let h1 = HashMap::new();
///     let h2 = mk_edges(vec![e1, e2, e3]);
///     Graph::new("g1".to_string(), h1, nset, h2)
/// }
///
/// let g = mk_g1();
/// let n1 = mk_node("n1");
/// let n2 = mk_node("n2");
/// let ns = neighbors_of(&g, &n2);
/// let n3 = mk_node("n3");
/// let n4 = mk_node("n4");
/// let mut comps_t = HashSet::new();
/// comps_t.insert(&n3);
/// comps_t.insert(&n4);
/// let mut comps_f = HashSet::new();
/// comps_f.insert(&n1);
/// ns == comps_t;// true
/// ns == comps_f;// false
/// ```
/// # References
/// Diestel R. Graph Theory. 2017.
pub fn neighbors_of<'a, 'b, N, E, G>(g: &'a G, n: &'b N) -> HashSet<&'a N>
where
    N: NodeTrait,
    E: EdgeTrait<N> + 'a,
    G: GraphTrait<N, E>,
{
    // check if node is in graph
    if !is_in(g, n) {
        panic!("{n} not in {g}");
    }
    let mut neighbors = HashSet::new();
    for e in g.edges() {
        if is_endvertice(e, n) {
            let n2 = get_other(e, n);
            neighbors.insert(n2);
        }
    }
    // check is in
    neighbors
}

/// get vertices using their identifier
/// # Description
/// Given an identifier get its corresponding node
///
/// # Args
/// - g: something that implements [Graph] trait
/// - vid: string reference
///
/// # Example
/// ```
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::types::graph::Graph;
/// use pgm_rust::graph::types::node::Node;
/// use pgm_rust::graph::ops::graph::nodeops::vertex_by_id;
/// use std::collections::HashMap;
/// use std::collections::HashSet;
///
/// fn mk_node(n_id: &str) -> Node {
///     Node::empty(n_id)
/// }
/// fn mk_nodes(ns: Vec<&str>) -> HashSet<Node> {
///     let mut hs: HashSet<Node> = HashSet::new();
///     for n in ns {
///         hs.insert(mk_node(n));
///     }
///     hs
/// }
/// fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge<Node> {
///     Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
/// }
/// fn mk_edges(es: Vec<Edge<Node>>) -> HashSet<Edge<Node>> {
///     let mut hs = HashSet::new();
///     for e in es {
///         hs.insert(e);
///     }
///     hs
/// }
/// fn mk_g1() -> Graph<Node, Edge<Node>> {
///     let e1 = mk_uedge("n1", "n3", "e1");
///     let e2 = mk_uedge("n2", "n3", "e2");
///     let e3 = mk_uedge("n2", "n4", "e3");
///     let nset = mk_nodes(vec!["n1", "n2", "n3", "n4"]);
///     let h1 = HashMap::new();
///     let h2 = mk_edges(vec![e1, e2, e3]);
///     Graph::new("g1".to_string(), h1, nset, h2)
/// }
///
/// let g = mk_g1();
/// let n1 = mk_node("n1");
/// vertex_by_id(&g, "n1") == &n1; // true
/// ```
pub fn vertex_by_id<'a, N, E, G>(g: &'a G, vid: &str) -> &'a N
where
    N: NodeTrait,
    E: EdgeTrait<N>,
    G: GraphTrait<N, E>,
{
    let f = |mg: &'a G| -> HashSet<&'a N> { mg.vertices() };
    by_id(g, vid, f)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::graph::types::edge::Edge;
    use crate::graph::types::edgetype::EdgeType;
    use crate::graph::types::graph::Graph;
    use crate::graph::types::node::Node;
    use std::collections::HashMap;

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
    fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge<Node> {
        Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
    }
    fn mk_edges(es: Vec<Edge<Node>>) -> HashSet<Edge<Node>> {
        let mut hs = HashSet::new();
        for e in es {
            hs.insert(e);
        }
        hs
    }
    fn mk_g1() -> Graph<Node, Edge<Node>> {
        let e1 = mk_uedge("n1", "n3", "e1");
        let e2 = mk_uedge("n2", "n3", "e2");
        let e3 = mk_uedge("n2", "n4", "e3");
        let nset = mk_nodes(vec!["n1", "n2", "n3", "n4"]);
        let h1 = HashMap::new();
        let h2 = mk_edges(vec![e1, e2, e3]);
        Graph::new("g1".to_string(), h1, nset, h2)
    }

    #[test]
    fn test_vertex_by_id() {
        let g = mk_g1();
        let n2 = mk_node("n2");
        assert_eq!(&n2, vertex_by_id(&g, "n2"));
    }

    #[test]
    fn test_neighbors_of_true() {
        let g = mk_g1();
        let n2 = mk_node("n2");
        let ns = neighbors_of(&g, &n2);
        let n3 = mk_node("n3");
        let n4 = mk_node("n4");
        let mut comps = HashSet::new();
        comps.insert(&n3);
        comps.insert(&n4);
        assert_eq!(ns, comps);
    }

    #[test]
    fn test_neighbors_of_false() {
        let g = mk_g1();
        let n2 = mk_node("n2");
        let n1 = mk_node("n1");
        let ns = neighbors_of(&g, &n2);
        let mut comps = HashSet::new();
        comps.insert(&n1);
        assert_ne!(ns, comps);
    }
}
