// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let g = Graph::new(5, 0.4, Box::new(Integer::default()))
        .directed(true)
        .weighted(false)
        .create_ints();
    let mut outgoing_edge_counts = vec![0i64; g.len()];
    let mut incoming_nodes: Vec<Vec<i64>> = (0..g.len()).map(|_| vec![-1i64; g.len()]).collect();
    let mut ranks: Vec<f64> = Vec::new();

    // define tracer variables {
    let graph_tracer = GraphTracer::new("Web Page inter-connections");
    let rank_tracer = Array1DTracer::new("Web Page Ranks");
    let oec_tracer = Array1DTracer::new("Outgoing Edge Counts");
    let in_tracer = Array2DTracer::new("Incoming Nodes");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![
        &graph_tracer,
        &rank_tracer,
        &oec_tracer,
        &in_tracer,
        &logger
    ]));

    graph_tracer.set(&g);
    oec_tracer.set(&outgoing_edge_counts);
    in_tracer.set(&incoming_nodes);
    Tracer::delay();
    // }

    // PageRank Algorithm Version 2
    // Equation:
    //   PR (X) = ( (1 - D)/N ) + D (Summation i->X (PR (I) / Out (i)))
    // NOTE: Algorithm uses the recommended damping factor (D). Number of iterations is small because only a small Web of 5 Pages is simulated

    // PRECOMPUTATIONS

    // logger {
    logger.println("Calculate Outgoing Edge Count for each Node");
    // }
    for i in 0..g.len() {
        outgoing_edge_counts[i] = array_sum(&g[i]);
        show_outgoing_edges(&graph_tracer, &g, i);

        // visualize {
        oec_tracer.patch(i as i64, Some(outgoing_edge_counts[i]));
        Tracer::delay();
        oec_tracer.depatch(i as i64);
        Tracer::delay();
        // }
    }

    // logger {
    logger.println("determine incoming nodes for each node");
    // }
    for i in 0..g.len() {
        for j in 0..g.len() {
            if g[i][j] != 0 {
                // there's an edge FROM i TO j
                // visualize {
                graph_tracer.visit(j as i64, Some(i as i64), None::<i64>);
                Tracer::delay();
                // }

                let next_pos = incoming_nodes[j].iter().position(|&v| v == -1).unwrap();
                incoming_nodes[j][next_pos] = i as i64;
                // visualize {
                in_tracer.patch(j as i64, next_pos as i64, Some(i as i64));
                Tracer::delay();
                in_tracer.depatch(j as i64, next_pos as i64);
                Tracer::delay();

                graph_tracer.leave(j as i64, Some(i as i64), None::<i64>);
                Tracer::delay();
                // }
            }
        }
    }

    // logger.println ('All -1s will be removed from incoming node records, they are irrelevant');
    for arr in incoming_nodes.iter_mut() {
        if let Some(index) = arr.iter().position(|&v| v == -1) {
            arr.truncate(index);
        } else {
            arr.pop();
        }
    }

    let damping = 0.85;
    let mut iterations = 7;
    let initial_rank = 1.0;

    // logger {
    logger.println(format!("Initialized all Page ranks to {}", initial_rank));
    // }
    ranks = vec![initial_rank; g.len()];

    // visualize {
    rank_tracer.set(&ranks);
    // }
    // logger {
    logger.println("Begin execution of PageRank Version #1");
    logger.println("Equation used: PR (X) = (1 - D) + D (In-Node-Summation i->X (PR (I) / Out (i)))");
    logger.println("D = Damping Factor, PR (X) = Page rank of Node X, i = the ith In-Node of X, Out (i) = outgoing Edge Count of i");
    logger.println("");
    // }

    while iterations > 0 {
        iterations -= 1;
        for node in 0..ranks.len() {
            let new_rank = update_rank(
                &in_tracer,
                &oec_tracer,
                &logger,
                &outgoing_edge_counts,
                &incoming_nodes,
                &ranks,
                node,
                damping,
            );
            ranks[node] = new_rank;
            // visualize {
            rank_tracer.patch(node as i64, Some(ranks[node]));
            Tracer::delay();
            rank_tracer.patch(node as i64, None::<f64>);
            Tracer::delay();
            // }
        }
    }

    // logger {
    logger.println("Page Ranks have been converged to.");
    for node in 0..ranks.len() {
        logger.println(format!("Rank of Node #{} = {}", node, ranks[node]));
    }
    logger.println("Done");
    // }
}

fn array_sum(array: &[i64]) -> i64 {
    // if curr is 0 (no edge) or undefined (loop not allowed), sum remains unchanged
    array.iter().filter(|&&value| value != 0).count() as i64
}

fn show_outgoing_edges(graph_tracer: &GraphTracer, g: &[Vec<i64>], i: usize) {
    for j in 0..g[i].len() {
        if g[i][j] != 0 {
            // visualize {
            graph_tracer.visit(j as i64, Some(i as i64), None::<i64>);
            Tracer::delay();
            graph_tracer.leave(j as i64, Some(i as i64), None::<i64>);
            Tracer::delay();
            // }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn update_rank(
    in_tracer: &Array2DTracer,
    oec_tracer: &Array1DTracer,
    logger: &LogTracer,
    outgoing_edge_counts: &[i64],
    incoming_nodes: &[Vec<i64>],
    ranks: &[f64],
    node_index: usize,
    damping: f64,
) -> f64 {
    let mut in_node_summation = 0.0f64;

    // logger {
    logger.println(format!("Updating rank of {}", node_index));
    logger.println(format!("The incoming Nodes of {} are being highlighted", node_index));
    // }

    for i in 0..incoming_nodes[node_index].len() {
        let incoming = incoming_nodes[node_index][i] as usize;
        // visualize {
        in_tracer.select(node_index as i64, i as i64, None::<i64>, None::<i64>);
        Tracer::delay();
        logger.println(format!(
            "Outgoing edge count of {} is {}",
            incoming, outgoing_edge_counts[incoming]
        ));
        oec_tracer.select(incoming as i64, None::<i64>);
        Tracer::delay();
        // }

        in_node_summation += ranks[incoming] / outgoing_edge_counts[incoming] as f64;

        // visualize {
        oec_tracer.deselect(incoming as i64, None::<i64>);
        Tracer::delay();
        in_tracer.deselect(node_index as i64, i as i64, None::<i64>, None::<i64>);
        Tracer::delay();
        // }
    }
    // logger {
    logger.println(format!("In-Node summation of {} = {}", node_index, in_node_summation));
    // }

    let result = ((1.0 - damping) / incoming_nodes.len() as f64) + (damping * in_node_summation);
    // logger {
    logger.println(format!("Therefore, using Equation, new rank of {} = {}", node_index, result));
    // }
    result
}
