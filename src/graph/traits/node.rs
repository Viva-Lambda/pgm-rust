// node trait

use crate::graph::traits::graph_obj::GraphObject;
use std::collections::HashMap;

/// Promotes anything that implements a [GraphObject] to being a [Node]
pub trait Node: GraphObject + Clone {
    /// factory function for node
    fn create(_: String, _: HashMap<String, Vec<String>>) -> Self;
}
