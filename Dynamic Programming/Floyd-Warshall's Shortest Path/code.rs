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
    let g = Graph::new(5, 1.0, Box::new(Integer::default())).weighted(true).create_ints();
    tracer.set(&g);
    Tracer::delay();
    // }

    // logger {
    logger.println("finding the shortest paths from and to all nodes");
    // }
    floyd_warshall(&tracer, &logger, &g);
}

fn floyd_warshall(tracer: &GraphTracer, logger: &LogTracer, g: &[Vec<i64>]) {
    let max_value = i64::MAX / 4;
    let n = g.len();
    // Finds the shortest path between all nodes
    let mut s = vec![vec![max_value; n]; n];
    for i in 0..n {
        for j in 0..n {
            // Distance to self is always 0
            if i == j {
                s[i][i] = 0;
            // Distance between connected nodes is their weight
            } else if g[i][j] > 0 {
                s[i][j] = g[i][j];
            // Else we don't know the distance and we set it to infinity
            } else {
                s[i][j] = max_value;
            }
        }
    }
    // If there is a shorter path using k, use it instead
    for k in 0..n {
        for i in 0..n {
            if k == i {
                continue;
            }
            // visualize {
            tracer.visit(k as i64, Some(i as i64), None::<i64>);
            Tracer::delay();
            // }
            for j in 0..n {
                if i == j || j == k {
                    continue;
                }
                // visualize {
                tracer.visit(j as i64, Some(k as i64), None::<i64>);
                Tracer::delay();
                // }
                if s[i][j] > s[i][k] + s[k][j] {
                    // visualize {
                    tracer.visit(j as i64, Some(i as i64), Some(s[i][j]));
                    Tracer::delay();
                    // }
                    s[i][j] = s[i][k] + s[k][j];
                    // visualize {
                    tracer.leave(j as i64, Some(i as i64), Some(s[i][j]));
                    // }
                }
                // visualize {
                tracer.leave(j as i64, Some(k as i64), None::<i64>);
                // }
            }
            // visualize {
            tracer.leave(k as i64, Some(i as i64), None::<i64>);
            Tracer::delay();
            // }
        }
    }
    // logger {
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == max_value {
                logger.println(format!("there is no path from {} to {}", i, j));
            } else {
                logger.println(format!("the shortest path from {} to {} is {}", i, j, s[i][j]));
            }
        }
    }
    // }
}
