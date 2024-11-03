///! Interface es for defining search behavior and result
use crate::graph::traits::edge::Edge;
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::traits::node::Node;
use std::collections::HashMap;
use std::collections::HashSet;

pub trait CycleInfo<'graph_lifetime>: Eq + Clone {
    fn ancestor(&self) -> &'graph_lifetime String;
    fn before(&self) -> &'graph_lifetime String;
    fn ancestor_first_time_visit(&self) -> usize;
    fn ancestor_last_time_visit(&self) -> Option<usize>;
    fn current_final_time_visit(&self) -> usize;
    fn create(
        &self,
        ancestor: &'graph_lifetime String,
        before: &'graph_lifetime String,
        ancestor_first_time: usize,
        ancestor_last_time: Option<usize>,
        current_final_time: usize,
    ) -> Self;
}

/// promotes object to search result
pub trait DepthFirstResult<
    'graph_lifetime,
    NodeType: Node,
    EdgeType: Edge<NodeType>,
    C: CycleInfo<'graph_lifetime>,
>: GraphObject
{
    fn forest(&self) -> &HashMap<&'graph_lifetime String, HashSet<&'graph_lifetime EdgeType>>;
    fn trees(
        &self,
    ) -> &HashMap<&'graph_lifetime String, HashMap<&'graph_lifetime String, &'graph_lifetime String>>;
    fn nb_component(&self) -> usize;
    fn components(&self) -> &HashMap<&'graph_lifetime String, HashSet<&'graph_lifetime String>>;
    fn first_visit_times(&self) -> &HashMap<&'graph_lifetime String, usize>;
    fn last_visit_times(&self) -> &HashMap<&'graph_lifetime String, usize>;
    fn cycle_info(&self) -> &C;
    fn create(
        &self,
        forest: HashMap<&'graph_lifetime String, HashSet<&'graph_lifetime EdgeType>>,
        tree: HashMap<
            &'graph_lifetime String,
            HashMap<&'graph_lifetime String, &'graph_lifetime String>,
        >,
        components: HashMap<&'graph_lifetime String, HashSet<&'graph_lifetime String>>,
        visit_times: (
            HashMap<&'graph_lifetime String, usize>,
            HashMap<&'graph_lifetime String, usize>,
        ),
        cycle: C,
    ) -> Self;
}
