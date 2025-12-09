//! A base graph which implements the Graph trait for doing graph theoretical
//! operations

use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::edge::EdgeSet as EdgeSetTrait;
use crate::graph::traits::graph::Graph as GraphTrait;
use crate::graph::traits::graph_obj::GraphObject as GraphObjectTrait;
use crate::graph::traits::node::Node as NodeTrait;
use crate::graph::traits::node::VertexSet as VertexSetTrait;
use crate::graph::traits::path::Path as PathTrait;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// checks if containers has two members or less
fn has_two_or_less<N: NodeTrait>(nodes: &Vec<&N>) {
    let c1 = nodes.len() <= 2;
    if !c1 {
        panic!("nodes have more than 2 elements")
    } else {
    }
}

fn extract_two_nodes<N: NodeTrait>(nodes: &Vec<&N>) -> (N, N) {
    let arr: [&N; 2] = nodes[0..2].try_into().unwrap();
    (arr[0].clone(), arr[1].clone())
}

/// Output nodes of the argument edges with different groupings
fn get_end_vertices_and_nodes<N, E>(edges: Vec<E>) -> (Vec<N>, HashSet<N>, (N, N))
where
    N: NodeTrait,
    E: EdgeTrait<N>,
{
    let mut ns: Vec<N> = Vec::new();
    let e_opt = edges.get(0);
    match e_opt {
        None => panic!("empty edge list"),
        Some(e) => {
            let e_start: &N = e.start();
            ns.push(e_start.clone());
        }
    }
    //
    let mut nodes: HashSet<&N> = HashSet::new();
    let mut snodes: HashSet<&N> = HashSet::new();
    let mut enodes: HashSet<&N> = HashSet::new();
    for e in &edges {
        let e_start: &N = e.start();
        let e_end: &N = e.end();
        snodes.insert(e_start);
        enodes.insert(e_end);
        nodes.insert(e_start);
        nodes.insert(e_end);

        let has_not_end = !ns.contains(&e_end);
        if has_not_end {
            ns.push(e_end.clone());
        }
    }
    let node_lst = ns;
    let node_set: HashSet<N> = nodes.iter().map(|&x| x.clone()).collect();
    let end_nodes: Vec<&N> = enodes
        .clone()
        .into_iter()
        .filter(|n| !snodes.contains(n))
        .collect();
    let mut start_nodes: Vec<&N> = snodes.into_iter().filter(|n| !enodes.contains(n)).collect();
    has_two_or_less(&end_nodes);
    has_two_or_less(&start_nodes);
    if start_nodes.len() == 2 {
        let start_end = extract_two_nodes(&start_nodes);
        (node_lst, node_set, start_end)
    } else if end_nodes.len() == 2 {
        let start_end = extract_two_nodes(&end_nodes);
        (node_lst, node_set, start_end)
    } else {
        start_nodes.extend(end_nodes.into_iter());
        let start_end = extract_two_nodes(&start_nodes);
        (node_lst, node_set, start_end)
    }
}

/// path is essentially a graph
/// path object as defined in Diestel 2017, p. 6
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Path<N: NodeTrait, E: EdgeTrait<N>> {
    /// edges of the path graph
    gdata: HashSet<E>,
    /// graph identifier required for [GraphObject] trait
    graph_id: String,
    /// graph data required for [GraphObject] trait
    graph_data: HashMap<String, Vec<String>>,
}

/// Path objects are hashed using their graphs
impl<T: NodeTrait, E: EdgeTrait<T>, G: GraphTrait<T, E> + GraphObjectTrait> Hash for Path<T, E, G> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.graph.hash(state);
    }
}

/// Path objects display their identifier when serialized to string.
impl<N: NodeTrait, E: EdgeTrait<N>> fmt::Display for Path<N, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nid = &self.graph.id();
        write!(f, "<Path id='{}'>", nid)
    }
}

impl<T: NodeTrait, E: EdgeTrait<T>, G: GraphTrait<T, E> + GraphObjectTrait> GraphObjectTrait
    for Path<T, E, G>
{
    fn id(&self) -> &str {
        &self.graph_id
    }

    fn data(&self) -> &HashMap<String, Vec<String>> {
        &self.graph.data()
    }
}

impl<T: NodeTrait, E: EdgeTrait<T> + Clone, G: GraphTrait<T, E> + GraphObjectTrait> GraphTrait<T, E>
    for Path<T, E, G>
{
    fn vertices(&self) -> HashSet<&T> {
        self.graph.vertices()
    }
    fn edges(&self) -> HashSet<&E> {
        self.graph.edges()
    }
    fn create(
        graph_id: String,
        graph_data: HashMap<String, Vec<String>>,
        nodes: HashSet<T>,
        edges: HashSet<E>,
    ) -> Path<T, E, G> {
        let graph = G::create(graph_id, graph_data, nodes, edges.clone());
        let edges: Vec<E> = edges.iter().map(|x| x.clone()).collect();
        let group = get_end_vertices_and_nodes::<T, E>(edges);
        let (_, _, (start, end)) = group;
        Path {
            graph: graph,
            ends: (start, end),
            edge_type: PhantomData,
        }
    }
    fn create_from_ref(
        graph_id: String,
        graph_data: HashMap<String, Vec<String>>,
        nodes: HashSet<&T>,
        edges: HashSet<&E>,
    ) -> Path<T, E, G> {
        let graph = G::create_from_ref(graph_id, graph_data, nodes, edges.clone());
        let edges: Vec<E> = edges.iter().map(|&x| x.clone()).collect();
        let group = get_end_vertices_and_nodes::<T, E>(edges);
        let (_, _, (start, end)) = group;
        Path {
            graph: graph,
            ends: (start, end),
            edge_type: PhantomData,
        }
    }
}

impl<T: NodeTrait, E: EdgeTrait<T> + Clone, G: GraphTrait<T, E> + GraphObjectTrait> PathTrait<T, E>
    for Path<T, E, G>
{
    /// number of edges inside the path, see Diestel 2017, p. 6
    fn length(&self) -> usize {
        self.graph.edges().len()
    }

    /// end nodes of path
    fn endvertices(&self) -> (&T, &T) {
        let (e1, e2) = &self.ends;
        (e1, e2)
    }
}

#[cfg(test)]
mod tests {

    use super::*; // brings in the parent scope to current module scope
    use crate::graph::types::edge::Edge;
    use crate::graph::types::graph::Graph;
    use crate::graph::types::node::Node;

    // mk node
    fn mk_node(n_id: &str) -> Node {
        Node::new(n_id.to_string(), HashMap::new())
    }

    fn mk_nodes(ns: Vec<&str>) -> HashSet<Node> {
        let mut h: HashSet<Node> = HashSet::new();
        for n in ns {
            h.insert(mk_node(n));
        }
        h
    }

    // mk edge
    fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge<Node> {
        let n1 = mk_node(n1_id);
        let n2 = mk_node(n2_id);
        let mut h1 = HashMap::new();
        h1.insert(String::from("my"), vec![String::from("data")]);
        Edge::undirected(e_id.to_string(), n1, n2, h1)
    }

    /// make a path
    /// n1 - n2 - n3 - n4 - n5 - n6 - n7
    fn mk_path() -> Path<Node, Edge<Node>, Graph<Node, Edge<Node>>> {
        let ns = mk_nodes(vec!["n1", "n2", "n3", "n4", "n5", "n6", "n7"]);
        let e1 = mk_uedge("n1", "n2", "e1");
        let e2 = mk_uedge("n2", "n3", "e2");
        let e3 = mk_uedge("n3", "n4", "e3");
        let e4 = mk_uedge("n4", "n5", "e4");
        let e5 = mk_uedge("n5", "n6", "e5");
        let e6 = mk_uedge("n6", "n7", "e6");
        let es = HashSet::from([e1, e2, e3, e4, e5, e6]);
        let p = Path::create("mpath".to_string(), HashMap::new(), ns, es);
        p
    }
    #[test]
    fn test_id() {
        let p = mk_path();
        assert_eq!(p.id(), "mpath");
    }

    #[test]
    fn test_length() {
        let p = mk_path();
        assert_eq!(p.length(), 6);
    }

    #[test]
    fn test_endvertices() {
        let p = mk_path();
        let n1 = mk_node("n1");
        let n7 = mk_node("n7");
        assert_eq!(p.ends, (n1, n7));
    }
}
