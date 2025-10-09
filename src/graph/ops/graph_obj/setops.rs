//! set operations on graph object

use crate::graph::traits::graph_obj::GraphObject;
use std::collections::HashSet;

/// indicates set operation kind
pub enum SetOpKind {
    /// union operation
    Union,
    /// intersection operation
    Intersection,
    /// difference operation
    Difference,
    /// symmetric difference operation
    SymmetricDifference,
}
/// set operation on set of references
pub fn set_op_graph_obj_ref_set<'a, T: GraphObject>(
    a: &'a HashSet<&T>,
    b: &'a HashSet<&T>,
    set_op_kind: SetOpKind,
) -> HashSet<&'a T> {
    let mut hset = HashSet::new();
    match set_op_kind {
        SetOpKind::Intersection => {
            for c in a.intersection(&b) {
                hset.insert(c.clone());
            }
        }
        SetOpKind::Union => {
            for c in a.union(&b) {
                hset.insert(c.clone());
            }
        }
        SetOpKind::Difference => {
            for c in a.difference(&b) {
                hset.insert(c.clone());
            }
        }
        SetOpKind::SymmetricDifference => {
            for c in a.symmetric_difference(&b) {
                hset.insert(c.clone());
            }
        }
    }
    hset
}

/// set operation on object sets
pub fn set_op_graph_obj_set<T: GraphObject + Clone>(
    a: &HashSet<T>,
    b: &HashSet<T>,
    set_op_kind: SetOpKind,
) -> HashSet<T> {
    let mut hset: HashSet<T> = HashSet::new();
    match set_op_kind {
        SetOpKind::Intersection => {
            for c in a.intersection(&b) {
                let cref: T = c.clone();
                hset.insert(cref);
            }
        }
        SetOpKind::Union => {
            for c in a.union(&b) {
                let cref: T = c.clone();
                hset.insert(cref);
            }
        }
        SetOpKind::Difference => {
            for c in a.difference(&b) {
                let cref: T = c.clone();
                hset.insert(cref);
            }
        }
        SetOpKind::SymmetricDifference => {
            for c in a.symmetric_difference(&b) {
                let cref: T = c.clone();
                hset.insert(cref);
            }
        }
    }
    hset
}

#[cfg(test)]
mod tests {
    use super::*; // brings in the parent scope to current module scope
    use crate::graph::types::node::Node;

    // Tests for the first function
    #[test]
    fn test_set_op_graph_obj_ref_set_union() {
        let n1 = Node::empty("n1");
        let n2 = Node::empty("n2");
        let n3 = Node::empty("n3");
        
        let set_a: HashSet<&Node> = HashSet::from([&n1, &n2]);
        let set_b: HashSet<&Node> = HashSet::from([&n2, &n3]);
        
        let result = set_op_graph_obj_ref_set(&set_a, &set_b, SetOpKind::Union);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&&n1));
        assert!(result.contains(&&n2));
        assert!(result.contains(&&n3));
    }

    #[test]
    fn test_set_op_graph_obj_ref_set_intersection() {
        let n1 = Node::empty("n1");
        let n2 = Node::empty("n2");
        let n3 = Node::empty("n3");
        
        let set_a: HashSet<&Node> = HashSet::from([&n1, &n2]);
        let set_b: HashSet<&Node> = HashSet::from([&n2, &n3]);
        
        let result = set_op_graph_obj_ref_set(&set_a, &set_b, SetOpKind::Intersection);
        assert_eq!(result.len(), 1);
        assert!(result.contains(&&n2));
    }

    #[test]
    fn test_set_op_graph_obj_ref_set_difference() {
        let n1 = Node::empty("n1");
        let n2 = Node::empty("n2");
        let n3 = Node::empty("n3");
        
        let set_a: HashSet<&Node> = HashSet::from([&n1, &n2]);
        let set_b: HashSet<&Node> = HashSet::from([&n2, &n3]);
        
        let result = set_op_graph_obj_ref_set(&set_a, &set_b, SetOpKind::Difference);
        assert_eq!(result.len(), 1);
        assert!(result.contains(&&n1));
    }

    #[test]
    fn test_set_op_graph_obj_ref_set_symmetric_difference() {
        let n1 = Node::empty("n1");
        let n2 = Node::empty("n2");
        let n3 = Node::empty("n3");
        
        let set_a: HashSet<&Node> = HashSet::from([&n1, &n2]);
        let set_b: HashSet<&Node> = HashSet::from([&n2, &n3]);
        
        let result = set_op_graph_obj_ref_set(&set_a, &set_b, SetOpKind::SymmetricDifference);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&&n1));
        assert!(result.contains(&&n3));
        assert!(!result.contains(&&n2));
    }

    // Tests for the second function
    #[test]
    fn test_set_op_graph_obj_set_union() {
        let n1 = Node::empty("n1");
        let n2 = Node::empty("n2");
        let n3 = Node::empty("n3");
        
        let set_a: HashSet<Node> = HashSet::from([n1.clone(), n2.clone()]);
        let set_b: HashSet<Node> = HashSet::from([n2.clone(), n3.clone()]);
        
        let result = set_op_graph_obj_set(&set_a, &set_b, SetOpKind::Union);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&n1));
        assert!(result.contains(&n2));
        assert!(result.contains(&n3));
    }

    #[test]
    fn test_set_op_graph_obj_set_intersection() {
        let n1 = Node::empty("n1");
        let n2 = Node::empty("n2");
        let n3 = Node::empty("n3");
        
        let set_a: HashSet<Node> = HashSet::from([n1.clone(), n2.clone()]);
        let set_b: HashSet<Node> = HashSet::from([n2.clone(), n3.clone()]);
        
        let result = set_op_graph_obj_set(&set_a, &set_b, SetOpKind::Intersection);
        assert_eq!(result.len(), 1);
        assert!(result.contains(&n2));
        assert!(!result.contains(&n1));
        assert!(!result.contains(&n3));
    }

    #[test]
    fn test_set_op_graph_obj_set_difference() {
        let n1 = Node::empty("n1");
        let n2 = Node::empty("n2");
        let n3 = Node::empty("n3");
        
        let set_a: HashSet<Node> = HashSet::from([n1.clone(), n2.clone()]);
        let set_b: HashSet<Node> = HashSet::from([n2.clone(), n3.clone()]);
        
        let result = set_op_graph_obj_set(&set_a, &set_b, SetOpKind::Difference);
        assert_eq!(result.len(), 1);
        assert!(result.contains(&n1));
        assert!(!result.contains(&n2));
        assert!(!result.contains(&n3));
    }

    #[test]
    fn test_set_op_graph_obj_set_symmetric_difference() {
        let n1 = Node::empty("n1");
        let n2 = Node::empty("n2");
        let n3 = Node::empty("n3");
        
        let set_a: HashSet<Node> = HashSet::from([n1.clone(), n2.clone()]);
        let set_b: HashSet<Node> = HashSet::from([n2.clone(), n3.clone()]);
        
        let result = set_op_graph_obj_set(&set_a, &set_b, SetOpKind::SymmetricDifference);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&n1));
        assert!(result.contains(&n3));
        assert!(!result.contains(&n2));
    }
}