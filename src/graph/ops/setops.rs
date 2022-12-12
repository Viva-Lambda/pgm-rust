//! Set operation functions defined on graphs

use crate::graph::traits::graph_obj::GraphObject;
use std::collections::HashSet;

/// Find intersection of graph object sets
pub fn intersection<'a, M: GraphObject>(a1: HashSet<&'a M>, a2: HashSet<&'a M>) -> HashSet<&'a M> {
    //
    let mut inter = HashSet::new();
    for i in a1.intersection(&a2) {
        inter.insert(i.clone());
    }
    inter
}

#[cfg(test)]
mod tests {}
