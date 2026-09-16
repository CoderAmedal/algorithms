// import visualization libraries {
use algorithm_visualizer::*;
use std::collections::HashSet;
// }

fn main() {
    // define tracer variables {
    let tracer = GraphTracer::new("Graph");
    tracer.directed(Some(false));
    tracer.weighted(Some(true));
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    let g = Graph::new(5, 1.0, Box::new(Integer::default())).directed(false).weighted(true).create_ints();
    tracer.set(&g);
    Tracer::delay();
    // }

    kruskal(&tracer, &logger, &g);
}

fn kruskal(tracer: &GraphTracer, logger: &LogTracer, g: &[Vec<i64>]) {
    let vcount = g.len();

    // Preprocess: sort edges by weight.
    let mut edges: Vec<(usize, usize, i64)> = Vec::new();
    for vi in 0..vcount - 1 {
        for vj in vi + 1..vcount {
            edges.push((vi, vj, g[vi][vj]));
        }
    }
    edges.sort_by_key(|edge| edge.2);

    // Give each vertex a tree to decide if they are already in the same tree.
    let mut t: Vec<HashSet<usize>> = (0..vcount)
        .map(|i| {
            let mut set = HashSet::new();
            set.insert(i);
            set
        })
        .collect();

    let mut wsum = 0i64;
    let mut n = 0usize;
    let mut index = 0usize;
    while n < vcount - 1 && index < edges.len() {
        let (e0, e1, weight) = edges[index]; // Get the edge of min weight
        index += 1;
        // visualize {
        tracer.visit(e0 as i64, Some(e1 as i64), None::<i64>);
        Tracer::delay();
        // }
        if t[e0] == t[e1] {
            // e[0] & e[1] already in the same tree, ignore
            // visualize {
            tracer.leave(e0 as i64, Some(e1 as i64), None::<i64>);
            Tracer::delay();
            // }
            continue;
        }

        // Choose the current edge.
        wsum += weight;

        // Merge tree of e[0] & e[1]
        let mut merged: HashSet<usize> = t[e0].clone();
        merged.extend(t[e1].iter().cloned());
        for &vertex in &merged {
            t[vertex] = merged.clone();
        }

        n += 1;
    }

    // logger {
    logger.println(format!("The sum of all edges is: {}", wsum));
    // }
}
