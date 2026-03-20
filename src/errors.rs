use thiserror::Error;

/// Error type for all graph-theoretical operations in this library
#[derive(Error, Debug)]
pub enum PGMRustError {
    /// Attempted to construct a path from an empty edge set
    #[error("Empty edge set")]
    EmptyEdgeSet,

    /// The provided edges do not form a simple path
    #[error("The provided edges do not form a simple path (endpoints found: {0})")]
    NotASimplePath(usize),

    /// The edge set is disconnected and cannot be ordered into a single path
    #[error("Path is disconnected: {0} edge(s) could not be reached from the current node")]
    DisconnectedPath(usize),

    /// A zero-length path is not supported
    #[error("Diestel defines a path as non-empty; length 0 path not supported here")]
    EmptyPath,

    /// The element (first field) is not contained in the graph (second field)
    #[error("{0} not in {1}")]
    NotInGraph(String, String),

    /// A required key was missing from an internal map
    #[error("Key not found: {0}")]
    KeyNotFound(String),
}

/// Convenience alias for `Result<T, PGMRustError>`
pub type PGMRustResult<T> = Result<T, PGMRustError>;
