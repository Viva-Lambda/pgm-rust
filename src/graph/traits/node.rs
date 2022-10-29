// node trait

use crate::graph::traits::graph_obj::GraphObject;

/// Promotes anything that implements a [GraphObject] to being a [Node]
pub trait Node: GraphObject {
}
