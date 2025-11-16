//

// behaviors that defines a [Graph]
// pub mod graph;

/// behaviors that defines a [GraphObject]
/// everything that implements a [GraphObject] can be used as a node.
pub mod graph_obj;

/// behaviors that defines a [Node]
/// Essentially it is a very thin wrapper over [GraphObject]. They only look
/// different when represented as strings.
pub mod node;

/// behaviors that defines an [Edge]
pub mod edge;

/// diverse behaviors that help with lib implementation
pub mod misc;

// behaviors that defines a [Tree]
// pub mod tree;

// behaviors that defines a [Path]
// pub mod path;

// behaviors that define a search result
// pub mod search;
