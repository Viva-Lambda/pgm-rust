//! Functions that has an [Edge] among arguments that output a [Node]
use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::types::edge::Edge;
use crate::graph::types::node::Node;

pub fn get_other<'a>(e: &'a Edge, n: &Node) -> &'a Node {
    let nid: &String = n.id();
    let start = e.start();
    let sid = start.id();
    let end = e.end();
    let eid = end.id();
    if sid == nid {
        e.end()
    } else if eid == nid {
        e.start()
    } else {
        let this_edge = dbg!(e);
        let this_node = dbg!(n);
        panic!("{n} does not belong to this {this_edge}");
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap; // brings in the parent scope to current module scope

    fn mk_uedge() -> Edge {
        let n1 = Node::new(String::from("m1"), HashMap::new());
        let n2 = Node::new(String::from("m2"), HashMap::new());
        let mut h1 = HashMap::new();
        h1.insert(String::from("my"), vec![String::from("data")]);
        Edge::undirected(String::from("uedge"), n1, n2, h1)
    }
    #[test]
    fn test_get_other() {
        let e = mk_uedge();
        let n2 = Node::new(String::from("m2"), HashMap::new());
        let n1 = Node::new(String::from("m1"), HashMap::new());
        assert_eq!(get_other(&e, &n2), &n1);
    }
}
