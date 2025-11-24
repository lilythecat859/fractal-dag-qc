use petgraph::graph::{Graph, NodeIndex};
use rand::prelude::*;

pub fn build_expander(n: usize) -> Graph<(), ()> {
    let mut g = Graph::new();
    let nodes: Vec<_> = (0..n).map(|_| g.add_node(())).collect();
    let mut rng = thread_rng();
    for u in nodes.iter() {
        for _ in 0..3 {
            let v = nodes[rng.gen_range(0..n)];
            g.add_edge(*u, v, ());
        }
    }
    g
}
