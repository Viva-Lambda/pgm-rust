// edge type enum
use std::fmt;

/// Indicates whether an edge is directed or undirected.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum EdgeType {
    /// directed edge: it has implications on neighborhood functions
    Directed,
    /// undirected edge: it has implications on neighborhood functions
    Undirected,
}

impl fmt::Display for EdgeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
