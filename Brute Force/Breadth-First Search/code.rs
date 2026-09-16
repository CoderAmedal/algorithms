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

    bfs(&tracer, &g, 0);
}

fn bfs(tracer: &GraphTracer, g: &[Vec<i64>], s: usize) {
    let mut q: Vec<usize> = Vec::new();
    q.push(s); // add start node to queue
    // visualize {
    tracer.visit(s as i64, None::<i64>, None::<i64>);
    Tracer::delay();
    // }
    while !q.is_empty() {
        let node = q.remove(0); // dequeue
        for i in 0..g[node].len() {
            if g[node][i] != 0 {
                // if current node has the i-th node as a child
                q.push(i); // add child node to queue
                // visualize {
                tracer.visit(i as i64, Some(node as i64), None::<i64>);
                Tracer::delay();
                // }
            }
        }
    }
}
