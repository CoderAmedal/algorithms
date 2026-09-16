// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let g = vec![
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
    tracer.log(&logger);
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    tracer.set(&g);
    tracer.layout_tree(Some(0i64), None::<bool>);
    Tracer::delay();
    // }

    dfs(&tracer, &g, 0, None);
}

fn dfs(tracer: &GraphTracer, g: &[Vec<i64>], node: usize, parent: Option<usize>) {
    // visualize {
    tracer.visit(node as i64, parent.map(|p| p as i64), None::<i64>);
    Tracer::delay();
    // }
    for i in 0..g[node].len() {
        if g[node][i] != 0 {
            // if current node has the i-th node as a child
            dfs(tracer, g, i, Some(node)); // recursively call DFS
        }
    }
}
