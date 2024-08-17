//! graph searching
use crate::graph::ops::edge::nodeops::get_other;
use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::graph::Graph as GraphTrait;
use crate::graph::traits::graph_obj::GraphObject;
use crate::graph::traits::node::Node as NodeTrait;
use std::collections::HashMap;
use std::collections::HashSet;
use std::option::Option;

/// holds information about cycles in the graph
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CycleInfo {
    ancestor: String,
    before: String,
    ancestor_first_time_visit: usize,
    ancestor_last_time_visit: Option<usize>,
    current_final_time_visit: usize,
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
fn dfs_forest<N, E, G, F>(
    g: &G,
    vertices: &mut HashMap<String, &N>,
    u: &String,
    pred: &mut HashMap<String, String>,
    marked: &mut HashMap<String, bool>,
    d: &mut HashMap<String, usize>,
    f: &mut HashMap<String, usize>,
    identifiers: &mut HashSet<String>,
    cycles: &mut HashMap<String, Vec<CycleInfo>>,
    mut time: usize,
    edge_generator: &F,
    check_cycle: bool,
) -> Option<(String, String)>
where
    N: NodeTrait,
    E: EdgeTrait<N>,
    G: GraphTrait<N, E>,
    F: Fn(&N) -> HashSet<&E>,
{
    marked.insert(u.to_string(), true);
    time += 1;
    d.insert(u.to_string(), time);
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
                            pred.insert(v.clone(), u.to_string());
                            identifiers.insert(v.clone());
                            dfs_forest(
                                g,
                                vertices,
                                v,
                                pred,
                                marked,
                                d,
                                f,
                                identifiers,
                                cycles,
                                time,
                                edge_generator,
                                check_cycle,
                            );
                        }
                    }
                }
            }
            //
            time += 1;
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
                                                let fv = first_v.clone();
                                                let lv = last_visit.copied();
                                                let ulv = u_last_visit.clone();
                                                let info = CycleInfo {
                                                    ancestor: ancestor,
                                                    before: u.to_string(),
                                                    ancestor_first_time_visit: fv,
                                                    ancestor_last_time_visit: lv,
                                                    current_final_time_visit: ulv,
                                                };
                                                //
                                                match cycles.get(u) {
                                                    None => panic!("node not in cycles"),
                                                    Some(ucyc) => {
                                                        let mut uc = ucyc.clone();
                                                        uc.push(info);
                                                        cycles.insert(u.to_string(), uc.to_vec());
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
                None
            } else {
                None
            }
        }
    }
}
