//! graph searching related types
use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::graph_obj::GraphObject as GraphObjectTrait;
use crate::graph::traits::node::Node as NodeTrait;
use crate::graph::traits::search::CycleInfo as CycleInfoTrait;
use crate::graph::traits::search::DepthFirstResult as DepthFirstResultTrait;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::option::Option;
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq)]
pub struct DfsForestMaps<
    'graph_lifetime,
    'dfs_lifetime,
    C: CycleInfoTrait<'graph_lifetime>,
    N: NodeTrait,
> {
    pub vertices: &'dfs_lifetime mut HashMap<String, &'graph_lifetime N>,
    pub first_visit: &'dfs_lifetime mut HashMap<&'graph_lifetime String, usize>,
    pub last_visit: &'dfs_lifetime mut HashMap<&'graph_lifetime String, usize>,
    pub pred: &'dfs_lifetime mut HashMap<&'graph_lifetime String, Option<&'graph_lifetime String>>,
    pub marked: &'dfs_lifetime mut HashMap<&'graph_lifetime String, bool>,
    pub identifiers: &'dfs_lifetime mut HashSet<&'graph_lifetime String>,
    pub cycles: &'dfs_lifetime mut HashMap<&'graph_lifetime String, Vec<C>>,
}

/// holds information about cycles in the graph
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CycleInfo<'graph_lifetime> {
    _ancestor: &'graph_lifetime String,
    _before: &'graph_lifetime String,
    _ancestor_first_time_visit: usize,
    _ancestor_last_time_visit: Option<usize>,
    _current_final_time_visit: usize,
}

impl<'graph_lifetime> CycleInfoTrait<'graph_lifetime> for CycleInfo<'graph_lifetime> {
    fn ancestor(&self) -> &'graph_lifetime String {
        let ancestor: &'graph_lifetime String = self._ancestor;
        ancestor
    }
    fn before(&self) -> &'graph_lifetime String {
        self._before
    }
    fn ancestor_first_time_visit(&self) -> usize {
        self._ancestor_first_time_visit
    }
    fn ancestor_last_time_visit(&self) -> Option<usize> {
        self._ancestor_last_time_visit.clone()
    }
    fn current_final_time_visit(&self) -> usize {
        self._current_final_time_visit
    }
    fn create(
        &self,
        ancestor: &'graph_lifetime String,
        before: &'graph_lifetime String,
        ancestor_first_time: usize,
        ancestor_last_time: Option<usize>,
        current_final_time: usize,
    ) -> CycleInfo<'graph_lifetime> {
        CycleInfo {
            _ancestor: ancestor,
            _before: before,
            _ancestor_first_time_visit: ancestor_first_time,
            _ancestor_last_time_visit: ancestor_last_time,
            _current_final_time_visit: current_final_time,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DepthFirstResult<
    'graph_lifetime,
    N: NodeTrait,
    E: EdgeTrait<N>,
    C: CycleInfoTrait<'graph_lifetime>,
> {
    _forest: HashMap<&'graph_lifetime String, HashSet<&'graph_lifetime E>>,
    _trees:
        HashMap<&'graph_lifetime String, HashMap<&'graph_lifetime String, &'graph_lifetime String>>,
    _nb_component: usize,
    _components: HashMap<&'graph_lifetime String, HashSet<&'graph_lifetime String>>,
    _first_visit_times: HashMap<&'graph_lifetime String, usize>,
    _last_visit_times: HashMap<&'graph_lifetime String, usize>,
    _cycle_info: C,
    _node_type: PhantomData<N>,
    _id: String,
    _data: HashMap<String, Vec<String>>,
}

impl<'graph_lifetime, N: NodeTrait, E: EdgeTrait<N>, C: CycleInfoTrait<'graph_lifetime>>
    fmt::Display for DepthFirstResult<'graph_lifetime, N, E, C>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nid: &String = &self._id;
        let nb_component: &String = &self._nb_component.to_string();
        write!(
            f,
            "<DepthFirstResult id='{}' nb_component='{}'/>",
            nid, nb_component
        )
    }
}

impl<'graph_lifetime, N: NodeTrait, E: EdgeTrait<N>, C: CycleInfoTrait<'graph_lifetime>> Hash
    for DepthFirstResult<'graph_lifetime, N, E, C>
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self._id.hash(state);
    }
}

impl<'graph_lifetime, N: NodeTrait, E: EdgeTrait<N>, C: CycleInfoTrait<'graph_lifetime>>
    GraphObjectTrait for DepthFirstResult<'graph_lifetime, N, E, C>
{
    fn id(&self) -> &String {
        &self._id
    }
    fn data(&self) -> &HashMap<String, Vec<String>> {
        &self._data
    }
}

impl<'graph_lifetime, N: NodeTrait, E: EdgeTrait<N>, C: CycleInfoTrait<'graph_lifetime>>
    DepthFirstResultTrait<'graph_lifetime, N, E, C> for DepthFirstResult<'graph_lifetime, N, E, C>
{
    fn forest(&self) -> &HashMap<&'graph_lifetime String, HashSet<&'graph_lifetime E>> {
        &self._forest
    }
    fn trees(
        &self,
    ) -> &HashMap<&'graph_lifetime String, HashMap<&'graph_lifetime String, &'graph_lifetime String>>
    {
        &self._trees
    }
    fn nb_component(&self) -> usize {
        self._nb_component
    }
    fn components(&self) -> &HashMap<&'graph_lifetime String, HashSet<&'graph_lifetime String>> {
        &self._components
    }
    fn first_visit_times(&self) -> &HashMap<&'graph_lifetime String, usize> {
        &self._first_visit_times
    }
    fn last_visit_times(&self) -> &HashMap<&'graph_lifetime String, usize> {
        &self._last_visit_times
    }
    fn cycle_info(&self) -> &C {
        &self._cycle_info
    }

    fn create(
        &self,
        forest: HashMap<&'graph_lifetime String, HashSet<&'graph_lifetime E>>,
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
    ) -> Self {
        DepthFirstResult::<'graph_lifetime, N, E, C> {
            _forest: forest,
            _trees: tree,
            _nb_component: components.len(),
            _components: components,
            _first_visit_times: visit_times.0,
            _last_visit_times: visit_times.1,
            _cycle_info: cycle,
            _node_type: PhantomData,
            _id: Uuid::new_v4().to_string(),
            _data: HashMap::new(),
        }
    }
}
