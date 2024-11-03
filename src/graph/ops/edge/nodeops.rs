//! Functions that has an [Edge] among arguments that output a [Node]
use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::node::Node as NodeTrait;
use std::option::Option;

/// get the opposite node from edge
/// # Description
pub fn get_other<'a, 'b, N, E>(e: &'a E, n: &'b N) -> Option<&'a N>
where
    N: NodeTrait,
    E: EdgeTrait<N>,
{
    let nid: &String = n.id();
    let start = e.start();
    let sid = start.id();
    let end = e.end();
    let eid = end.id();
    if sid == nid {
        Some(e.end())
    } else if eid == nid {
        Some(e.start())
    } else {
        None
        //panic!("{n} does not belong to this {e}");
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::graph::types::edge::Edge;
    use crate::graph::types::node::Node;
    use std::collections::HashMap; // brings in the parent scope to current module scope

    fn mk_uedge() -> Edge<Node> {
        let n1 = Node::new(String::from("m1"), HashMap::new());
        let n2 = Node::new(String::from("m2"), HashMap::new());
        let mut h1 = HashMap::new();
        h1.insert(String::from("my"), vec![String::from("data")]);
        Edge::undirected(String::from("uedge"), n1, n2, h1)
    }
    #[test]
    fn test_get_other_some() {
        let e = mk_uedge();
        let n2 = Node::new(String::from("m2"), HashMap::new());
        let n1 = Some(Node::new(String::from("m1"), HashMap::new()));
        assert_eq!(get_other(&e, &n2), n1.as_ref());
    }

    fn test_get_other_none() {
        let e = mk_uedge();
        let n2 = Node::new(String::from("m3"), HashMap::new());
        assert_eq!(get_other(&e, &n2), None);
    }
}
