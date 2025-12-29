// graph node

// call the GraphObject trait
use crate::graph::traits::generic::default_all_impl;
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::traits::node::Node as NodeTrait;
use crate::graph::traits::node::VertexSet as VertexSetTrait;

// call the utilities
use crate::graph::traits::utils::from_borrowed_data;
use crate::graph::traits::utils::to_borrowed_data;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Node object.
/// Formally defined as a member/point/vertex of a graph, see Diestel 2017, p.2
#[derive(Debug, Clone)]
pub struct Node {
    _id: String,
    _data: HashMap<String, Vec<String>>,
}
default_all_impl!(Node);

/// Short hand for set of nodes
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Vertices<N: NodeTrait> {
    /// node set field
    pub vertex_set: HashSet<N>,
}
impl<N: NodeTrait> VertexSetTrait<N> for Vertices<N> {
    fn members(&self) -> HashSet<&N> {
        let mut ms: HashSet<&N> = HashSet::new();
        for v in &self.vertex_set {
            ms.insert(v);
        }
        ms
    }
    fn create(vs: HashSet<&N>) -> Self {
        let mut ms: HashSet<N> = HashSet::new();
        for v in &vs {
            let m: &N = v;
            ms.insert(m.clone());
        }
        Vertices { vertex_set: ms }
    }
}

impl Node {
    /// constructor for Node object
    pub fn new(nid: String, ndata: HashMap<String, Vec<String>>) -> Node {
        Node {
            _id: nid,
            _data: ndata,
        }
    }
    /// constructor for node like objects that implement node trait with borrowing
    pub fn from_nodish_ref<T: NodeTrait>(n: &T) -> Node {
        let ndata = n.data();
        let data = from_borrowed_data(&ndata);
        Node {
            _id: n.id().to_string(),
            _data: data,
        }
    }

    /// constructor for node like objects that implement node trait with move
    pub fn from_nodish<T: NodeTrait>(n: T) -> Node {
        Node::from_nodish_ref(&n)
    }
    /// empty constructor
    pub fn from_id(nid: &str) -> Node {
        let ndata: HashMap<String, Vec<String>> = HashMap::new();
        Node {
            _id: nid.to_string(),
            _data: ndata,
        }
    }
}

impl GraphObject for Node {
    fn null() -> Node {
        let nid = String::from("");
        Node::from_id(&nid)
    }
}

impl NodeTrait for Node {
    fn create(nid: String, ndata: HashMap<String, Vec<String>>) -> Node {
        Node::new(nid, ndata)
    }
}

#[cfg(test)]
mod tests {

    use super::*; // brings in the parent scope to current module scope

    // A dummy struct that implements NodeTrait, necessary for testing generic functions.
    #[derive(Debug, PartialEq, Eq, Clone, Hash)]
    struct DummyNode {
        id: String,
        is_mock: bool,
    }
    impl fmt::Display for DummyNode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "<DummyNode id='{}'/>", self.id)
        }
    }

    impl Identified for DummyNode {
        fn id(&self) -> &str {
            &self.id
        }
    }
    impl Loaded for DummyNode {
        fn data(&self) -> HashMap<&str, Vec<&str>> {
            HashMap::new()
        } // Simplification for mock
    }
    impl Loaded for DummyNode {
        fn null() -> Self {
            DummyNode {
                id: "".to_string(),
                is_mock: true,
            }
        }
    }
    impl IdChanger for DummyNode {
        fn set_id(&self, idstr: &str) -> Self {
            DummyNode {
                id: idstr.to_string(),
                is_mock: self.is_mock,
            }
        }
    }
    impl LoadChanger for DummyNode {
        fn set_data(&self, _: HashMap<&str, Vec<&str>>) -> Self {
            self.clone()
        }
    }

    impl NodeTrait for DummyNode {
        fn create(nid: String, _ndata: HashMap<String, Vec<String>>) -> DummyNode {
            DummyNode {
                id: nid,
                is_mock: false,
            }
        }
    }

    fn sample_data() -> HashMap<String, Vec<String>> {
        HashMap::from([
            ("meta".to_string(), vec!["version".to_string()]),
            (
                "info".to_string(),
                vec!["user_a".to_string(), "active".to_string()],
            ),
        ])
    }

    fn sample_borrowed_data() -> HashMap<&'static str, Vec<&'static str>> {
        HashMap::from([
            ("meta", vec!["version"]),
            ("info", vec!["user_a", "active"]),
        ])
    }

    fn create_sample_node() -> Node {
        Node::new("n1".to_string(), sample_data())
    }

    // --- NODE CONSTRUCTOR TESTS ---

    #[test]
    fn test_node_new() {
        let node = Node::new("n_test".to_string(), sample_data());
        assert_eq!(node.node_id, "n_test");
        assert_eq!(node.node_data, sample_data());
    }

    #[test]
    fn test_node_from_id() {
        let node = Node::from_id("n_empty");
        assert_eq!(node.node_id, "n_empty");
        assert!(node.node_data.is_empty());
    }

    // --- NODE TRAIT CONSTRUCTOR TESTS ---

    #[test]
    fn test_node_from_nodish_ref() {
        let dummy = DummyNode {
            id: "d1".to_string(),
            is_mock: true,
        };
        // The data() method in DummyNode returns an empty map, so we expect an empty data map in Node
        let node = Node::from_nodish_ref(&dummy);

        assert_eq!(node.node_id, "d1");
        assert!(node.node_data.is_empty());
    }

    #[test]
    fn test_node_from_nodish() {
        let dummy = DummyNode {
            id: "d2".to_string(),
            is_mock: false,
        };
        let node = Node::from_nodish(dummy); // Test with move
        assert_eq!(node.node_id, "d2");
    }

    // --- GRAPH OBJECT TRAIT IMPLEMENTATION TESTS (Node) ---

    #[test]
    fn test_node_id() {
        let node = create_sample_node();
        assert_eq!(node.id(), "n1");
    }

    #[test]
    fn test_node_data() {
        let node = create_sample_node();
        let borrowed_data = node.data();
        assert_eq!(borrowed_data, sample_borrowed_data());
    }

    #[test]
    fn test_node_null() {
        let null_node = Node::null();
        assert_eq!(null_node.node_id, "");
        assert!(null_node.node_data.is_empty());
    }

    #[test]
    fn test_node_set_id() {
        let original = create_sample_node();
        let modified = original.set_id("n_modified");

        assert_eq!(modified.id(), "n_modified");
        assert_eq!(
            modified.node_data, original.node_data,
            "Data should be cloned"
        );
        assert_ne!(original.id(), modified.id(), "IDs must differ");
    }

    #[test]
    fn test_node_set_data() {
        let original = create_sample_node();
        let new_borrowed_data = HashMap::from([("config", vec!["true"])]);

        let modified = original.set_data(new_borrowed_data);

        // Check new data (owned version of the input borrowed data)
        assert_eq!(
            modified.node_data,
            HashMap::from([("config".to_string(), vec!["true".to_string()]),])
        );

        // ID must be preserved
        assert_eq!(modified.id(), original.id());
    }

    // --- DISPLAY, HASH, AND EQUALITY TESTS (Node) ---

    #[test]
    fn test_node_display() {
        let node = Node::from_id("n_fmt");
        assert_eq!(format!("{}", node), "<Node id='n_fmt'/>");
    }

    #[test]
    fn test_node_hash_eq_based_on_id() {
        // Two nodes with the same ID but different data
        let data1 = HashMap::from([("k".to_string(), vec!["v1".to_string()])]);
        let data2 = HashMap::from([("k".to_string(), vec!["v2".to_string()])]);

        let node1 = Node::new("same_id".to_string(), data1.clone());
        let node2 = Node::new("same_id".to_string(), data2.clone());
        let node3 = Node::new("diff_id".to_string(), data1.clone());

        // 1. Equality check (MUST FAIL: node_data differs)
        // This test now PASSES because node1 != node2 according to the new PartialEq impl.
        assert_eq!(
            node1, node2,
            "Nodes with same ID but different data must be equal."
        );
        assert_ne!(
            node1, node3,
            "Nodes with different ID but same data must not be equal."
        );

        // We check the hashing behavior, which should still be ID-only
        let mut hasher1 = std::collections::hash_map::DefaultHasher::new();
        node1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = std::collections::hash_map::DefaultHasher::new();
        node2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        // 2. Hashing check (MUST PASS: Hash only uses ID)
        assert_eq!(
            hash1, hash2,
            "Hashing must be identical because IDs are identical."
        );

        // 3. Check HashSet behavior (Should hold both because node1 != node2)
        let mut set = HashSet::new();
        set.insert(node1.clone());
        set.insert(node2.clone());

        // The set should contain both items because they are not equal, even though their hashes collide.
        assert_eq!(
            set.len(),
            1,
            "Set should contain both elements due to full state comparison in PartialEq."
        );
    }

    // --- VERTICES STRUCT TESTS ---

    #[test]
    fn test_vertices_members() {
        let n1 = Node::from_id("v1");
        let n2 = Node::from_id("v2");
        let mut initial_set = HashSet::new();
        initial_set.insert(n1.clone());
        initial_set.insert(n2.clone());

        let vertices = Vertices {
            vertex_set: initial_set,
        };
        let members = vertices.members();

        // Check that the member count is correct
        assert_eq!(members.len(), 2);

        // Check that the returned references point to the original nodes
        assert!(members.contains(&&n1));
        assert!(members.contains(&&n2));
    }

    #[test]
    fn test_vertices_create() {
        let d1 = DummyNode {
            id: "d_a".to_string(),
            is_mock: false,
        };
        let d2 = DummyNode {
            id: "d_b".to_string(),
            is_mock: false,
        };

        let mut refs_set = HashSet::new();
        refs_set.insert(&d1);
        refs_set.insert(&d2);

        let vertices = Vertices::<DummyNode>::create(refs_set);

        // Check size
        assert_eq!(vertices.vertex_set.len(), 2);

        // Check if the members are cloned copies
        assert!(vertices.vertex_set.contains(&d1));
        assert!(vertices.vertex_set.contains(&d2));
    }

    #[test]
    fn test_id() {
        let my_node = Node {
            _id: String::from("mnode"),
            _data: HashMap::new(),
        };
        assert_eq!(my_node.id(), &String::from("mnode"));
    }
    #[test]
    fn test_data() {
        let mut my_map: HashMap<String, Vec<String>> = HashMap::new();
        let myv = vec![
            String::from("awesome"),
            String::from("string"),
            String::from("stuff"),
        ];
        my_map.insert(String::from("my"), myv);
        let my_node = Node {
            _id: String::from("mnode"),
            _data: my_map,
        };
        let mut my_map2: HashMap<&str, Vec<&str>> = HashMap::new();
        let myv2 = vec!["awesome", "string", "stuff"];
        my_map2.insert("my", myv2);

        assert_eq!(my_node.data(), my_map2);
    }

    #[test]
    fn test_from_nodish_ref() {
        let mut my_map: HashMap<String, Vec<String>> = HashMap::new();
        let myv = vec![
            String::from("awesome"),
            String::from("string"),
            String::from("stuff"),
        ];
        my_map.insert(String::from("my"), myv);
        let my_node = Node {
            _id: String::from("mnode"),
            _data: my_map,
        };
        let n2 = Node::from_nodish_ref(&my_node);
        assert_eq!(my_node, n2);
    }
    #[test]
    fn test_from_nodish() {
        let mut my_map: HashMap<String, Vec<String>> = HashMap::new();
        let myv = vec![
            String::from("awesome"),
            String::from("string"),
            String::from("stuff"),
        ];
        my_map.insert(String::from("my"), myv);
        let my_node = Node {
            _id: String::from("mnode"),
            _data: my_map,
        };
        let n1 = my_node.clone();
        let n2 = Node::from_nodish(my_node);
        assert_eq!(n1, n2);
    }
}
