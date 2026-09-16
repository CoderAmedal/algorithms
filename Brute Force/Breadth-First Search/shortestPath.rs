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
    let mut e = s; // e = start node
    while e == s {
        e = Integer::new(0, g.len() as i64 - 1).create_int();
    }
    let max_value = 0x7fffffffi64;
    // logger {
    logger.println(format!("finding the shortest path from {} to {}", s, e));
    // }
    let min_weight = bfs(&tracer, &g, s, e, max_value);
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

fn bfs(tracer: &GraphTracer, g: &[Vec<i64>], s: i64, e: i64, max_value: i64) -> i64 {
    let mut w = vec![max_value; g.len()]; // W[i] indicates the length of the shortest path from start node to the i-th node
    let mut q: Vec<usize> = Vec::new();
    for i in 0..g.len() {
        // visualize {
        tracer.update_node(i as i64, Some(max_value), None::<i64>, None::<i64>);
        // }
    }
    w[s as usize] = 0;
    q.push(s as usize); // add start node to queue
    // visualize {
    tracer.visit(s, None::<i64>, Some(0i64));
    Tracer::delay();
    // }
    while !q.is_empty() {
        let node = q.remove(0); // dequeue
        for i in 0..g[node].len() {
            if g[node][i] != 0 {
                // if the edge from current node to the i-th node exists
                if w[i] > w[node] + g[node][i] {
                    // if current path is shorter than the previously shortest path
                    w[i] = w[node] + g[node][i]; // update the length of the shortest path
                    q.push(i); // add child node to queue
                    // visualize {
                    tracer.visit(i as i64, Some(node as i64), Some(w[i]));
                    Tracer::delay();
                    // }
                }
            }
        }
    }
    w[e as usize]
}
