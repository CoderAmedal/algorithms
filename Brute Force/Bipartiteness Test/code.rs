// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let g = vec![
        vec![0, 1, 0, 1, 1],
        vec![1, 0, 1, 0, 0],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 0], // <-- replace latest 0 with 1 to make G not biparted
        vec![1, 0, 0, 0, 0],
    ];

    // define tracer variables {
    let tracer = GraphTracer::new("Graph");
    tracer.directed(Some(false));
    let logger = LogTracer::new("Log");
    tracer.log(&logger);
    tracer.set(&g);
    let colors_tracer = Array1DTracer::new("Colors");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger, &colors_tracer]));
    Tracer::delay();
    // }

    bfs_check_bipartiteness(&tracer, &logger, &colors_tracer, &g, 0);
}

fn bfs_check_bipartiteness(
    tracer: &GraphTracer,
    logger: &LogTracer,
    colors_tracer: &Array1DTracer,
    g: &[Vec<i64>],
    s: usize,
) -> bool {
    let mut q: Vec<usize> = Vec::new();

    // Create a new matrix to set colors (0,1)
    let mut colors = vec![-1i64; g.len()];
    // visualize {
    colors_tracer.set(&colors);
    // }

    colors[s] = 1;
    // visualize {
    colors_tracer.patch(s as i64, Some(1i64));
    // }

    q.push(s); // add start node to queue

    while !q.is_empty() {
        let node = q.remove(0); // dequeue
        // visualize {
        tracer.visit(node as i64, None::<i64>, None::<i64>);
        Tracer::delay();
        // }

        for i in 0..g[node].len() {
            if g[node][i] != 0 {
                if colors[i] == -1 {
                    colors[i] = 1 - colors[node];
                    // visualize {
                    colors_tracer.patch(i as i64, Some(colors[i]));
                    // }

                    q.push(i);
                    // visualize {
                    tracer.visit(i as i64, Some(node as i64), None::<i64>);
                    Tracer::delay();
                    // }
                } else if colors[i] == colors[node] {
                    // logger {
                    logger.println("Graph is not biparted");
                    // }
                    return false;
                }
            }
        }
    }

    // logger {
    logger.println("Graph is biparted");
    // }
    true
}
