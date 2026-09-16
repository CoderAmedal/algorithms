// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let tracer = GraphTracer::new("Graph");
    tracer.weighted(Some(true));
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    tracer.log(&logger);
    let g = Graph::new(5, 0.5, Box::new(Integer::new(-2, 5))).weighted(true).create_ints();
    tracer.set(&g);
    Tracer::delay();
    // }

    let max_value = 0x7fffffff_i64;

    let src = Integer::new(0, g.len() as i64 - 1).create_int();
    let mut dest = src;
    while dest == src {
        dest = Integer::new(0, g.len() as i64 - 1).create_int();
    }

    // logger {
    logger.println(format!("finding the shortest path from {} to {}", src, dest));
    // }

    let min_weight = bellman_ford(&tracer, &logger, &g, src, dest, max_value);

    // logger {
    if min_weight == max_value {
        logger.println(format!("there is no path from {} to {}", src, dest));
    } else {
        logger.println(format!("the shortest path from {} to {} is {}", src, dest, min_weight));
    }
    // }
}

fn bellman_ford(
    tracer: &GraphTracer,
    logger: &LogTracer,
    g: &[Vec<i64>],
    src: i64,
    dest: i64,
    max_value: i64,
) -> i64 {
    let n = g.len();
    let mut weights = vec![max_value; n];
    for i in 0..n {
        weights[i] = max_value;
        // visualize {
        tracer.update_node(i as i64, Some(weights[i]), None, None);
        // }
    }
    weights[src as usize] = 0;
    // visualize {
    tracer.update_node(src, Some(0i64), None, None);
    // }

    // logger {
    logger.println(format!("Initializing weights to: [{}]", join(&weights)));
    logger.println("");
    // }

    // begin BF algorithm execution
    let mut k = n;
    while k > 0 {
        k -= 1;
        // logger {
        logger.println(format!("Iteration: {}", n - k));
        logger.println("------------------------------------------------------------------");
        // }

        for i in 0..n {
            for j in 0..n {
                if g[i][j] != 0 {
                    if weights[j] > weights[i] + g[i][j] {
                        weights[j] = weights[i] + g[i][j];
                        // logger {
                        logger.println(format!("weights[{}] = weights[{}] + {}", j, i, g[i][j]));
                        // }
                    }
                    // visualize {
                    tracer.visit(j as i64, Some(i as i64), Some(weights[j]));
                    Tracer::delay();
                    tracer.leave(j as i64, Some(i as i64), None::<i64>);
                    Tracer::delay();
                    // }
                }
            }
        }

        // logger {
        logger.println(format!("updated weights: [{}]", join(&weights)));
        logger.println("");
        // }
    }

    // check for cycle
    logger.println("checking for cycle");
    for i in 0..n {
        for j in 0..n {
            if g[i][j] != 0 {
                if weights[j] > weights[i] + g[i][j] {
                    // logger {
                    logger.println(format!("A cycle was detected: weights[{}] > weights[{}] + {}", j, i, g[i][j]));
                    // }
                    return max_value;
                }
            }
        }
    }

    // logger {
    logger.println(format!(
        "No cycles detected. Final weights for the source {} are: [{}]",
        src,
        join(&weights)
    ));
    // }

    weights[dest as usize]
}

fn join(values: &[i64]) -> String {
    values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(", ")
}
