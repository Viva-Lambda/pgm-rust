//! graph operations that output edge
//
use crate::graph::ops::edge::boolops::is_endvertice;
use crate::graph::ops::graph::boolops::is_in;
use crate::graph::ops::graph::miscops::by_id;
use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::graph::Graph;
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::traits::node::Node;
use crate::graph::types::edge::Edge;
use std::collections::HashSet;

fn mk_edgeset<'a, 'b, G, N, F>(g: &'a G, n: &'b N, mut f: F) -> HashSet<&'a Edge>
where
    G: Graph,
    N: Node,
    F: FnMut(&'a Edge, &'b N) -> bool,
{
    if !is_in(g, n) {
        panic!("{g} does not contain {n}");
    }
    let mut hset = HashSet::new();
    for e in g.edges() {
        if f(e, n) {
            hset.insert(e);
        }
    }
    hset
}

/// get all the edges associated to a node.
/// # Description
/// We iterate over the edges of a graphish object,
/// and collect the edges incident to given nodish object.
/// # Args
/// - g something that implements [Graph] trait
/// - n something that implements [Node] trait

/// # Example
/// ```
/// use pgm_rust::graph::traits::edge::Edge as EdgeTrait;
/// use pgm_rust::graph::traits::graph::Graph as GraphTrait;
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::ops::graph::edgeops::edges_of;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::types::graph::Graph;
/// use pgm_rust::graph::types::node::Node;
/// use std::collections::HashMap;
/// use std::collections::HashSet;
///
/// fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge {
///     Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
/// }
/// fn mk_g1() -> Graph {
///     let e1 = mk_uedge("n1", "n2", "e1");
///     let e2 = mk_uedge("n2", "n3", "e2");
///     let mut nset = HashSet::new();
///     nset.insert(e1.start().clone());
///     nset.insert(e1.end().clone());
///     nset.insert(e2.start().clone());
///     nset.insert(e2.end().clone());
///     nset.insert(Node::empty("n4"));
///     let mut h1 = HashMap::new();
///     h1.insert(String::from("my"), vec![String::from("data")]);
///     let mut h2 = HashSet::new();
///     h2.insert(e1);
///     h2.insert(e2);
///     Graph::new("g1".to_string(), nset, h2, h1)
/// }
/// let g = mk_g1();
/// let n2 = Node::empty("n2");
/// let hset = edges_of(&g, &n2);
/// let es = g.edges();
/// hset == es; // true
/// ```
pub fn edges_of<'a, 'b, G, N>(g: &'a G, n: &'b N) -> HashSet<&'a Edge>
where
    G: Graph,
    N: Node,
{
    let cond_fn = |e: &'a Edge, n: &'b N| -> bool { is_endvertice(e, n) };
    mk_edgeset(g, n, cond_fn)
}

/// gets the outgoing edges of a given node object
/// # Description
/// Gets all the outgoing edges of the given node object. The outgoing edge
/// set implies that all of its member start from the given node object.
/// # Args
/// - g something that implements [Graph] trait
/// - n something that implements [Node] trait
/// # Example
/// ```
/// use pgm_rust::graph::traits::edge::Edge as EdgeTrait;
/// use pgm_rust::graph::traits::graph::Graph as GraphTrait;
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::ops::graph::edgeops::outgoing_edges_of;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::types::graph::Graph;
/// use pgm_rust::graph::types::node::Node;
/// use std::collections::HashMap;
/// use std::collections::HashSet;
///
/// fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge {
///     Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
/// }
/// fn mk_g1() -> Graph {
///     let e1 = mk_uedge("n1", "n2", "e1");
///     let e2 = mk_uedge("n2", "n3", "e2");
///     let mut nset = HashSet::new();
///     nset.insert(e1.start().clone());
///     nset.insert(e1.end().clone());
///     nset.insert(e2.start().clone());
///     nset.insert(e2.end().clone());
///     nset.insert(Node::empty("n4"));
///     let mut h1 = HashMap::new();
///     h1.insert(String::from("my"), vec![String::from("data")]);
///     let mut h2 = HashSet::new();
///     h2.insert(e1);
///     h2.insert(e2);
///     Graph::new("g1".to_string(), nset, h2, h1)
/// }
/// let g = mk_g1();
/// let n2 = Node::empty("n2");
/// let hset = outgoing_edges_of(&g, &n2);
/// let mut h2 = HashSet::new();
/// let e2 = mk_uedge("n2", "n3", "e2");
/// h2.insert(&e2);
/// hset == h2; // true
/// ```
pub fn outgoing_edges_of<'a, 'b, G, N>(g: &'a G, n: &'b N) -> HashSet<&'a Edge>
where
    G: Graph,
    N: Node,
{
    let cond_fn = |e: &'a Edge, n: &'b N| -> bool { e.start().id() == n.id() };
    mk_edgeset(g, n, cond_fn)
}

/// gets the incoming edges of a given node object
/// # Description
/// Gets all the incoming edges of the given node object. The incoming edge
/// set implies that all of its member end at the given node object.
/// # Args
/// - g something that implements [Graph] trait
/// - n something that implements [Node] trait
/// # Example
/// ```
/// use pgm_rust::graph::traits::edge::Edge as EdgeTrait;
/// use pgm_rust::graph::traits::graph::Graph as GraphTrait;
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::ops::graph::edgeops::incoming_edges_of;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::types::graph::Graph;
/// use pgm_rust::graph::types::node::Node;
/// use std::collections::HashMap;
/// use std::collections::HashSet;
///
/// fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge {
///     Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
/// }
/// fn mk_g1() -> Graph {
///     let e1 = mk_uedge("n1", "n2", "e1");
///     let e2 = mk_uedge("n2", "n3", "e2");
///     let mut nset = HashSet::new();
///     nset.insert(e1.start().clone());
///     nset.insert(e1.end().clone());
///     nset.insert(e2.start().clone());
///     nset.insert(e2.end().clone());
///     nset.insert(Node::empty("n4"));
///     let mut h1 = HashMap::new();
///     h1.insert(String::from("my"), vec![String::from("data")]);
///     let mut h2 = HashSet::new();
///     h2.insert(e1);
///     h2.insert(e2);
///     Graph::new("g1".to_string(), nset, h2, h1)
/// }
/// let g = mk_g1();
/// let n2 = Node::empty("n2");
/// let hset = incoming_edges_of(&g, &n2);
/// let mut h2 = HashSet::new();
/// let e1 = mk_uedge("n1", "n2", "e1");
/// h2.insert(&e1);
/// hset == h2; // true
/// ```
pub fn incoming_edges_of<'a, 'b, G, N>(g: &'a G, n: &'b N) -> HashSet<&'a Edge>
where
    G: Graph,
    N: Node,
{
    let cond_fn = |e: &'a Edge, n: &'b N| -> bool { e.end().id() == n.id() };
    mk_edgeset(g, n, cond_fn)
}
/// collect edges using their end vertices
/// # Description
/// We collect edges that have given nodes as their end vertices.
/// # Args
/// - g something that implements [Graph] trait
/// - n1 something that implements [Node] trait
/// - n2 something that implements [Node] trait
/// # Example
/// ```
/// use pgm_rust::graph::traits::edge::Edge as EdgeTrait;
/// use pgm_rust::graph::traits::graph::Graph as GraphTrait;
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::ops::graph::edgeops::edges_by_vertices;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::types::graph::Graph;
/// use pgm_rust::graph::types::node::Node;
/// use std::collections::HashMap;
/// use std::collections::HashSet;
///
/// fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge {
///     Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
/// }
/// fn mk_g1() -> Graph {
///     let e1 = mk_uedge("n1", "n2", "e1");
///     let e2 = mk_uedge("n2", "n3", "e2");
///     let mut nset = HashSet::new();
///     nset.insert(e1.start().clone());
///     nset.insert(e1.end().clone());
///     nset.insert(e2.start().clone());
///     nset.insert(e2.end().clone());
///     nset.insert(Node::empty("n4"));
///     let mut h1 = HashMap::new();
///     h1.insert(String::from("my"), vec![String::from("data")]);
///     let mut h2 = HashSet::new();
///     h2.insert(e1);
///     h2.insert(e2);
///     Graph::new("g1".to_string(), nset, h2, h1)
/// }
/// let g = mk_g1();
/// let n1 = Node::empty("n1");
/// let n2 = Node::empty("n2");
/// let hset = edges_by_vertices(&g, &n1, &n2);
/// let mut h2 = HashSet::new();
/// let e1 = mk_uedge("n1", "n2", "e1");
/// h2.insert(&e1);
/// hset == h2; // true
/// ```
pub fn edges_by_vertices<'a, 'b, G, N>(g: &'a G, n1: &'b N, n2: &'b N) -> HashSet<&'a Edge>
where
    G: Graph,
    N: Node,
{
    if !is_in(g, n1) {
        panic!("{g} does not contain {n1}");
    }
    if !is_in(g, n2) {
        panic!("{g} does not contain {n2}");
    }
    //
    let mut hset = HashSet::new();
    for e in g.edges() {
        if is_endvertice(e, n1) && is_endvertice(e, n2) {
            hset.insert(e);
        }
    }
    hset
}

/// get an edge using its identifier
/// # Description
/// We output an edge using its identifier
/// # Args
/// - g something that implements [Graph] trait
/// - id a string slice
/// # Example
/// ```
/// use pgm_rust::graph::traits::edge::Edge as EdgeTrait;
/// use pgm_rust::graph::traits::graph::Graph as GraphTrait;
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::ops::graph::edgeops::edges_by_vertices;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::types::graph::Graph;
/// use pgm_rust::graph::types::node::Node;
/// use std::collections::HashMap;
/// use std::collections::HashSet;
///
/// fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge {
///     Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
/// }
/// fn mk_g1() -> Graph {
///     let e1 = mk_uedge("n1", "n2", "e1");
///     let e2 = mk_uedge("n2", "n3", "e2");
///     let mut nset = HashSet::new();
///     nset.insert(e1.start().clone());
///     nset.insert(e1.end().clone());
///     nset.insert(e2.start().clone());
///     nset.insert(e2.end().clone());
///     nset.insert(Node::empty("n4"));
///     let mut h1 = HashMap::new();
///     h1.insert(String::from("my"), vec![String::from("data")]);
///     let mut h2 = HashSet::new();
///     h2.insert(e1);
///     h2.insert(e2);
///     Graph::new("g1".to_string(), nset, h2, h1)
/// }
/// let g = mk_g1();
/// let n1 = Node::empty("n1");
/// let n2 = Node::empty("n2");
/// let hset = edges_by_vertices(&g, &n1, &n2);
/// let mut h2 = HashSet::new();
/// let e1 = mk_uedge("n1", "n2", "e1");
/// h2.insert(&e1);
/// hset == h2; // true
/// ```
pub fn edge_by_id<'a, 'b, G>(g: &'a G, id: &str) -> &'a Edge
where
    G: Graph,
{
    //
    let f = |mg: &'a G| -> HashSet<&'a Edge> { mg.edges() };
    by_id(g, id, f)
}

#[cfg(test)]
mod tests {
    use super::*;
    //
    use crate::graph::traits::edge::Edge as EdgeTrait;
    use crate::graph::traits::graph::Graph as GraphTrait;
    use crate::graph::types::edge::Edge;
    use crate::graph::types::edgetype::EdgeType;
    use crate::graph::types::graph::Graph;
    use crate::graph::types::node::Node;
    use std::collections::HashMap;

    fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge {
        Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
    }
    fn mk_g1() -> Graph {
        let e1 = mk_uedge("n1", "n2", "e1");
        let e2 = mk_uedge("n2", "n3", "e2");
        let mut nset = HashSet::new();
        nset.insert(e1.start().clone());
        nset.insert(e1.end().clone());
        nset.insert(e2.start().clone());
        nset.insert(e2.end().clone());
        nset.insert(Node::empty("n4"));
        let mut h1 = HashMap::new();
        h1.insert(String::from("my"), vec![String::from("data")]);
        let mut h2 = HashSet::new();
        h2.insert(e1);
        h2.insert(e2);
        Graph::new("g1".to_string(), nset, h2, h1)
    }

    #[test]
    fn test_edges_of() {
        let g = mk_g1();
        let n2 = Node::empty("n2");
        let hset = edges_of(&g, &n2);
        let es = g.edges();
        assert_eq!(hset, es);
    }

    #[test]
    fn test_outgoing_edges_of() {
        let g = mk_g1();
        let n2 = Node::empty("n2");
        let hset = outgoing_edges_of(&g, &n2);
        let mut h2 = HashSet::new();
        let e2 = mk_uedge("n2", "n3", "e2");
        h2.insert(&e2);
        assert_eq!(hset, h2);
    }

    #[test]
    fn test_incoming_edges_of() {
        let g = mk_g1();
        let n2 = Node::empty("n2");
        let hset = incoming_edges_of(&g, &n2);
        let mut h2 = HashSet::new();
        let e1 = mk_uedge("n1", "n2", "e1");
        h2.insert(&e1);
        assert_eq!(hset, h2);
    }

    #[ignore]
    #[test]
    fn test_edges_by_end() {}

    #[ignore]
    #[test]
    fn test_edge_by_id() {}

    #[test]
    fn test_edge_by_vertices() {
        let g = mk_g1();
        let n2 = Node::empty("n2");
        let n1 = Node::empty("n1");
        let hset = edges_by_vertices(&g, &n1, &n2);
        let mut h2 = HashSet::new();
        let e1 = mk_uedge("n1", "n2", "e1");
        h2.insert(&e1);
        assert_eq!(hset, h2); // true
    }
}
