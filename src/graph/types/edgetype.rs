// edge type enum
use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum EdgeType {
    Directed,
    Undirected,
}

impl fmt::Display for EdgeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
