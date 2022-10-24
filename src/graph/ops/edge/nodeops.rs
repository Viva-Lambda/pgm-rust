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
    } else if sid == eid {
        e.start()
    } else {
        let this_edge = dbg!(e);
        let this_node = dbg!(n);
        panic!("{n} does not belong to this {this_edge}");
    }
}
