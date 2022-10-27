//! functions that has a graph among its arguments that output a value

use crate::graph::ops::edge::miscops::node_ids;
use crate::graph::traits::graph::Graph;
use std::collections::HashMap;
use std::option::Option;
use std::vec::Vec;

/// create an edge list representation of graph

/// for each node we register all the edges
pub fn to_edgelist<G: Graph>(g: &G) -> HashMap<String, Option<Vec<String>>> {
    let mut elist: HashMap<String, Option<Vec<String>>> = HashMap::new();
    for node in g.vertices() {
        let mut n_es: Vec<String> = Vec::new();
        for edge in g.edges() {
            let n_ids = node_ids(edge);
            if n_ids.contains(node.id()) {
                n_es.push(edge.id().clone());
            }
        }
        let nid = node.id().clone();
        if n_es.is_empty() {
            elist.insert(nid, None);
        } else {
            elist.insert(nid, Some(n_es));
        }
    }
    elist
}

#[cfg(test)]
mod tests {

    #[ignore]
    #[test]
    fn test_to_edgelist() {}

    #[ignore]
    #[test]
    fn test_to_adjmat() {}

    #[ignore]
    #[test]
    fn test_get_subgraph_by_vertices() {}
}
