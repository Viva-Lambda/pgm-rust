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
