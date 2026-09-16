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
    let g = Graph::new(5, 1.0, Box::new(Integer::default()))
        .directed(false)
        .weighted(true)
        .create_ints();
    tracer.set(&g);
    Tracer::delay();
    // }

    let s = Integer::new(0, g.len() as i64 - 1).create_int(); // s = start node
    let mut e = s; // e = end node
    while e == s {
        e = Integer::new(0, g.len() as i64 - 1).create_int();
    }
    let max_value = i64::MAX;
    let mut min_weight = max_value;
    // logger {
    logger.println(format!("finding the shortest path from {} to {}", s, e));
    // }
    let mut d = vec![false; g.len()]; // D[i] indicates whether the i-th node is discovered or not
    dfs(&tracer, &g, &mut d, s as usize, None, 0, e, &mut min_weight);
    // logger {
    if min_weight == max_value {
        logger.println(format!("there is no path from {} to {}", s, e));
    } else {
        logger.println(format!(
            "the shortest path from {} to {} is {}",
            s, e, min_weight
        ));
    }
    // }
}

#[allow(clippy::too_many_arguments)]
fn dfs(
    tracer: &GraphTracer,
    g: &[Vec<i64>],
    d: &mut Vec<bool>,
    node: usize,
    parent: Option<usize>,
    weight: i64,
    e: i64,
    min_weight: &mut i64,
) {
    if *min_weight < weight {
        return;
    }
    if node as i64 == e {
        // visualize {
        tracer.visit(node as i64, parent.map(|p| p as i64), Some(weight));
        Tracer::delay();
        // }
        if *min_weight > weight {
            *min_weight = weight;
        }
        // visualize {
        tracer.leave(node as i64, parent.map(|p| p as i64), Some(*min_weight));
        Tracer::delay();
        // }
        return;
    }
    d[node] = true; // label current node as discovered
    // visualize {
    tracer.visit(node as i64, parent.map(|p| p as i64), Some(weight));
    Tracer::delay();
    // }
    for i in 0..g[node].len() {
        if g[node][i] != 0 {
            // if the path from current node to the i-th node exists
            if !d[i] {
                // if the i-th node is not labeled as discovered
                dfs(
                    tracer,
                    g,
                    d,
                    i,
                    Some(node),
                    weight + g[node][i],
                    e,
                    min_weight,
                ); // recursively call DFS
            }
        }
    }
    d[node] = false; // label current node as undiscovered
    // visualize {
    tracer.leave(node as i64, parent.map(|p| p as i64), Some(0i64));
    Tracer::delay();
    // }
}
