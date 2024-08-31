// graph trait
use crate::graph::traits::edge::Edge;
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::traits::node::Node;
use std::collections::HashMap;
use std::collections::HashSet;

/// Promotes an object to being a graph.
/// This trait is the gateway for using all the graph related operations in
/// the library
pub trait Graph<NodeType: Node, EdgeType: Edge<NodeType>>: GraphObject {
    /// outputs a [Node] set.
    /// a [Node] can constructed anything that implements the Node trait
    fn vertices(&self) -> HashSet<&NodeType>;

    /// outputs an [Edge] set.
    /// an [Edge] can constructed anything that implements the Edge trait
    fn edges(&self) -> HashSet<&EdgeType>;

    /// create graph from edges and vertices
    fn create(
        _: String,
        _: HashMap<String, Vec<String>>,
        _: HashSet<NodeType>,
        _: HashSet<EdgeType>,
    ) -> Self;

    /// create graph from edge and vertex references
    fn create_from_ref(
        _: String,
        _: HashMap<String, Vec<String>>,
        _: HashSet<&NodeType>,
        _: HashSet<&EdgeType>,
    ) -> Self;

    /// create vertex id: vertex map
    fn vmap(&self) -> HashMap<String, &NodeType> {
        let vs = self.vertices();
        self._idmap::<NodeType>(vs)
    }
    fn _idmap<'a, T: GraphObject>(&'a self, ts: HashSet<&'a T>) -> HashMap<String, &T> {
        let mut hs: HashMap<String, &'a T> = HashMap::new();
        for v in ts {
            let id: String = v.id().to_string();
            hs.insert(id, v);
        }
        hs
    }
    /// create vertex id: vertex map
    fn emap(&self) -> HashMap<String, &EdgeType> {
        let vs = self.edges();
        self._idmap::<EdgeType>(vs)
    }
}
//
