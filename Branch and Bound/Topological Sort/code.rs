// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // G[i][j] indicates whether the path from the i-th node to the j-th node exists or not. NOTE: The graph must be Directed-Acyclic
    let g: Vec<Vec<i64>> = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 1, 0, 0],
        vec![1, 1, 0, 0, 0, 0],
    ];

    // define tracer variables {
    let tracer = GraphTracer::new("Graph");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    tracer.log(&logger);
    tracer.set(&g);
    Tracer::delay();
    // }

    let n = g.len();
    let mut in_degrees = vec![0i64; n]; // create an Array of n number of 0s
    let mut q: Vec<i64> = Vec::new();
    let mut iter = 0;

    // logger {
    logger.println("Calculating in-degrees for each Node...");
    // }

    for curr_node in 0..n {
        for curr_node_neighbor in 0..n {
            if g[curr_node][curr_node_neighbor] != 0 {
                // visualize {
                logger.println(format!(
                    "{} has an incoming edge from {}",
                    curr_node_neighbor, curr_node
                ));
                tracer.visit(
                    curr_node_neighbor as i64,
                    Some(curr_node as i64),
                    None::<i64>,
                );
                Tracer::delay();
                // }
                in_degrees[curr_node_neighbor] += 1;
                // visualize {
                tracer.leave(
                    curr_node_neighbor as i64,
                    Some(curr_node as i64),
                    None::<i64>,
                );
                Tracer::delay();
                // }
            }
        }
    }
    // logger {
    logger.println(format!("Done. In-Degrees are: [ {} ]", join(&in_degrees)));
    logger.println("");

    logger.println("Initializing queue with all the sources (nodes with no incoming edges)");
    // }
    for node in 0..in_degrees.len() {
        // visualize {
        tracer.visit(node as i64, None::<i64>, None::<i64>);
        Tracer::delay();
        // }
        if in_degrees[node] == 0 {
            // logger {
            logger.println(format!("{} is a source", node));
            // }
            q.push(node as i64);
        }
        // visualize {
        tracer.leave(node as i64, None::<i64>, None::<i64>);
        Tracer::delay();
        // }
    }
    // logger {
    logger.println(format!("Done. Initial State of Queue: [ {} ]", join(&q)));
    logger.println("");
    // }

    // begin topological sort (kahn)
    while !q.is_empty() {
        // logger {
        logger.println(format!(
            "Iteration #{}. Queue state: [ {} ]",
            iter,
            join(&q)
        ));
        // }
        let curr_node = q.remove(0);
        // visualize {
        tracer.visit(curr_node, None::<i64>, None::<i64>);
        Tracer::delay();
        // }

        for i in 0..n {
            if g[curr_node as usize][i] != 0 {
                // visualize {
                logger.println(format!(
                    "{} has an incoming edge from {}. Decrementing {}'s in-degree by 1.",
                    i, curr_node, i
                ));
                tracer.visit(i as i64, Some(curr_node), None::<i64>);
                Tracer::delay();
                // }
                in_degrees[i] -= 1;
                // visualize {
                tracer.leave(i as i64, Some(curr_node), None::<i64>);
                Tracer::delay();
                // }

                if in_degrees[i] == 0 {
                    // logger {
                    logger.println(format!("{}'s in-degree is now 0. Enqueuing {}", i, i));
                    // }
                    q.push(i as i64);
                }
            }
        }
        // visualize {
        tracer.leave(curr_node, None::<i64>, None::<i64>);
        Tracer::delay();
        // }
        // logger {
        logger.println(format!("In-degrees are: [{} ]", join(&in_degrees)));
        logger.println("-------------------------------------------------------------------");
        // }

        iter += 1;
    }
}

fn join(values: &[i64]) -> String {
    values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(", ")
}
