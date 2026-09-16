// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let g = Graph::new(5, 1.0, Box::new(Integer::default()))
        .directed(false)
        .weighted(true)
        .create_ints();
    let max_value = 1_000_000_000i64;
    let mut s_dist = vec![max_value; g.len()]; // S[end] returns the distance from start node to end node

    // define tracer variables {
    let tracer = GraphTracer::new("Graph");
    tracer.directed(Some(false));
    tracer.weighted(Some(true));
    let tracer_s = Array1DTracer::new("Distances");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &tracer_s, &logger]));
    tracer.log(&logger);
    tracer.set(&g);
    tracer_s.set(&s_dist);
    Tracer::delay();
    // }

    let s = Integer::new(0, g.len() as i64 - 1).create_int(); // s = start node
    let mut e = s; // e = end node
    while e == s {
        e = Integer::new(0, g.len() as i64 - 1).create_int();
    }
    // logger {
    logger.println(format!("finding the shortest path from {} to {}", s, e));
    Tracer::delay();
    // }

    dijkstra(&tracer, &tracer_s, &logger, &g, &mut s_dist, s, e, max_value);
}

fn dijkstra(
    tracer: &GraphTracer,
    tracer_s: &Array1DTracer,
    logger: &LogTracer,
    g: &[Vec<i64>],
    s_dist: &mut Vec<i64>,
    start: i64,
    end: i64,
    max_value: i64,
) {
    let mut discovered = vec![false; g.len()]; // D[i] indicates whether the i-th node is discovered or not
    s_dist[start as usize] = 0; // Starting node is at distance 0 from itself
    // visualize {
    tracer_s.patch(start, Some(s_dist[start as usize]));
    Tracer::delay();
    tracer_s.depatch(start);
    tracer_s.select(start, None);
    // }
    let mut k = g.len();
    while k > 0 {
        k -= 1;
        // Finding a node with the shortest distance from s_dist[min_index]
        let mut min_distance = max_value;
        let mut min_index: i64 = -1;
        for i in 0..g.len() {
            if s_dist[i] < min_distance && !discovered[i] {
                min_distance = s_dist[i];
                min_index = i as i64;
            }
        }
        if min_distance == max_value {
            break; // If there is no edge from current node, jump out of loop
        }
        discovered[min_index as usize] = true;
        // visualize {
        tracer_s.select(min_index, None);
        tracer.visit(min_index, None::<i64>, None::<i64>);
        Tracer::delay();
        // }
        // For every unvisited neighbour of current node, we check
        // whether the path to it is shorter if going over the current node
        for i in 0..g.len() {
            if g[min_index as usize][i] != 0
                && s_dist[i] > s_dist[min_index as usize] + g[min_index as usize][i]
            {
                s_dist[i] = s_dist[min_index as usize] + g[min_index as usize][i];
                // visualize {
                tracer_s.patch(i as i64, Some(s_dist[i]));
                tracer.visit(i as i64, Some(min_index), Some(s_dist[i]));
                Tracer::delay();
                tracer_s.depatch(i as i64);
                tracer.leave(i as i64, Some(min_index), None::<i64>);
                Tracer::delay();
                // }
            }
        }
        // visualize {
        tracer.leave(min_index, None::<i64>, None::<i64>);
        Tracer::delay();
        // }
    }
    // logger {
    if s_dist[end as usize] == max_value {
        logger.println(format!("there is no path from {} to {}", start, end));
    } else {
        logger.println(format!(
            "the shortest path from {} to {} is {}",
            start, end, s_dist[end as usize]
        ));
    }
    // }
}
