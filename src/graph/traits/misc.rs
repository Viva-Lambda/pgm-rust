/// diverse traits that help with various tasks
use crate::graph::traits::graph_obj::GraphObject;

/// set operation for graph objects
pub trait SetOp: GraphObject {
    /// the output of the set operation
    type Output;

    /// input type
    type Input;

    /// set intersection operation
    fn intersection(a: Self::Input, other: Self::Input) -> Self::Output;
    /// set union operation
    fn union(a: Self::Input, other: Self::Input) -> Self::Output;
    /// set difference
    fn difference(a: Self::Input, other: Self::Input) -> Self::Output;
    /// symmetric set difference
    fn symmetric_difference(a: Self::Input, other: Self::Input) -> Self::Output;
}

