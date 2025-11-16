// graph object
//
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

/// Promotes anything that is hashable and converted to string to a [GraphObject]
/// This is almost exchangeable with being a [Node]
pub trait GraphObject: fmt::Display + Hash + Eq {
    /// identifier for graph object
    fn id(&self) -> &str;
    /// data that is associated to graph object
    fn data(&self) -> HashMap<&str, Vec<&str>>;

    /// set id, notice ref is immutable
    fn set_id(&self, idstr: &str) -> Self;

    /// set data, notice ref is immutable
    fn set_data(&self, data: HashMap<&str, Vec<&str>>) -> Self;

    /// null constructor
    fn null() -> Self;
}
