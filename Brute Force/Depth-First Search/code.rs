// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let g = Graph::new(8, 0.3, Box::new(Integer::default()))
        .directed(false)
        .weighted(false)
        .create_ints();

    // define tracer variables {
    let graph_tracer = GraphTracer::new("Graph");
    graph_tracer.directed(Some(false));
    let visited_tracer = Array1DTracer::new("visited");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&graph_tracer, &visited_tracer, &logger]));
    graph_tracer.log(&logger);
    graph_tracer.set(&g);
    Tracer::delay();
    // }

    let visited = dfs_explore(&graph_tracer, &visited_tracer, &g, 0);
    let mut check = true;
    for i in 0..visited.len() {
        check &= visited[i];
    }
    // logger {
    if check {
        logger.println("The Graph is CONNECTED");
    } else {
        logger.println("The Graph is NOT CONNECTED");
    }
    // }
}

fn dfs_explore(
    graph_tracer: &GraphTracer,
    visited_tracer: &Array1DTracer,
    graph: &[Vec<i64>],
    source: usize,
) -> Vec<bool> {
    let mut stack: Vec<(usize, Option<usize>)> = vec![(source, None)];
    let mut visited = vec![false; graph.len()];
    // visualize {
    visited_tracer.set(&visited);
    // }

    while let Some((node, prev)) = stack.pop() {
        if !visited[node] {
            visited[node] = true;
            // visualize {
            visited_tracer.patch(node as i64, Some(visited[node]));

            if let Some(p) = prev {
                if graph[node][p] != 0 {
                    graph_tracer.visit(node as i64, Some(p as i64), None::<i64>);
                    Tracer::delay();
                } else {
                    graph_tracer.visit(node as i64, None::<i64>, None::<i64>);
                    Tracer::delay();
                }
            } else {
                graph_tracer.visit(node as i64, None::<i64>, None::<i64>);
                Tracer::delay();
            }
            // }

            for i in 0..graph.len() {
                if graph[node][i] != 0 {
                    stack.push((i, Some(node)));
                }
            }
        }
    }

    visited
}
