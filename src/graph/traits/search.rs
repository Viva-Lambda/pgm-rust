///! Interface es for defining search behavior and result
use crate::graph::traits::edge::Edge;
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::traits::node::Node;
use std::collections::HashMap;
use std::collections::HashSet;

pub trait CycleInfo: Eq + Clone {
    fn ancestor(&self) -> String;
    fn before(&self) -> String;
    fn ancestor_first_time_visit(&self) -> usize;
    fn ancestor_last_time_visit(&self) -> Option<usize>;
    fn current_final_time_visit(&self) -> usize;
    fn create(
        ancestor: String,
        before: String,
        ancestor_first_time: usize,
        ancestor_last_time: Option<usize>,
        current_final_time: usize,
    ) -> Self;
}

/// promotes object to search result
pub trait DepthFirstResult<'a, NodeType: Node, EdgeType: Edge<NodeType>, C: CycleInfo>:
    GraphObject
{
    fn forest(&self) -> &HashMap<String, HashSet<&'a EdgeType>>;
    fn trees(&self) -> &HashMap<String, HashMap<String, String>>;
    fn nb_component(&self) -> usize;
    fn components(&self) -> &HashMap<String, HashSet<String>>;
    fn first_visit_times(&self) -> &HashMap<String, usize>;
    fn last_visit_times(&self) -> &HashMap<String, usize>;
    fn cycle_info(&self) -> &C;
    fn create(
        &self,
        forest: HashMap<String, HashSet<&'a EdgeType>>,
        tree: HashMap<String, HashMap<String, String>>,
        components: HashMap<String, HashSet<String>>,
        visit_times: (HashMap<String, usize>, HashMap<String, usize>),
        cycle: C,
    ) -> Self;
}
