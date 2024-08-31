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
pub struct DfsForestMaps<'a, C: CycleInfoTrait, N: NodeTrait> {
    pub vertices: &'a mut HashMap<String, &'a N>,
    pub first_visit: &'a mut HashMap<String, usize>,
    pub last_visit: &'a mut HashMap<String, usize>,
    pub pred: &'a mut HashMap<String, Option<String>>,
    pub marked: &'a mut HashMap<String, bool>,
    pub identifiers: &'a mut HashSet<String>,
    pub cycles: &'a mut HashMap<String, Vec<C>>,
}

/// holds information about cycles in the graph
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CycleInfo {
    _ancestor: String,
    _before: String,
    _ancestor_first_time_visit: usize,
    _ancestor_last_time_visit: Option<usize>,
    _current_final_time_visit: usize,
}

impl CycleInfoTrait for CycleInfo {
    fn ancestor(&self) -> String {
        self._ancestor.clone()
    }
    fn before(&self) -> String {
        self._before.clone()
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
        ancestor: String,
        before: String,
        ancestor_first_time: usize,
        ancestor_last_time: Option<usize>,
        current_final_time: usize,
    ) -> CycleInfo {
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
pub struct DepthFirstResult<'a, N: NodeTrait, E: EdgeTrait<N>, C: CycleInfoTrait> {
    _forest: HashMap<String, HashSet<&'a E>>,
    _trees: HashMap<String, HashMap<String, String>>,
    _nb_component: usize,
    _components: HashMap<String, HashSet<String>>,
    _first_visit_times: HashMap<String, usize>,
    _last_visit_times: HashMap<String, usize>,
    _cycle_info: C,
    _node_type: PhantomData<N>,
    _id: String,
    _data: HashMap<String, Vec<String>>,
}

impl<N: NodeTrait, E: EdgeTrait<N>, C: CycleInfoTrait> fmt::Display
    for DepthFirstResult<'_, N, E, C>
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

impl<N: NodeTrait, E: EdgeTrait<N>, C: CycleInfoTrait> Hash for DepthFirstResult<'_, N, E, C> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self._id.hash(state);
    }
}

impl<N: NodeTrait, E: EdgeTrait<N>, C: CycleInfoTrait> GraphObjectTrait
    for DepthFirstResult<'_, N, E, C>
{
    fn id(&self) -> &String {
        &self._id
    }
    fn data(&self) -> &HashMap<String, Vec<String>> {
        &self._data
    }
}

impl<'a, N: NodeTrait, E: EdgeTrait<N>, C: CycleInfoTrait> DepthFirstResultTrait<'a, N, E, C>
    for DepthFirstResult<'a, N, E, C>
{
    fn forest(&self) -> &HashMap<String, HashSet<&'a E>> {
        &self._forest
    }
    fn trees(&self) -> &HashMap<String, HashMap<String, String>> {
        &self._trees
    }
    fn nb_component(&self) -> usize {
        self._nb_component
    }
    fn components(&self) -> &HashMap<String, HashSet<String>> {
        &self._components
    }
    fn first_visit_times(&self) -> &HashMap<String, usize> {
        &self._first_visit_times
    }
    fn last_visit_times(&self) -> &HashMap<String, usize> {
        &self._last_visit_times
    }
    fn cycle_info(&self) -> &C {
        &self._cycle_info
    }

    fn create(
        &self,
        forest: HashMap<String, HashSet<&'a E>>,
        tree: HashMap<String, HashMap<String, String>>,
        components: HashMap<String, HashSet<String>>,
        visit_times: (HashMap<String, usize>, HashMap<String, usize>),
        cycle: C,
    ) -> Self {
        DepthFirstResult::<'a, N, E, C> {
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
