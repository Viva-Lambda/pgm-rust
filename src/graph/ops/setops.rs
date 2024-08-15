//! Set operation functions defined on graphs

use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::graph::Graph as GraphTrait;
use crate::graph::traits::node::Node as NodeTrait;
use std::collections::HashMap;
use std::collections::HashSet;
use uuid::Uuid;

/// # Intersection Operations

/// ## Intersection of Two Edges
/// ### Description
/// Since the edges are defined as sets of vertices with two members, it is
/// natural to define set theoretical operations for them as well
///
/// ### Args
/// a1: Something that implements [Edge] trait
/// a2: Something that implements [Edge] trait
/// returns: a node set. Notice that it takes anything that implements the
/// edge trait, but returns a specific type.
///
/// ### Example
/// ```
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::types::graph::Graph;
/// use pgm_rust::graph::types::node::Node;
/// use pgm_rust::graph::types::node::Vertices;
/// use pgm_rust::graph::traits::node::VertexSet;
/// use pgm_rust::graph::ops::setops::intersection_edge;
/// use std::collections::HashSet;
///
/// fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge<Node> {
///     Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
/// }

/// let e1 = mk_uedge("n1", "n2", "e1");
/// let e2 = mk_uedge("n2", "n30", "e2");
/// let einter = intersection_edge(&e1, &e2);
/// let mut comph = HashSet::new();
/// let n2 = Node::empty("n2");
/// comph.insert(n2);
/// let comp = Vertices {vertex_set: comph};
/// einter == comp.members(); // outputs true
/// ```
pub fn intersection_edge<'a, N, E>(a1: &'a E, a2: &'a E) -> HashSet<&'a N>
where
    N: NodeTrait,
    E: EdgeTrait<N>,
{
    let mut hset1 = HashSet::new();
    let mut hset2 = HashSet::new();
    hset1.insert(a1.start());
    hset1.insert(a1.end());
    hset2.insert(a2.start());
    hset2.insert(a2.end());
    let mut inters = HashSet::new();
    for i in hset1.intersection(&hset2) {
        let vref: &'a N = i.clone();
        inters.insert(vref);
    }
    inters
}

/// ## Intersection of Edge Sets
/// ### Description
/// Basic intersection operation that works with hash sets with members
/// implementing the [Edge] trait
///
/// ### Args
/// a1: A hash set of things that implement [Edge] trait
/// a2: A hash set of things that implement [Edge] trait
/// returns: an set of things that implement [Edge] trait.
/// Notice that this operation conserves types of the members..
///
/// ### Example
/// ```
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::traits::graph::Graph as GraphTrait;
/// use pgm_rust::graph::types::graph::Graph;
/// use pgm_rust::graph::types::node::Node;
/// use pgm_rust::graph::ops::setops::intersection_edges;
/// use std::collections::HashSet;
/// use std::collections::HashMap;
///
///
/// fn mk_nodes(ns: Vec<&str>) -> HashSet<Node> {
///     let mut hs: HashSet<Node> = HashSet::new();
///     for n in ns {
///         hs.insert(Node::empty(n));
///     }
///     hs
/// }
/// fn mk_edges(es: Vec<Edge<Node>>) -> HashSet<Edge<Node>> {
///     let mut hs = HashSet::new();
///     for e in es {
///         hs.insert(e);
///     }
///     hs
/// }
///
/// fn mk_edge_refs<'a>(es: &'a Vec<Edge<Node>>) -> HashSet<&'a Edge<Node>> {
///     let mut hs = HashSet::new();
///     for e in es {
///         hs.insert(e);
///     }
///     hs
/// }
/// fn mk_node_refs<'a>(es: &'a Vec<Node>) -> HashSet<&'a Node> {
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
///     let nset = mk_nodes(vec!["n1", "n2", "n3", "n4", "n5"]);
///     let h1 = HashMap::new();
///     let h2 = mk_edges(vec![e1, e2, e3]);
///     Graph::new("g1".to_string(), h1, nset, h2)
/// }
/// fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge<Node> {
///     Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
/// }
/// let g1 = mk_g1();
/// let g1es = g1.edges();
/// let e1 = mk_uedge("n1", "n3", "e1");
/// let e2 = mk_uedge("n20", "n30", "e2");
/// let e3 = mk_uedge("n20", "n40", "e3");
/// let evs = vec![e1.clone(), e2, e3];
/// let es = mk_edge_refs(&evs);
/// let einter = intersection_edges(g1es, es);
/// let mut comp = HashSet::new();
/// comp.insert(&e1);
/// einter == comp;
/// ```
pub fn intersection_edges<'a, N, E>(a1: HashSet<&'a E>, a2: HashSet<&'a E>) -> HashSet<&'a E>
where
    N: NodeTrait,
    E: EdgeTrait<N>,
{
    let mut inter = HashSet::new();
    for i in a1.intersection(&a2) {
        // instead of moving the reference we copy the reference
        let tref: &'a E = i.clone();
        inter.insert(tref);
    }
    inter
}
/// ## Intersection of Node Sets
/// ### Description
/// Basic intersection operation that works with hash sets with members
/// implementing the [Node] trait
///
/// ### Args
/// a1: A hash set of things that implement [Node] trait
/// a2: A hash set of things that implement [Node] trait
/// returns: an set of things that implement [Node] trait.
/// Notice that this operation conserves types of the members..
///
/// ### Example
/// ```
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::traits::node::Node as NodeTrait;
/// use pgm_rust::graph::types::node::Node;
/// use pgm_rust::graph::ops::setops::intersection_nodes;
/// use std::collections::HashSet;
///
/// fn mk_node_refs<'a>(es: &'a Vec<Node>) -> HashSet<&'a Node> {
///     let mut hs = HashSet::new();
///     for e in es {
///         hs.insert(e);
///     }
///     hs
/// }
///
/// let n1 = Node::empty("n1");
/// let n2 = Node::empty("n2");
/// let n3 = Node::empty("n3");
/// let n4 = Node::empty("n4");
/// let n20 = Node::empty("n20");
/// let n30 = Node::empty("n30");
/// let nvs1 = vec![n1.clone(), n2, n3];
/// let nvs10 = vec![n1.clone(), n20, n30];
/// let ns1 = mk_node_refs(&nvs1);
/// let ns10 = mk_node_refs(&nvs10);
/// let ninter = intersection_nodes(ns1, ns10);
/// let mut comp = HashSet::new();
/// comp.insert(&n1);
/// ninter == comp;
/// ```
pub fn intersection_nodes<'a, T: NodeTrait>(
    a1: HashSet<&'a T>,
    a2: HashSet<&'a T>,
) -> HashSet<&'a T> {
    let mut inter = HashSet::new();
    for i in a1.intersection(&a2) {
        // instead of moving the reference we copy the reference
        let tref: &'a T = i.clone();
        inter.insert(tref);
    }
    inter
}
/// ## Intersection of Two Graphs
/// ### Description
/// Basic intersection operation that works with things that
/// implement the [Graph] trait
///
/// ### Args
/// a1: Something that implements [Graph] trait
/// a2: Something that implements [Graph] trait
/// returns: a Graph object
/// Notice that this operation returns a type.
///
/// ### Example
/// ```
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::traits::graph::Graph as GraphTrait;
/// use pgm_rust::graph::types::graph::Graph;
/// use pgm_rust::graph::types::node::Node;
/// use pgm_rust::graph::ops::setops::intersection;
/// use std::collections::HashSet;
/// use std::collections::HashMap;
///
/// fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge<Node> {
///     Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
/// }

/// fn mk_nodes(ns: Vec<&str>) -> HashSet<Node> {
///     let mut hs: HashSet<Node> = HashSet::new();
///     for n in ns {
///         hs.insert(Node::empty(n));
///     }
///     hs
/// }
/// fn mk_node_refs<'a>(es: &'a Vec<Node>) -> HashSet<&'a Node> {
///     let mut hs = HashSet::new();
///     for e in es {
///         hs.insert(e);
///     }
///     hs
/// }
///
/// fn mk_edges(es: Vec<Edge<Node>>) -> HashSet<Edge<Node>> {
///     let mut hs = HashSet::new();
///     for e in es {
///         hs.insert(e);
///     }
///     hs
/// }
///
/// fn mk_g1() -> Graph<Node, Edge<Node>> {
///     let e1 = mk_uedge("n1", "n3", "e1");
///     let e2 = mk_uedge("n2", "n3", "e2");
///     let e3 = mk_uedge("n2", "n4", "e3");
///     let nset = mk_nodes(vec!["n1", "n2", "n3", "n4", "n5"]);
///     let h1 = HashMap::new();
///     let h2 = mk_edges(vec![e1, e2, e3]);
///     Graph::new("g1".to_string(), h1, nset, h2)
/// }
/// fn mk_g2() -> Graph<Node, Edge<Node>> {
///     let e1 = mk_uedge("n1", "n3", "e1");
///     let e2 = mk_uedge("n20", "n30", "e2");
///     let e3 = mk_uedge("n20", "n40", "e3");
///     let nset = mk_nodes(vec!["n1", "n2", "n3", "n20", "n30"]);
///     let h1 = HashMap::new();
///     let h2 = mk_edges(vec![e1, e2, e3]);
///     Graph::new("g2".to_string(), h1, nset, h2)
/// }
///
/// let g1 = mk_g1();
/// let g2 = mk_g2();
/// let g1interg2 = intersection(&g1, &g2);
/// let inter_v = g1interg2.vertices();
/// let inter_e = g1interg2.edges();
/// let mut comp_e = HashSet::new();
/// let e1 = mk_uedge("n1", "n3", "e1");
/// comp_e.insert(&e1);
/// let vs = vec![Node::empty("n1"), Node::empty("n2"), Node::empty("n3")];
/// let comp_v = mk_node_refs(&vs);
/// inter_v == comp_v;
/// inter_e == comp_e;
///
/// ```
pub fn intersection<'a, N, E, G>(a1: &'a G, a2: &'a G) -> G
where
    N: NodeTrait,
    E: EdgeTrait<N>,
    G: GraphTrait<N, E>,
{
    //
    let vs1 = a1.vertices();
    let vs2 = a2.vertices();

    let es1 = a1.edges();
    let es2 = a2.edges();
    let vs = intersection_nodes(vs1, vs2);
    let es = intersection_edges(es1, es2);
    let gid = Uuid::new_v4().to_string();
    G::create_from_ref(gid, HashMap::new(), vs, es)
}

/// # Union Operations
/// ## Union of Node Sets
/// ### Description
/// Basic union operation that works with hash sets with members
/// implementing the [Node] trait
///
/// ### Args
/// a1: A hash set of things that implement [Node] trait
/// a2: A hash set of things that implement [Node] trait
/// returns: an set of things that implement [Node] trait.
/// Notice that this operation conserves types of the members..
///
/// ### Example
/// ```
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::traits::node::Node as NodeTrait;
/// use pgm_rust::graph::types::node::Node;
/// use pgm_rust::graph::ops::setops::union_nodes;
/// use std::collections::HashSet;
///
/// fn mk_node_refs<'a>(es: &'a Vec<Node>) -> HashSet<&'a Node> {
///     let mut hs = HashSet::new();
///     for e in es {
///         hs.insert(e);
///     }
///     hs
/// }
///
/// let n1 = Node::empty("n1");
/// let n2 = Node::empty("n2");
/// let n3 = Node::empty("n3");
/// let n4 = Node::empty("n4");
/// let n20 = Node::empty("n20");
/// let n30 = Node::empty("n30");
/// let nvs1 = vec![n1.clone(), n2, n3, n4];
/// let nvs10 = vec![n1.clone(), n20.clone(), n30.clone()];
/// let ns1 = mk_node_refs(&nvs1);
/// let ns10 = mk_node_refs(&nvs10);
/// let nunion = union_nodes(ns1.clone(), ns10);
/// let mut comp = HashSet::new();
/// for n in ns1 {
///     comp.insert(n);
/// }
/// comp.insert(&n20);
/// comp.insert(&n30);
/// nunion == comp;
/// ```
pub fn union_nodes<'a, T: NodeTrait>(a1: HashSet<&'a T>, a2: HashSet<&'a T>) -> HashSet<&'a T> {
    let mut inter = HashSet::new();
    for i in a1.union(&a2) {
        // instead of moving the reference we copy the reference
        inter.insert(i.clone());
    }
    inter
}
/// ## Union of Two Edges
/// ### Description
/// Produces a union of two edges. Since edges are defined to be
/// set of nodes, see Diestel 2017, p.2, we can apply set operations
/// to them.
///
/// ### Args
///
/// - a1: something that implements the [Edge] trait.
/// - a2: something that implements the [Edge] trait.
/// - returns: a set of node references. Notice that this is not a type
/// conserving operation. We output a specific type and not something that
/// implements a node trait.
///
/// ### Example
/// ```
/// use pgm_rust::graph::traits::edge::Edge as EdgeTrait;
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::types::node::Node;
/// use pgm_rust::graph::ops::setops::union_edge;
/// use std::collections::HashSet;

/// fn mk_node_refs<'a>(es: &'a Vec<Node>) -> HashSet<&'a Node> {
///     let mut hs = HashSet::new();
///     for e in es {
///         hs.insert(e);
///     }
///     hs
/// }
/// fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge<Node> {
///     Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
/// }
///
/// let e2 = mk_uedge("n20", "n30", "e2");
/// let e3 = mk_uedge("n20", "n40", "e3");
/// let eunion = union_edge(&e2, &e3);
/// let comp_v = vec![Node::empty("n20"), Node::empty("n30"), Node::empty("n40")];
/// let comp = mk_node_refs(&comp_v);
/// eunion == comp;
/// ```
pub fn union_edge<'a, N, E>(a1: &'a E, a2: &'a E) -> HashSet<&'a N>
where
    N: NodeTrait,
    E: EdgeTrait<N>,
{
    let mut a1nodes = HashSet::new();
    a1nodes.insert(a1.start());
    a1nodes.insert(a1.end());
    let mut a2nodes = HashSet::new();
    a2nodes.insert(a2.start());
    a2nodes.insert(a2.end());
    union_nodes(a1nodes, a2nodes)
}

/// ## Union of Edge Sets
/// ### Description
/// We unite the two sets whose members implement [Edge] trait.
/// TODO: There should be an option to output a node set as output
///
/// ### Args
///
/// - a1: set of things that implement the [Edge] trait.
/// - a2: set of things that implement the [Edge] trait.
/// - returns: a set of things that implement the [Edge] trait.
/// Notice that this is a type conserving operation.
///
/// ### Example
/// ```
/// use pgm_rust::graph::traits::edge::Edge as EdgeTrait;
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::traits::graph::Graph as GraphTrait;
/// use pgm_rust::graph::types::graph::Graph;
/// use pgm_rust::graph::types::node::Node;
/// use pgm_rust::graph::ops::setops::union_edges;
/// use std::collections::HashSet;
/// use std::collections::HashMap;

/// fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge<Node> {
///     Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
/// }
/// fn mk_nodes(ns: Vec<&str>) -> HashSet<Node> {
///     let mut hs: HashSet<Node> = HashSet::new();
///     for n in ns {
///         hs.insert(Node::empty(n));
///     }
///     hs
/// }
/// fn mk_edges(es: Vec<Edge<Node>>) -> HashSet<Edge<Node>> {
///     let mut hs = HashSet::new();
///     for e in es {
///         hs.insert(e);
///     }
///     hs
/// }
/// fn mk_edge_refs<'a>(es: &'a Vec<Edge<Node>>) -> HashSet<&'a Edge<Node>> {
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
///     let nset = mk_nodes(vec!["n1", "n2", "n3", "n4", "n5"]);
///     let h1 = HashMap::new();
///     let h2 = mk_edges(vec![e1, e2, e3]);
///     Graph::new("g1".to_string(), h1, nset, h2)
/// }
/// let g1 = mk_g1();
/// let g1es = g1.edges();
/// let e1 = mk_uedge("n1", "n3", "e1");
/// let e2 = mk_uedge("n20", "n30", "e2");
/// let e3 = mk_uedge("n20", "n40", "e3");
/// let evs = vec![e1.clone(), e2.clone(), e3.clone()];
/// let es = mk_edge_refs(&evs);
/// let eunion = union_edges(g1es.clone(), es);
/// let mut comp = HashSet::new();
/// for e in g1es {
///     comp.insert(e);
/// }
/// comp.insert(&e2);
/// comp.insert(&e3);
/// eunion == comp;
/// ```
pub fn union_edges<'a, N, E>(a1: HashSet<&'a E>, a2: HashSet<&'a E>) -> HashSet<&'a E>
where
    N: NodeTrait,
    E: EdgeTrait<N>,
{
    let mut inter = HashSet::new();
    for i in a1.union(&a2) {
        // instead of moving the reference we copy the reference
        inter.insert(i.clone());
    }
    inter
}
/// ## Union of Graph
/// ### Description
/// Get the union of two things implementing the [Graph] trait
///
/// ### Args
///
/// - a1: something that implements the [Graph] trait
/// - a2: something that implements the [Graph] trait
/// - returns: a [Graph] type.
/// Notice that this operation does not conserve types.
///
/// ### Example
/// ```
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::traits::graph::Graph as GraphTrait;
/// use pgm_rust::graph::types::graph::Graph;
/// use pgm_rust::graph::types::node::Node;
/// use pgm_rust::graph::ops::setops::union_graph;
/// use std::collections::HashSet;
/// use std::collections::HashMap;

/// fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge<Node> {
///     Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
/// }
/// fn mk_nodes(ns: Vec<&str>) -> HashSet<Node> {
///     let mut hs: HashSet<Node> = HashSet::new();
///     for n in ns {
///         hs.insert(Node::empty(n));
///     }
///     hs
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
///     let nset = mk_nodes(vec!["n1", "n2", "n3", "n4", "n5"]);
///     let h1 = HashMap::new();
///     let h2 = mk_edges(vec![e1, e2, e3]);
///     Graph::new("g1".to_string(), h1, nset, h2)
/// }
/// fn mk_g2() -> Graph<Node, Edge<Node>> {
///     let e1 = mk_uedge("n1", "n3", "e1");
///     let e2 = mk_uedge("n20", "n30", "e2");
///     let e3 = mk_uedge("n20", "n40", "e3");
///     let nset = mk_nodes(vec!["n1", "n2", "n3", "n20", "n30"]);
///     let h1 = HashMap::new();
///     let h2 = mk_edges(vec![e1, e2, e3]);
///     Graph::new("g2".to_string(), h1, nset, h2)
/// }
/// let g1 = mk_g1();
/// let g2 = mk_g2();
/// let g1uniong2 = union_graph(&g1, &g2);
/// let union_v = g1uniong2.vertices();
/// let union_e = g1uniong2.edges();
/// let mut comp_v = HashSet::new();
/// for v in g1.vertices() {
///     comp_v.insert(v);
/// }
/// for v in g2.vertices() {
///     comp_v.insert(v);
/// }
/// let mut comp_e = HashSet::new();
/// for e in g1.edges() {
///     comp_e.insert(e);
/// }
/// for e in g2.edges() {
///     comp_e.insert(e);
/// }
/// union_v == comp_v;
/// union_e == comp_e;
/// ```
pub fn union_graph<'a, N, E, G>(a1: &'a G, a2: &'a G) -> G
where
    N: NodeTrait,
    E: EdgeTrait<N>,
    G: GraphTrait<N, E>,
{
    //
    let vs1 = a1.vertices();
    let vs2 = a2.vertices();

    let es1 = a1.edges();
    let es2 = a2.edges();
    let vs = union_nodes(vs1, vs2);
    let es = union_edges(es1, es2);
    let gid = Uuid::new_v4().to_string();
    G::create_from_ref(gid, HashMap::new(), vs, es)
}

/// # Difference Operations
/// ## Difference of nodes
/// ### Description
/// Get the set difference of two node sets. Set difference is defined as
/// `A \ B = {a: a \in A and a \not \in B}`
///
/// ### Args
///
/// - a1: a set of things that implement the [Node] trait.
/// - a2: a set of things that implement the [Node] trait.
/// - returns: a set of things that implement the [Node] trait.
/// Notice that this is a type conserving operation.
///
/// ### Example
/// ```
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::traits::graph::Graph as GraphTrait;
/// use pgm_rust::graph::types::graph::Graph;
/// use pgm_rust::graph::types::node::Node;
/// use pgm_rust::graph::ops::setops::difference_nodes;
/// use std::collections::HashSet;
/// use std::collections::HashMap;
///
/// fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge<Node> {
///     Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
/// }
/// fn mk_nodes(ns: Vec<&str>) -> HashSet<Node> {
///     let mut hs: HashSet<Node> = HashSet::new();
///     for n in ns {
///         hs.insert(Node::empty(n));
///     }
///     hs
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
///     let nset = mk_nodes(vec!["n1", "n2", "n3", "n4", "n5"]);
///     let h1 = HashMap::new();
///     let h2 = mk_edges(vec![e1, e2, e3]);
///     Graph::new("g1".to_string(), h1, nset, h2)
/// }
/// fn mk_node_refs<'a>(es: &'a Vec<Node>) -> HashSet<&'a Node> {
///     let mut hs = HashSet::new();
///     for e in es {
///         hs.insert(e);
///     }
///     hs
/// }
/// let g1 = mk_g1();
/// let g1ns = g1.vertices();
/// let n1 = Node::empty("n1");
/// let n2 = Node::empty("n20");
/// let n3 = Node::empty("n30");
/// let nvs = vec![n1.clone(), n2.clone(), n3.clone()];
/// let ns = mk_node_refs(&nvs);
/// let ndiff = difference_nodes(ns, g1ns.clone());
/// let mut comp = HashSet::new();
/// comp.insert(&n2);
/// comp.insert(&n3);
/// ndiff == comp;
/// ```
pub fn difference_nodes<'a, T: NodeTrait>(
    a1: HashSet<&'a T>,
    a2: HashSet<&'a T>,
) -> HashSet<&'a T> {
    let mut inter = HashSet::new();
    for i in a1.difference(&a2) {
        // instead of moving the reference we copy the reference
        inter.insert(i.clone());
    }
    inter
}

/// ## Difference of Two Edges
/// ### Description
/// Get the set difference of two edges. Set difference is defined as
/// `A \ B = {a: a \in A and a \not \in B}`. This makes sense because the
/// edges are defined as sets with two nodes
///
/// ### Args
/// - a1: something that implements the [Edge] trait
/// - a2: something that implements the [Edge] trait
/// - returns: a set of nodes
/// Notice that this operation does not conserve types.
///
/// ### Example
///
/// ```
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::types::node::Node;
/// use pgm_rust::graph::ops::setops::difference_edge;
/// use std::collections::HashSet;
///
/// fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge<Node> {
///     Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
/// }
/// fn mk_node_refs<'a>(es: &'a Vec<Node>) -> HashSet<&'a Node> {
///     let mut hs = HashSet::new();
///     for e in es {
///         hs.insert(e);
///     }
///     hs
/// }
/// let e2 = mk_uedge("n20", "n30", "e2");
/// let e3 = mk_uedge("n20", "n40", "e3");
/// let ediff = difference_edge(&e2, &e3);
/// let comp_v = vec![Node::empty("n30")];
/// let comp = mk_node_refs(&comp_v);
/// ediff == comp;
/// ```
pub fn difference_edge<'a, N: NodeTrait, E: EdgeTrait<N>>(a1: &'a E, a2: &'a E) -> HashSet<&'a N> {
    let mut a1nodes = HashSet::new();
    a1nodes.insert(a1.start());
    a1nodes.insert(a1.end());
    let mut a2nodes = HashSet::new();
    a2nodes.insert(a2.start());
    a2nodes.insert(a2.end());
    difference_nodes(a1nodes, a2nodes)
}
/// ## Difference of Two Edge Sets
/// ### Description
/// Get the set difference of two edge sets. Set difference is defined as
/// `A \ B = {a: a \in A and a \not \in B}`. Two edges are considered equal if
/// they have same nodes and same identifier.
///
/// ### Args
/// - a1: set of something that implements the [Edge] trait
/// - a2: set of something that implements the [Edge] trait
/// - returns: a set of something that implements the [Edge] trait
/// Notice that this operation conserve types.
///
/// ### Example
/// ```
/// use pgm_rust::graph::types::edge::Edge;
/// use pgm_rust::graph::types::edgetype::EdgeType;
/// use pgm_rust::graph::types::node::Node;
/// use pgm_rust::graph::ops::setops::difference_edges;
/// use std::collections::HashSet;
///
/// fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge<Node> {
///     Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
/// }
/// fn mk_edge_refs<'a>(es: &'a Vec<Edge<Node>>) -> HashSet<&'a Edge<Node>> {
///     let mut hs = HashSet::new();
///     for e in es {
///         hs.insert(e);
///     }
///     hs
/// }
///
/// let e1 = mk_uedge("n1", "n3", "e1");
/// let e2 = mk_uedge("n20", "n30", "e2");
/// let e3 = mk_uedge("n20", "n40", "e3");
/// let evs1 = vec![e1.clone(), e2.clone(), e3.clone()];
/// let es1 = mk_edge_refs(&evs1);
/// let e4 = mk_uedge("n2", "n3", "e1");
/// let e5 = mk_uedge("n20", "n30", "e2");
/// let e6 = mk_uedge("n20", "n40", "e3");
/// let evs2 = vec![e4.clone(), e5.clone(), e6.clone()];
/// let es2 = mk_edge_refs(&evs2);
/// let es_diff = difference_edges(es1, es2);
/// let mut comp = HashSet::new();
/// comp.insert(&e1);
/// es_diff == comp;
/// ```
pub fn difference_edges<'a, N: NodeTrait, E: EdgeTrait<N>>(
    a1: HashSet<&'a E>,
    a2: HashSet<&'a E>,
) -> HashSet<&'a E> {
    let mut inter = HashSet::new();
    for i in a1.difference(&a2) {
        // instead of moving the reference we copy the reference
        inter.insert(i.clone());
    }
    inter
}
/// difference of graph
pub fn difference<'a, N: NodeTrait, E: EdgeTrait<N>, G: GraphTrait<N, E>>(
    a1: &'a G,
    a2: &'a G,
) -> G {
    //
    let vs1 = a1.vertices();
    let vs2 = a2.vertices();

    let es1 = a1.edges();
    let es2 = a2.edges();
    let vs = difference_nodes(vs1, vs2);
    let es = difference_edges(es1, es2);
    let gid = Uuid::new_v4().to_string();
    G::create_from_ref(gid, HashMap::new(), vs, es)
}

/// symmetric difference
pub fn symmetric_difference_edges<'a, N: NodeTrait, E: EdgeTrait<N>>(
    a1: HashSet<&'a E>,
    a2: HashSet<&'a E>,
) -> HashSet<&'a E> {
    let mut inter = HashSet::new();
    for i in a1.symmetric_difference(&a2) {
        // instead of moving the reference we copy the reference
        inter.insert(i.clone());
    }
    inter
}
/// symmetric difference of nodes
pub fn symmetric_difference_node<'a, T: NodeTrait>(
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
pub fn symmetric_difference<'a, N: NodeTrait, E: EdgeTrait<N>, G: GraphTrait<N, E>>(
    a1: &'a G,
    a2: &'a G,
) -> G {
    //
    let vs1 = a1.vertices();
    let vs2 = a2.vertices();

    let es1 = a1.edges();
    let es2 = a2.edges();
    let vs = symmetric_difference_node(vs1, vs2);
    let es = symmetric_difference_edges(es1, es2);
    let gid = Uuid::new_v4().to_string();
    G::create_from_ref(gid, HashMap::new(), vs, es)
}

/// contains

/// contains of edges
pub fn contains_edges<'a, N: NodeTrait, E: EdgeTrait<N>>(
    a1: HashSet<&'a E>,
    a2: HashSet<&'a E>,
) -> bool {
    a2.is_subset(&a1)
}
/// contains of nodes
pub fn contains_node<'a, T: NodeTrait>(a1: HashSet<&'a T>, a2: HashSet<&'a T>) -> bool {
    a2.is_subset(&a1)
}
/// contains of graph
pub fn contains<'a, N, E, G>(a1: &'a G, a2: &'a G) -> bool
where
    N: NodeTrait,
    E: EdgeTrait<N>,
    G: GraphTrait<N, E>,
{
    //
    let vs1 = a1.vertices();
    let vs2 = a2.vertices();

    let es1 = a1.edges();
    let es2 = a2.edges();
    let has_node = contains_node(vs1, vs2);
    let has_edge = contains_edges(es1, es2);
    has_node && has_edge
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::traits::graph::Graph as GraphTrait;
    use crate::graph::traits::node::VertexSet;
    use crate::graph::types::edge::Edge;
    use crate::graph::types::edgetype::EdgeType;
    use crate::graph::types::graph::Graph;
    use crate::graph::types::node::Node;
    use crate::graph::types::node::Vertices;
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

    fn mk_edge_refs<'a>(es: &'a Vec<Edge<Node>>) -> HashSet<&'a Edge<Node>> {
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
    fn mk_g1() -> Graph<Node, Edge<Node>> {
        let e1 = mk_uedge("n1", "n3", "e1");
        let e2 = mk_uedge("n2", "n3", "e2");
        let e3 = mk_uedge("n2", "n4", "e3");
        let nset = mk_nodes(vec!["n1", "n2", "n3", "n4", "n5"]);
        let h1 = HashMap::new();
        let h2 = mk_edges(vec![e1, e2, e3]);
        Graph::new("g1".to_string(), h1, nset, h2)
    }

    fn mk_g2() -> Graph<Node, Edge<Node>> {
        let e1 = mk_uedge("n1", "n3", "e1");
        let e2 = mk_uedge("n20", "n30", "e2");
        let e3 = mk_uedge("n20", "n40", "e3");
        let nset = mk_nodes(vec!["n1", "n2", "n3", "n20", "n30"]);
        let h1 = HashMap::new();
        let h2 = mk_edges(vec![e1, e2, e3]);
        Graph::new("g2".to_string(), h1, nset, h2)
    }

    #[test]
    fn test_intersection_edge() {
        let e1 = mk_uedge("n1", "n2", "e1");
        let e2 = mk_uedge("n2", "n30", "e2");
        let einter = intersection_edge(&e1, &e2);
        let mut comph = HashSet::new();
        let n2 = mk_node("n2");
        comph.insert(n2);
        let comp = Vertices { vertex_set: comph };
        assert_eq!(einter, comp.members());
    }
    #[test]
    fn test_intersection_edges() {
        let g1 = mk_g1();
        let g1es = g1.edges();
        let e1 = mk_uedge("n1", "n3", "e1");
        let e2 = mk_uedge("n20", "n30", "e2");
        let e3 = mk_uedge("n20", "n40", "e3");
        let evs = vec![e1.clone(), e2, e3];
        let es = mk_edge_refs(&evs);
        let einter = intersection_edges(g1es, es);
        let mut comp = HashSet::new();
        comp.insert(&e1);
        assert_eq!(einter, comp);
    }
    #[test]
    fn test_intersection_nodes() {
        let g1 = mk_g1();
        let g1ns = g1.vertices();
        let n1 = mk_node("n1");
        let n2 = mk_node("n20");
        let n3 = mk_node("n30");
        let nvs = vec![n1.clone(), n2, n3];
        let ns = mk_node_refs(&nvs);
        let ninter = intersection_nodes(g1ns, ns);
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
    // union tests
    #[test]
    fn test_union_edge() {
        let e2 = mk_uedge("n20", "n30", "e2");
        let e3 = mk_uedge("n20", "n40", "e3");
        let eunion = union_edge(&e2, &e3);
        let comp_v = vec![Node::empty("n20"), Node::empty("n30"), Node::empty("n40")];
        let comp = mk_node_refs(&comp_v);
        assert_eq!(eunion, comp);
    }
    #[test]
    fn test_union_edges() {
        let g1 = mk_g1();
        let g1es = g1.edges();
        let e1 = mk_uedge("n1", "n3", "e1");
        let e2 = mk_uedge("n20", "n30", "e2");
        let e3 = mk_uedge("n20", "n40", "e3");
        let evs = vec![e1.clone(), e2.clone(), e3.clone()];
        let es = mk_edge_refs(&evs);
        let eunion = union_edges(g1es.clone(), es);
        let mut comp = HashSet::new();
        for e in g1es {
            comp.insert(e);
        }
        comp.insert(&e2);
        comp.insert(&e3);
        assert_eq!(eunion, comp);
    }
    #[test]
    fn test_union_nodes() {
        let g1 = mk_g1();
        let g1ns = g1.vertices();
        let n1 = mk_node("n1");
        let n2 = mk_node("n20");
        let n3 = mk_node("n30");
        let nvs = vec![n1.clone(), n2.clone(), n3.clone()];
        let ns = mk_node_refs(&nvs);
        let nunion = union_nodes(g1ns.clone(), ns);
        let mut comp = HashSet::new();
        for n in g1ns {
            comp.insert(n);
        }
        comp.insert(&n2);
        comp.insert(&n3);
        assert_eq!(nunion, comp);
    }
    #[test]
    fn test_union_graph() {
        let g1 = mk_g1();
        let g2 = mk_g2();
        let g1uniong2 = union_graph(&g1, &g2);
        let union_v = g1uniong2.vertices();
        let union_e = g1uniong2.edges();
        let mut comp_v = HashSet::new();
        for v in g1.vertices() {
            comp_v.insert(v);
        }
        for v in g2.vertices() {
            comp_v.insert(v);
        }
        let mut comp_e = HashSet::new();
        for e in g1.edges() {
            comp_e.insert(e);
        }
        for e in g2.edges() {
            comp_e.insert(e);
        }
        assert_eq!(union_v, comp_v);
        assert_eq!(union_e, comp_e);
    }

    #[test]
    fn test_difference_edge() {
        let e2 = mk_uedge("n20", "n30", "e2");
        let e3 = mk_uedge("n20", "n40", "e3");
        let ediff = difference_edge(&e2, &e3);
        let comp_v = vec![Node::empty("n30")];
        let comp = mk_node_refs(&comp_v);
        assert_eq!(ediff, comp);
    }
    #[test]
    fn test_difference_edges() {
        let e1 = mk_uedge("n1", "n3", "e1");
        let e2 = mk_uedge("n20", "n30", "e2");
        let e3 = mk_uedge("n20", "n40", "e3");
        let evs1 = vec![e1.clone(), e2.clone(), e3.clone()];
        let es1 = mk_edge_refs(&evs1);
        let e4 = mk_uedge("n2", "n3", "e1");
        let e5 = mk_uedge("n20", "n30", "e2");
        let e6 = mk_uedge("n20", "n40", "e3");
        let evs2 = vec![e4.clone(), e5.clone(), e6.clone()];
        let es2 = mk_edge_refs(&evs2);
        let es_diff = difference_edges(es1, es2);
        let mut comp = HashSet::new();
        comp.insert(&e1);
        assert_eq!(es_diff, comp);
    }
    #[test]
    fn test_difference_nodes() {
        let g1 = mk_g1();
        let g1ns = g1.vertices();
        let n1 = mk_node("n1");
        let n2 = mk_node("n20");
        let n3 = mk_node("n30");
        let nvs = vec![n1.clone(), n2.clone(), n3.clone()];
        let ns = mk_node_refs(&nvs);
        let nunion = difference_nodes(ns, g1ns.clone());
        let mut comp = HashSet::new();
        comp.insert(&n2);
        comp.insert(&n3);
        assert_eq!(nunion, comp);
    }
    //#[test]
    //fn test_difference() {
    //    let g1 = mk_g1();
    //    let g2 = mk_g2();
    //    let g1uniong2 = difference(&g1, &g2);
    //    let difference_v = g1uniong2.vertices();
    //    let difference_e = g1uniong2.edges();
    //    let mut comp_v = HashSet::new();
    //    for v in g1.vertices() {
    //        comp_v.insert(v);
    //    }
    //    for v in g2.vertices() {
    //        comp_v.insert(v);
    //    }
    //    let mut comp_e = HashSet::new();
    //    for e in g1.edges() {
    //        comp_e.insert(e);
    //    }
    //    for e in g2.edges() {
    //        comp_e.insert(e);
    //    }
    //    assert_eq!(difference_v, comp_v);
    //    assert_eq!(difference_e, comp_e);
    //}
}
