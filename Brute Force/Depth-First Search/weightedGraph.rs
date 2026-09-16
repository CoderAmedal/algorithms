// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let tracer = GraphTracer::new("Graph");
    tracer.directed(Some(false));
    tracer.weighted(Some(true));
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    tracer.log(&logger);
    let g = Graph::new(5, 1.0, Box::new(Integer::default()))
        .directed(false)
        .weighted(true)
        .create_ints();
    tracer.set(&g);
    Tracer::delay();
    // }

    for i in 0..g.len() {
        // start from every node
        // logger {
        logger.println(format!("start from {}", i));
        // }
        let mut d = vec![false; g.len()]; // D[i] indicates whether the i-th node is discovered or not
        dfs(&tracer, &g, &mut d, i, None, 0);
    }
}

fn dfs(
    tracer: &GraphTracer,
    g: &[Vec<i64>],
    d: &mut Vec<bool>,
    node: usize,
    parent: Option<usize>,
    weight: i64,
) {
    // visualize {
    tracer.visit(node as i64, parent.map(|p| p as i64), Some(weight));
    Tracer::delay();
    // }
    d[node] = true; // label current node as discovered
    for i in 0..g[node].len() {
        if g[node][i] != 0 {
            // if the edge from current node to the i-th node exists
            if !d[i] {
                // if the i-th node is not labeled as discovered
                dfs(tracer, g, d, i, Some(node), weight + g[node][i]); // recursively call DFS
            }
        }
    }
    d[node] = false; // label current node as undiscovered
    // visualize {
    tracer.leave(node as i64, parent.map(|p| p as i64), Some(0i64));
    Tracer::delay();
    // }
}
