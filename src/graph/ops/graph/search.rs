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
fn dfs_forest<N, E, G, F, C>(
    g: &G,
    dfs_record: &mut DfsForestMaps<C, N>,
    u: &String,
    time: &mut usize,
    edge_generator: &F,
    check_cycle: bool,
) -> Option<(String, String)>
where
    N: NodeTrait,
    E: EdgeTrait<N>,
    G: GraphTrait<N, E>,
    F: Fn(&N) -> HashSet<&E>,
    C: CycleInfoTrait,
{
    let marked: &mut HashMap<String, bool> = dfs_record.marked;
    marked.insert(u.to_string(), true);
    *time += 1;
    let d: &mut HashMap<String, usize> = dfs_record.first_visit;
    let vertices: &mut HashMap<String, &N> = dfs_record.vertices;
    d.insert(u.to_string(), *time);
    match vertices.get(u) {
        None => panic!("node not in vertices"),
        Some(unode_some) => {
            let unode: &N = unode_some.clone();
            let edges: HashSet<&E> = edge_generator(unode);
            for edge in &edges {
                let vnode = get_other(edge.clone(), unode);
                let v: &String = vnode.id();
                match marked.get(v) {
                    None => panic!("node not in vertices"),
                    Some(mark) => {
                        if !mark {
                            insert_to_pred_identifiers(
                                v.to_string(),
                                u.to_string(),
                                g,
                                dfs_record,
                                time,
                                edge_generator,
                                check_cycle,
                            );
                        }
                    }
                }
            }
            //
            *time += 1;
            f.insert(u.to_string(), time);
            if check_cycle {
                // v ancestor, u visiting node
                // edge between them is a back edge
                // see p. 151, and p. 159-160
                // unode = V[u]
                for edge in &edges {
                    let vnode = get_other(edge.clone(), unode);
                    let vid: &String = vnode.id();
                    match pred.get(u) {
                        None => panic!("node not in pred parent"),
                        Some(unode_parent_opt) => {
                            match unode_parent_opt {
                                None => panic!("parent node is empty"),
                                Some(unode_parent) => {
                                    if vid == unode_parent {
                                        let first_visit = d.get(vid);
                                        let last_visit = f.get(vid);
                                        match f.get(u) {
                                            None => panic!("node not in last visit times"),
                                            Some(u_last_visit) => match first_visit {
                                                None => panic!("node not in last visit times"),
                                                Some(first_v) => {
                                                    if first_v < u_last_visit {
                                                        let ancestor = vid.to_string();
                                                        let ancestor_first_time = first_v.clone();
                                                        let ancestor_last_time =
                                                            last_visit.copied();
                                                        let current_final_time =
                                                            u_last_visit.clone();
                                                        let before = u.to_string();
                                                        let info = C::create(
                                                            ancestor,
                                                            before,
                                                            ancestor_first_time,
                                                            ancestor_last_time,
                                                            current_final_time,
                                                        );
                                                        //
                                                        match cycles.get(u) {
                                                            None => panic!("node not in cycles"),
                                                            Some(ucyc) => {
                                                                let mut uc = ucyc.clone();
                                                                uc.push(info);
                                                                cycles.insert(
                                                                    u.to_string(),
                                                                    uc.to_vec(),
                                                                );
                                                            }
                                                        }
                                                    }
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                None
            } else {
                None
            }
        }
    }
}

fn fix_edges<N, E, G, F, C>(
    g: &G,
    unode_some: &N,
    edge_generator: &F,
    dfs_record: &mut DfsForestMaps<C, N>,
    u: String,
    time: &mut usize,
    check_cycle: bool,
) -> ()
where
    N: NodeTrait,
    E: EdgeTrait<N>,
    G: GraphTrait<N, E>,
    F: Fn(&N) -> HashSet<&E>,
    C: CycleInfoTrait,
{
    let unode: &N = unode_some;
    let edges: HashSet<&E> = edge_generator(unode);
    let marked: &mut HashMap<String, bool> = dfs_record.marked;
    for edge in &edges {
        let vnode = get_other(edge.clone(), unode);
        let v: &String = vnode.id();
        match marked.get(v) {
            None => panic!("node not in vertices"),
            Some(mark) => {
                if !mark {
                    insert_to_pred_identifiers(
                        v.to_string(),
                        u.to_string(),
                        g,
                        dfs_record,
                        time,
                        edge_generator,
                        check_cycle,
                    );
                }
            }
        }
    }
}

fn insert_to_pred_identifiers<N, E, G, F, C>(
    v: String,
    u: String,
    g: &G,
    dfs_record: &mut DfsForestMaps<C, N>,
    time: &mut usize,
    edge_generator: &F,
    check_cycle: bool,
) -> Option<(String, String)>
where
    N: NodeTrait,
    E: EdgeTrait<N>,
    G: GraphTrait<N, E>,
    F: Fn(&N) -> HashSet<&E>,
    C: CycleInfoTrait,
{
    let pred: &mut HashMap<String, Option<String>> = dfs_record.pred;
    let identifiers: &mut HashSet<String> = dfs_record.identifiers;
    pred.insert(v.clone(), Some(u.to_string()));
    identifiers.insert(v.clone());
    dfs_forest(g, dfs_record, &v, time, edge_generator, check_cycle)
}

fn get_vertex_lst<'a, N: NodeTrait>(
    vs: HashMap<String, &'a N>,
    start_node: Option<&'a N>,
) -> (Vec<String>, HashMap<String, &'a N>) {
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
    for u in vlist {
        match marked.get(&u) {
            None => {
                panic!("key not exist in marked")
            }
            Some(val_ref) => {
                if !val_ref {
                    let mut identifiers: HashSet<String> = HashSet::new();
                    let mut pred: HashMap<String, Option<String>> =
                        vertices.keys().map(|x| (x.clone(), None)).collect();
                    dfs_forest(
                        g,
                        &mut vertices,
                        &u,
                        &mut pred,
                        &mut marked,
                        &mut d,
                        &mut f,
                        &mut identifiers,
                        &mut cycles,
                        time,
                        edge_generator,
                        check_cycle,
                    );
                }
            }
        }
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
