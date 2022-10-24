//! functions that has a graph among its arguments that output a boolean value
use crate::graph::traits::graph::Graph as GraphTrait;
use crate::graph::types::graph::Graph;

pub fn is_empty(g: &Graph) -> bool {
    g.vertices().is_empty()
}
