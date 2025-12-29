// edge type

use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::edge::EdgeSet as EdgeSetTrait;
use crate::graph::traits::graph_obj::GraphObject;

use crate::graph::traits::generic::default_with_hash_partial_eq_impl;
use crate::graph::traits::node::Node as NodeTrait;
use crate::graph::traits::utils::from_borrowed_data;
use crate::graph::traits::utils::to_borrowed_data;
use crate::graph::types::edgetype::EdgeType;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::marker::PhantomData;

/// Edge object.
/// Formally defined as set with two elements, see Diestel 2017, p. 2
#[derive(Debug, Clone)]
pub struct Edge<T: NodeTrait> {
    _id: String,
    _data: HashMap<String, Vec<String>>,
    edge_type: EdgeType,
    start_node: T,
    end_node: T,
}
default_with_hash_partial_eq_impl!(Edge);

/// short hand for edge set
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Edges<N: NodeTrait, E: EdgeTrait<N>> {
    /// edge set content
    pub edge_set: HashSet<E>,
    node_type: PhantomData<N>,
}

impl<N: NodeTrait, E: EdgeTrait<N> + Clone> EdgeSetTrait<N, E> for Edges<N, E> {
    fn members(&self) -> HashSet<&E> {
        let mut es: HashSet<&E> = HashSet::new();
        for e in &self.edge_set {
            es.insert(&e);
        }
        es
    }
    fn create(hset: HashSet<&E>) -> Self {
        let mut es: HashSet<E> = HashSet::new();
        for e in hset {
            es.insert(e.clone());
        }
        Edges {
            edge_set: es,
            node_type: PhantomData,
        }
    }
}

impl<T: NodeTrait> fmt::Display for Edge<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let eid = &self.info.id;
        let n1 = &self.start_node;
        let n2 = &self.end_node;
        let et = &self.info.edge_type;
        write!(
            f,
            "<Edge id='{}' type='{}'><start>{}</start><end>{}</end></Edge>",
            eid, et, n1, n2
        )
    }
}

impl<T: NodeTrait> GraphObject for Edge<T> {
    fn null() -> Edge<T> {
        let s = T::null();
        let e = T::null();
        let info = EdgeInfo::null();
        Edge {
            _id: String::from_str(""),
            _data: HashMap::new(),
            edge_type: EdgeType::Undirected,
            start_node: s,
            end_node: e,
        }
    }
}

impl<NodeType: NodeTrait> EdgeTrait<NodeType> for Edge<NodeType> {
    fn start(&self) -> &NodeType {
        &self.start_node
    }
    fn end(&self) -> &NodeType {
        &self.end_node
    }
    fn has_type(&self) -> &EdgeType {
        &self.info.edge_type
    }
    fn create(
        eid: String,
        e_data: HashMap<String, Vec<String>>,
        snode: NodeType,
        enode: NodeType,
        etype: EdgeType,
    ) -> Edge<NodeType> {
        Edge::from_info(eid, e_data, etype, snode, enode)
    }
}

impl<T: NodeTrait> Edge<T> {
    /// edge constructor
    pub fn new(
        eid: String,
        e_data: HashMap<String, Vec<String>>,
        etype: EdgeType,
        snode: T,
        enode: T,
    ) -> Edge<T> {
        Edge {
            _id: eid,
            _data: e_data,
            edge_type: etype,
            start_node: snode,
            end_node: enode,
        }
    }

    /// undirected edge constructor
    pub fn undirected(
        eid: String,
        snode: T,
        enode: T,
        e_data: HashMap<String, Vec<String>>,
    ) -> Edge<T> {
        Edge::new(eid, e_data, EdgeType::Undirected, snode, enode)
    }
    /// directed edge constructor
    pub fn directed(
        eid: String,
        snode: T,
        enode: T,
        e_data: HashMap<String, Vec<String>>,
    ) -> Edge<T> {
        Edge::new(eid, e_data, EdgeType::Directed, snode, enode)
    }
    /// a generic constructor for edge like objects with burrowing
    pub fn from_edgish_ref<E: EdgeTrait<T>>(e: &E) -> Edge<T> {
        let eid = e.id().to_string();
        let e_data = from_borrowed_data(&e.data());
        let etype = e.has_type().clone();
        let snode = e.start().clone();
        let enode = e.end().clone();
        Edge::new(eid, e_data, etype, snode, enode)
    }
    /// a generic constructor for edge like objects with move
    pub fn from_edgish<E: EdgeTrait<T>>(e: E) -> Edge<T> {
        Edge::from_edgish_ref(&e)
    }
    /// empty edge constructor.
    pub fn empty() -> Edge<T> {
        Edge::null()
    }

    /// construct edge using identifiers
    pub fn from_ids(eid: &str, etype: EdgeType, start_id: &str, end_id: &str) -> Edge<T> {
        let mut e = Edge::null().set_id(eid);
        e.edge_type = etype;
        let start_n = T::null().set_id(start_id);
        e.start_node = start_n;
        let end_n = T::null().set_id(end_id);
        e.end_node = end_n;
        e
    }
}
#[cfg(test)]
mod tests {

    use super::*; // brings in the parent scope to current module scope
    use crate::graph::types::node::Node;

    fn create_mock_node(idstr: &str) -> Node {
        Node::from_id(idstr)
    }

    fn setup_test_edge() -> Edge<Node> {
        let start_node = Node::from_id("n1");
        let end_node = create_mock_node("n2");
        let mut data = HashMap::new();
        data.insert("value".to_string(), vec!["1.5".to_string()]);

        Edge::from_info(
            "e_test".to_string(),
            data,
            EdgeType::Directed,
            start_node,
            end_node,
        )
    }

    // --- EDGE<T> TESTS ---

    #[test]
    fn test_edge_new_and_from_info_constructors() {
        let s_node = create_mock_node("s");
        let e_node = create_mock_node("e");
        let data = HashMap::from([("key".to_string(), vec!["val".to_string()])]);

        // Test from_info
        let edge = Edge::from_info(
            "e_f".to_string(),
            data.clone(),
            EdgeType::Undirected,
            s_node.clone(),
            e_node.clone(),
        );

        assert_eq!(edge.info.id, "e_f");
        assert_eq!(edge.info.edge_type, EdgeType::Undirected);
        assert_eq!(edge.start_node.id(), "s");
        assert_eq!(edge.end_node.id(), "e");

        // Test new (using existing EdgeInfo)
        let info = EdgeInfo::new("e_n".to_string(), data.clone(), EdgeType::Directed);
        let edge_new = Edge::new(info, s_node.clone(), e_node.clone());

        assert_eq!(edge_new.info.id, "e_n");
        assert_eq!(edge_new.info.edge_type, EdgeType::Directed);
    }

    #[test]
    fn test_edge_undirected_and_directed_constructors() {
        let s_node = create_mock_node("s");
        let e_node = create_mock_node("e");
        let data = HashMap::new();

        let directed_edge = Edge::directed(
            "e_d".to_string(),
            s_node.clone(),
            e_node.clone(),
            data.clone(),
        );
        assert_eq!(directed_edge.info.id, "e_d");
        assert_eq!(directed_edge.info.edge_type, EdgeType::Directed);

        let undirected_edge = Edge::undirected("e_u".to_string(), s_node, e_node, data);
        assert_eq!(undirected_edge.info.id, "e_u");
        assert_eq!(undirected_edge.info.edge_type, EdgeType::Undirected);
    }

    #[test]
    fn test_edge_trait_methods() {
        let edge = setup_test_edge();

        assert_eq!(edge.start().id(), "n1");
        assert_eq!(edge.end().id(), "n2");
        assert_eq!(edge.has_type(), &EdgeType::Directed);
    }

    #[test]
    fn test_edge_graph_object_id_and_data() {
        let edge = setup_test_edge();

        assert_eq!(edge.id(), "e_test");
        let borrowed_data = edge.data();
        assert_eq!(borrowed_data.get("value").unwrap(), &vec!["1.5"]);
    }

    #[test]
    fn test_edge_graph_object_null_and_empty() {
        let null_edge = Edge::<Node>::null();
        assert_eq!(null_edge.id(), "");
        assert_eq!(null_edge.start_node.id(), "");

        let empty_edge = Edge::<Node>::empty();
        assert_eq!(empty_edge.id(), "");
    }

    #[test]
    fn test_edge_graph_object_set_id() {
        let edge = setup_test_edge();
        let new_id = "e_updated";
        let new_edge = edge.set_id(new_id);

        assert_eq!(new_edge.id(), new_id);
        // Check that other fields are cloned correctly
        assert_eq!(new_edge.start_node.id(), edge.start_node.id());
        assert_eq!(new_edge.info.edge_type, edge.info.edge_type);
    }

    #[test]
    fn test_edge_graph_object_set_data() {
        let edge = setup_test_edge();
        let new_data = HashMap::from([("new_key", vec!["new_val"])]);
        let new_edge = edge.set_data(new_data.clone());

        assert_eq!(new_edge.id(), edge.id());
        let updated_data = new_edge.data();
        assert_eq!(updated_data.get("new_key").unwrap(), &vec!["new_val"]);
        assert!(!updated_data.contains_key("value")); // Old data overwritten
    }

    #[test]
    fn test_edge_equality_ignores_nodes_and_data() {
        let s_node1 = create_mock_node("n1");
        let e_node1 = create_mock_node("n2");
        let s_node2 = create_mock_node("n3"); // Different nodes
        let e_node2 = create_mock_node("n4");

        let mut data1 = HashMap::new();
        data1.insert("color".to_string(), vec!["red".to_string()]);

        let mut data2 = HashMap::new();
        data2.insert("color".to_string(), vec!["blue".to_string()]); // Different data

        // Edge A: Directed, n1->n2
        let edge_a = Edge::from_info(
            "id_x".to_string(),
            data1.clone(),
            EdgeType::Directed,
            s_node1.clone(),
            e_node1.clone(),
        );

        // Edge B: Undirected, n3->n4 (Different nodes and type, same ID)
        let edge_b = Edge::from_info(
            "id_x".to_string(),
            data2.clone(),
            EdgeType::Undirected,
            s_node2.clone(),
            e_node2.clone(),
        );

        // Edge C: Directed, n1->n2 (Same nodes/type, different ID)
        let edge_c = Edge::from_info(
            "id_y".to_string(),
            data1,
            EdgeType::Directed,
            s_node1.clone(),
            e_node1.clone(),
        );

        // A == B because IDs are the same
        assert_eq!(edge_a, edge_b);

        // A != C because IDs are different
        assert_ne!(edge_a, edge_c);
    }

    #[test]
    fn test_edge_display_format() {
        let edge = setup_test_edge();

        let expected =
            "<Edge id='e_test' type='Directed'><start><Node id='n1'/></start><end><Node id='n2'/></end></Edge>";

        assert_eq!(format!("{}", edge), expected);
    }

    #[test]
    fn test_edge_from_ids() {
        let edge = Edge::<Node>::from_ids("e_id", EdgeType::Undirected, "s_id", "e_id");

        assert_eq!(edge.id(), "e_id");
        assert_eq!(edge.start_node.id(), "s_id");
        assert_eq!(edge.end_node.id(), "e_id");
        assert_eq!(edge.info.edge_type, EdgeType::Undirected);
    }

    // --- EDGES<N, E> TESTS (Edge Set) ---

    #[test]
    fn test_edges_create_and_size() {
        let edge1 = setup_test_edge();
        let edge2 = setup_test_edge().set_id("e_2");
        let edge3 = setup_test_edge().set_id("e_3");

        let mut hset: HashSet<&Edge<Node>> = HashSet::new();
        hset.insert(&edge1);
        hset.insert(&edge2);
        hset.insert(&edge3);

        let edges_set = Edges::create(hset);

        assert_eq!(edges_set.edge_set.len(), 3);

        // Check for cloning during creation
        let edge1_in_set = edges_set
            .edge_set
            .iter()
            .find(|e| e.id() == "e_test")
            .unwrap();
        assert_eq!(edge1_in_set.id(), "e_test");
    }

    #[test]
    fn test_edges_members_returns_references() {
        let edge1 = setup_test_edge();
        let edge2 = setup_test_edge().set_id("e_2");

        let mut hset: HashSet<&Edge<Node>> = HashSet::new();
        hset.insert(&edge1);
        hset.insert(&edge2);
        let edges_set = Edges::create(hset);

        // Test the members method
        let members = edges_set.members();

        assert_eq!(members.len(), 2);

        // Verify that the references point to the correct items
        assert!(members.iter().any(|&e| e.id() == "e_test"));
        assert!(members.iter().any(|&e| e.id() == "e_2"));
    }

    /// Test initialization via the `new` constructor.
    #[test]
    fn test_edge_info_new_initializes_correctly() {
        let edge_id = "e_123".to_string();
        let edge_type = EdgeType::Directed;
        let mut data = HashMap::new();
        data.insert("color".to_string(), vec!["blue".to_string()]);

        let edge = EdgeInfo::new(edge_id.clone(), data.clone(), edge_type.clone());

        assert_eq!(edge.id, edge_id);
        assert_eq!(edge.edge_type, edge_type);
        assert!(edge.data.contains_key("color"));
        assert_eq!(edge.data.get("color").unwrap(), &vec!["blue".to_string()]);
    }

    /// Test the `null` constructor for default/empty values.
    #[test]
    fn test_edge_info_null_creates_default_instance() {
        let null_edge = EdgeInfo::null();

        assert_eq!(null_edge.id, "", "Null edge ID should be an empty string.");
        assert!(
            null_edge.data.is_empty(),
            "Null edge data map should be empty."
        );
        assert_eq!(
            null_edge.edge_type,
            EdgeType::Undirected,
            "Null edge type should be Undirected."
        );
    }

    /// Test the `PartialEq` implementation, which relies only on the `id`.
    #[test]
    fn test_edge_info_equality_ignores_data_and_type() {
        // Edge 1: Full data, Directed
        let mut data_1 = HashMap::new();
        data_1.insert("weight".to_string(), vec!["10".to_string()]);
        let edge_a = EdgeInfo::new("same_id".to_string(), data_1, EdgeType::Directed);

        // Edge 2: Different data, Different type
        let mut data_2 = HashMap::new();
        data_2.insert("weight".to_string(), vec!["5".to_string()]);
        let edge_b = EdgeInfo::new("same_id".to_string(), data_2, EdgeType::Undirected);

        // Edge 3: Different ID
        let edge_c = EdgeInfo::new(
            "different_id".to_string(),
            HashMap::new(),
            EdgeType::Directed,
        );

        // A and B should be equal because their IDs are the same
        assert_eq!(
            edge_a, edge_b,
            "Edges with the same ID but different data/type should be equal."
        );

        // A and C should be unequal because their IDs are different
        assert_ne!(
            edge_a, edge_c,
            "Edges with different IDs should not be equal."
        );
    }
}
