///
use crate::graph::traits::graph::Graph;
use crate::graph::traits::node::Node;
use std::collections::HashSet;

/// for directed edges we assume neighbor is 'm in n -> m'.
/// for undirected edges we assume neighbor is 'm,k in k -> n & n -> m'
pub fn neighbors_of<G, N>(g: &'a G, n: &'a N) -> HashSet<&'a Node>
where
    G: Graph,
    N: Node,
{
    let mut neighbors = HashSet::new();

    // check is in
}

#[cfg(tests)]
mod tests {

    #[ignore]
    #[test]
    fn test_get_nodes() {}

    #[ignore]
    #[test]
    fn test_vertex_by_id() {}

    #[ignore]
    #[test]
    fn test_vertices_of() {}

    #[ignore]
    #[test]
    fn test_neighbors_of() {}
}
