// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // G[i][j] indicates whether the path from the i-th node to the j-th node exists or not
    let g: Vec<Vec<i64>> = vec![
        vec![0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    // define tracer variables {
    let tracer = GraphTracer::new("Graph");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    tracer.log(&logger);
    tracer.set(&g);
    tracer.layout_tree(Some(0), None);
    Tracer::delay();
    // }

    // logger {
    logger.println(format!(
        "Number of descendant is {}",
        dls_count(&tracer, &logger, &g, 2, 0, None)
    ));
    // }
}

// This is a sample DLS applications where
// we try to find number of descendant of root within some depth
fn dls_count(
    tracer: &GraphTracer,
    logger: &LogTracer,
    g: &[Vec<i64>],
    limit: i64,
    node: usize,
    parent: Option<i64>,
) -> i64 {
    // node = current node, parent = previous node
    // visualize {
    tracer.visit(node as i64, parent, None::<i64>);
    Tracer::delay();
    // }
    let mut child = 0i64;
    if limit > 0 {
        // cut off the search
        for i in 0..g[node].len() {
            if g[node][i] != 0 {
                // if current node has the i-th node as a child
                child += 1 + dls_count(tracer, logger, g, limit - 1, i, Some(node as i64)); // recursively call DLS
            }
        }
        return child;
    }
    child
}
