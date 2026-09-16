// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let g = vec![
        vec![0, 1, 0, 0, 0, 0],
        vec![1, 0, 0, 1, 1, 0],
        vec![0, 0, 0, 1, 0, 0],
        vec![0, 1, 1, 0, 1, 1],
        vec![0, 1, 0, 1, 0, 0],
        vec![0, 0, 0, 1, 0, 0],
    ];

    // define tracer variables {
    let tracer = GraphTracer::new("Graph");
    tracer.directed(Some(false));
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    tracer.set(&g);
    Tracer::delay();
    // }

    let bridges = find_bridges(&tracer, &logger, &g);

    // logger {
    logger.println("The bridges are: ");
    for i in 0..bridges.len() {
        logger.println(format!("{} to {}", bridges[i].0, bridges[i].1));
    }
    logger.println("NOTE: A bridge is both ways, i.e., from A to B and from B to A, because this is an Undirected Graph");
    // }
}

// Depth First Search Exploration Algorithm to test connectedness of the Graph (see Graph Algorithms/DFS/exploration), without the tracer & logger commands
fn dfs_explore(graph: &[Vec<i64>], source: usize) -> Vec<bool> {
    let mut stack: Vec<(usize, Option<usize>)> = vec![(source, None)];
    let mut visited = vec![false; graph.len()];

    while let Some((node, _prev)) = stack.pop() {
        if !visited[node] {
            visited[node] = true;

            for i in 0..graph.len() {
                if graph[node][i] != 0 {
                    stack.push((i, Some(node)));
                }
            }
        }
    }

    visited
}

fn find_bridges(
    tracer: &GraphTracer,
    logger: &LogTracer,
    graph: &[Vec<i64>],
) -> Vec<(i64, i64)> {
    let mut bridges: Vec<(i64, i64)> = Vec::new();

    for i in 0..graph.len() {
        for j in 0..graph.len() {
            if graph[i][j] != 0 {
                // check if an edge exists
                // visualize {
                logger.println(format!("Deleting edge {}->{} and calling DFSExplore ()", i, j));
                tracer.visit(j as i64, Some(i as i64), None::<i64>);
                Tracer::delay();
                tracer.leave(j as i64, Some(i as i64), None::<i64>);
                Tracer::delay();
                // }

                let mut temp_graph: Vec<Vec<i64>> = graph.to_vec();
                temp_graph[i][j] = 0;
                temp_graph[j][i] = 0;
                let visited = dfs_explore(&temp_graph, 0);

                let count = visited.iter().filter(|&&value| value).count();
                if count == graph.len() {
                    // logger {
                    logger.println("Graph is CONNECTED. Edge is NOT a bridge");
                    // }
                } else {
                    // logger {
                    logger.println("Graph is DISCONNECTED. Edge IS a bridge");
                    // }
                    bridges.push((i as i64, j as i64));
                }
            }
        }
    }

    bridges
}
