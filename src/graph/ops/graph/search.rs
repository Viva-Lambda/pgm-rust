//! graph searching
use crate::graph::ops::edge::nodeops::get_other;
use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::graph::Graph as GraphTrait;
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::traits::node::Node as NodeTrait;
use crate::graph::traits::search::CycleInfo as CycleInfoTrait;
use crate::graph::traits::search::DepthFirstResult as DepthFirstResultTrait;
use crate::graph::types::search::DfsForestMaps;
use std::collections::HashMap;
use std::collections::HashSet;
use std::option::Option;

fn get_vertex_lst<'nodeLT, N: NodeTrait>(
    vs: HashMap<String, &'nodeLT N>,
    start_node: Option<&'nodeLT N>,
) -> (Vec<String>, HashMap<String, &'nodeLT N>) {
    let mut ns = vs;
    match start_node {
        Some(nref) => {
            ns.insert(nref.id().to_string(), nref);
        }
        None => {}
    }
    let mut vecs: Vec<String> = ns.keys().map(|x| x.to_string()).collect();
    vecs.sort();
    (vecs, ns)
}

fn _init_dfs_result<'a, N, E, C, DFSResult>(vertices: HashMap<String, &'a N>) -> DFSResult
where
    N: NodeTrait,
    E: EdgeTrait<N>,
    C: CycleInfoTrait,
    DFSResult: DepthFirstResultTrait<'a, N, E, C>,
{
    let mut marked: HashMap<String, bool> = vertices.keys().map(|x| (x.clone(), false)).collect();
    let mut preds: HashMap<String, HashMap<String, Option<String>>> = HashMap::new();
    let mut ids: HashMap<String, HashSet<String>> = HashMap::new();
    let mut cycles: HashMap<String, Vec<C>> = HashMap::new();
    let mut d: HashMap<String, usize> = vertices.keys().map(|x| (x.clone(), usize::MAX)).collect();
    let mut f: HashMap<String, usize> = vertices.keys().map(|x| (x.clone(), usize::MAX)).collect();

    let mut identifiers: HashSet<String> = HashSet::new();
    let mut pred: HashMap<String, Option<String>> =
        vertices.keys().map(|x| (x.clone(), None)).collect();
    let mut dfs_record = DfsForestMaps {
        vertices: &mut vertices,
        first_visit: &mut d,
        last_visit: &mut f,
        pred: &mut pred,
        marked: &mut marked,
        identifiers: &mut identifiers,
        cycles: &mut cycles,
    };
    dfs_record
}

fn depth_first_search_v2<'graphLT, N, E, G, F, C, DFSResult>(
    g: &'graphLT G,
    edge_generator: &F,
    check_cycle: bool,
    start_node: Option<&'graphLT N>,
) -> DFSResult
where
    N: NodeTrait,
    E: EdgeTrait<N>,
    G: GraphTrait<N, E>,
    F: Fn(&'graphLT G, &'graphLT N) -> HashSet<&'graphLT E>,
    C: CycleInfoTrait,
    DFSResult: DepthFirstResultTrait<'graphLT, N, E, C>,
{
}

fn depth_first_search<'a, N, E, G, F, C, DFSResult>(
    g: &G,
    edge_generator: &F,
    check_cycle: bool,
    start_node: Option<&N>,
) -> DFSResult
where
    N: NodeTrait,
    E: EdgeTrait<N>,
    G: GraphTrait<N, E>,
    F: Fn(&N) -> HashSet<&E>,
    C: CycleInfoTrait,
    DFSResult: DepthFirstResultTrait<'a, N, E, C>,
{
    let vm = g.vmap();
    let (vlist, vertices) = get_vertex_lst::<N>(vm, start_node);
    let mut time: usize = 0;
    let mut marked: HashMap<String, bool> = vertices.keys().map(|x| (x.clone(), false)).collect();
    let mut preds: HashMap<String, HashMap<String, Option<String>>> = HashMap::new();
    let mut ids: HashMap<String, HashSet<String>> = HashMap::new();
    let mut cycles: HashMap<String, Vec<C>> = HashMap::new();
    let mut d: HashMap<String, usize> = vertices.keys().map(|x| (x.clone(), usize::MAX)).collect();
    let mut f: HashMap<String, usize> = vertices.keys().map(|x| (x.clone(), usize::MAX)).collect();
    let mut component_counter = 0;
    for u in vlist {
        match marked.get(&u) {
            None => {
                panic!("key not exist in marked");
            }
            Some(val_ref) => {
                if !val_ref {
                    let mut identifiers: HashSet<String> = HashSet::new();
                    let mut pred: HashMap<String, Option<String>> =
                        vertices.keys().map(|x| (x.clone(), None)).collect();
                    let mut dfs_record = DfsForestMaps {
                        vertices: &mut vertices,
                        first_visit: &mut d,
                        last_visit: &mut f,
                        pred: &mut pred,
                        marked: &mut marked,
                        identifiers: &mut identifiers,
                        cycles: &mut cycles,
                    };
                    dfs_forest2(
                        g,
                        &mut dfs_record,
                        u.to_string(),
                        &mut time,
                        edge_generator,
                        check_cycle,
                    );
                    component_counter += 1;
                }
            }
        }
    }
}

/// dfs search adapted for cycle detection
/// # Description
/// the algorithm comes from dfs recursive forest from Erciyes 2018, Guide Graph ..., p.152 alg. 6.7
/// # Args
/// - f storing last visit times per node
/// - d storing first visit times per node
/// - cycles storing cycle info
/// - marked storing if node is visited
/// - pred storing the parent of nodes
/// - g graph we are searching for
/// - vertices vertex set of g converted to dict for easy access
/// - u node id
/// - T set of pred nodes
/// - time global visit counter
/// - check_cycle fill cycles if it is detected
/// - edge_generator generate edges of a vertex with respect to graph type
fn dfs_forest2<'a, N, E, G, F, C>(
    g: &'a G,
    dfs_record: &mut DfsForestMaps<C, N>,
    u: &'a String,
    time: &mut usize,
    edge_generator: &F,
    check_cycle: bool,
) -> ()
where
    N: NodeTrait,
    E: EdgeTrait<N>,
    G: GraphTrait<N, E>,
    F: Fn(&N) -> HashSet<&E>,
    C: CycleInfoTrait,
{
    dfs_record.marked.insert(u.to_string(), true);
    *time += 1;
    dfs_record.first_visit.insert(u.to_string(), *time);
    let uopt = dfs_record.vertices.get(u);
    match uopt {
        None => (),
        Some(unode) => {
            for edge in edge_generator(unode) {
                let vopt: Option<String> = fetch_v_from_edge(edge, unode);
                match vopt {
                    None => (),
                    Some(v) => {
                        if is_not_marked(v.clone(), dfs_record) {
                            insert_to_pred(v.clone(), &u, dfs_record);
                            dfs_forest2(g, dfs_record, &v, time, edge_generator, check_cycle);
                        }
                    }
                }
            }
        }
    }
    *time += 1;
    dfs_record.last_visit.insert(u.clone(), time.clone());
    if check_cycle {
        //
        // v ancestor, u visiting node
        // edge between them is a back edge
        // see p. 151, and p. 159-160
        match dfs_record.vertices.get(u) {
            None => (),
            Some(unode) => {
                for edge in edge_generator(unode) {
                    let vid_opt: Option<String> = fetch_v_from_edge(edge, unode);
                    let u_id: String = unode.id().to_string();
                    if is_vid_in_prediction(&u_id, &vid_opt, dfs_record) {
                        if fetch_cond(dfs_record, &vid_opt, u_id) {
                            let cycle_info_opt = mk_cycle_info(&u, vid_opt, dfs_record);
                            add_to_cycle(u, cycle_info_opt, dfs_record);
                        }
                    }
                }
            }
        }
    }
}

fn mk_cycle_info<C: CycleInfoTrait, N: NodeTrait>(
    u: &String,
    vid_opt: Option<String>,
    dfs_record: &DfsForestMaps<C, N>,
) -> Option<C> {
    let d: &HashMap<String, usize> = dfs_record.first_visit;
    match vid_opt {
        None => None,
        Some(vid) => match dfs_record.last_visit.get(&vid) {
            None => None,
            Some(first_visit) => match d.get(&vid) {
                None => None,
                Some(last_visit) => match dfs_record.last_visit.get(u) {
                    None => None,
                    Some(current_u) => {
                        let cycle_info: C = C::create(
                            vid,
                            u.to_string(),
                            first_visit.clone(),
                            Some(last_visit.clone()),
                            current_u.clone(),
                        );
                        Some(cycle_info)
                    }
                },
            },
        },
    }
}

fn add_to_cycle<'a, C: CycleInfoTrait, N: NodeTrait>(
    u: &'a String,
    cycle_info_opt: Option<C>,
    dfs_record: &mut DfsForestMaps<C, N>,
) -> () {
    let uvec_op = dfs_record.cycles.get(u);
    match cycle_info_opt {
        None => (),
        Some(cycle_info) => match uvec_op {
            None => (),
            Some(uvec) => {
                let mut uv = uvec.clone();
                uv.push(cycle_info);
                dfs_record.cycles.insert(u, uv);
            }
        },
    }
}

fn fetch_cond<C: CycleInfoTrait, N: NodeTrait>(
    dfs_record: &DfsForestMaps<C, N>,
    vid_opt: &Option<String>,
    u: String,
) -> bool {
    let d: &HashMap<String, usize> = dfs_record.first_visit;
    let f: &HashMap<String, usize> = dfs_record.last_visit;
    match vid_opt {
        None => false,
        Some(vid) => match d.get(&vid) {
            None => false,
            Some(d_vid) => match f.get(&u) {
                None => false,
                Some(f_u) => d_vid < f_u,
            },
        },
    }
}

fn insert_to_pred<N: NodeTrait, C: CycleInfoTrait>(
    v: String,
    u: &String,
    dfs_record: &mut DfsForestMaps<C, N>,
) -> () {
    dfs_record.pred.insert(v.clone(), Some(u.to_string()));
    dfs_record.identifiers.insert(v.clone());
}

fn fetch_v_from_edge<N, E>(e: &E, unode: &N) -> Option<String>
where
    N: NodeTrait,
    E: EdgeTrait<N>,
{
    match get_other(e, unode) {
        None => None,
        Some(vnode) => Some(vnode.id().clone()),
    }
}

fn is_not_marked<N, C>(v: String, dfs_record: &DfsForestMaps<C, N>) -> bool
where
    N: NodeTrait,
    C: CycleInfoTrait,
{
    let marked: &HashMap<String, bool> = dfs_record.marked;
    match marked.get(&v) {
        None => false,
        Some(is_m) => !is_m,
    }
}

fn is_vid_in_prediction<N, C>(
    u: &String,
    vid_opt: &Option<String>,
    dfs_record: &DfsForestMaps<C, N>,
) -> bool
where
    N: NodeTrait,
    C: CycleInfoTrait,
{
    let pred: &HashMap<String, Option<String>> = dfs_record.pred;
    match vid_opt {
        None => false,
        Some(vid) => match pred.get(&u) {
            None => false,
            Some(upred_opt) => match upred_opt {
                None => false,
                Some(upred) => *upred != vid,
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::types::edge::Edge;
    use crate::graph::types::edgetype::EdgeType;
    use crate::graph::types::graph::Graph;
    use crate::graph::types::node::Node;

    //
    fn mk_node(n_id: &str) -> Node {
        Node::empty(n_id)
    }
    fn mk_nodes(ns: Vec<&str>) -> HashSet<Node> {
        let mut hs: HashSet<Node> = HashSet::new();
        for n in ns {
            hs.insert(mk_node(n));
        }
        hs
    }
    fn mk_uedge(n1_id: &str, n2_id: &str, e_id: &str) -> Edge<Node> {
        Edge::empty(e_id, EdgeType::Undirected, n1_id, n2_id)
    }
    fn mk_edges(es: Vec<Edge<Node>>) -> HashSet<Edge<Node>> {
        let mut hs = HashSet::new();
        for e in es {
            hs.insert(e);
        }
        hs
    }
    fn mk_g1() -> Graph<Node, Edge<Node>> {
        let e1 = mk_uedge("n1", "n3", "e1");
        let e2 = mk_uedge("n2", "n3", "e2");
        let e3 = mk_uedge("n2", "n4", "e3");
        let nset = mk_nodes(vec!["n1", "n2", "n3", "n4"]);
        let h1 = HashMap::new();
        let h2 = mk_edges(vec![e1, e2, e3]);
        Graph::new("g1".to_string(), h1, nset, h2)
    }

    /// Alan Gibbons, Algorithmic graph theory 1985, p. 22, fig. 1.16
    /// depth first undirected graph
    fn mk_ugraph() -> Graph<Node, Edge<Node>> {
        let ns = mk_nodes(vec![
            "n1", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9", "n10", "n11", "n12", "n13",
        ]);
        let es = mk_edges(vec![
            mk_uedge("n1", "n4", "n1n4"),
            mk_uedge("n1", "n3", "n1n3"),
            mk_uedge("n1", "n2", "n1n2"),
            mk_uedge("n1", "n5", "n1n5"),
            mk_uedge("n1", "n6", "n1n6"),
            mk_uedge("n1", "n7", "n1n7"),
            mk_uedge("n1", "n8", "n1n8"),
            mk_uedge("n8", "n2", "n8n2"),
            mk_uedge("n9", "n10", "n9n10"),
            mk_uedge("n9", "n13", "n9n13"),
            mk_uedge("n10", "n11", "n10n11"),
            mk_uedge("n10", "n12", "n10n12"),
        ]);
        let h1 = HashMap::new();
        Graph::new("g1".to_string(), h1, ns, es)
    }

    //
    #[test]
    fn test_depth_first_search() {
        let ugraph = mk_ugraph();
    }
}
