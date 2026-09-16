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
    let g = Graph::new(10, 0.4, Box::new(Integer::default())).directed(false).weighted(true).create_ints();
    tracer.set(&g);
    Tracer::delay();
    // }

    // logger {
    logger.println("nodes that belong to minimum spanning tree are: ");
    // }
    prim(&tracer, &logger, &g);
}

fn prim(tracer: &GraphTracer, logger: &LogTracer, g: &[Vec<i64>]) {
    // Finds a tree so that there exists a path between
    // every two nodes while keeping the cost minimal
    let n = g.len();
    let mut sum = 0i64;
    let mut d = vec![0i64; n];
    d[0] = 1; // First node is visited
    for _ in 0..n - 1 {
        // Searching for k edges
        let mut min_d = i64::MAX;
        let mut min_i = 0usize;
        let mut min_j = 0usize;
        for i in 0..n {
            if d[i] != 0 {
                // First node in an edge must be visited
                for j in 0..n {
                    if d[j] == 0 && g[i][j] != 0 {
                        // visualize {
                        tracer.visit(i as i64, Some(j as i64), None::<i64>);
                        Tracer::delay();
                        // }
                        // Second node must not be visited and must be connected to first node
                        if g[i][j] < min_d {
                            // Searching for cheapest edge which satisfies requirements
                            min_d = g[i][j];
                            min_i = i;
                            min_j = j;
                        }
                        // visualize {
                        tracer.leave(i as i64, Some(j as i64), None::<i64>);
                        Tracer::delay();
                        // }
                    }
                }
            }
        }
        // visualize {
        tracer.visit(min_i as i64, Some(min_j as i64), None::<i64>);
        Tracer::delay();
        // }
        d[min_j] = 1; // Visit second node and insert it into or tree
        sum += g[min_i][min_j];
    }
    // logger {
    logger.println(format!("The sum of all edges is: {}", sum));
    // }
}
