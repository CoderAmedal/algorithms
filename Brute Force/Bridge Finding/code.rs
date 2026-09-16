// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let g = vec![
        vec![0, 1, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 1, 1],
        vec![1, 1, 0, 1, 0, 0],
        vec![0, 0, 0, 1, 0, 0],
    ];

    // define tracer variables {
    let graph_tracer = GraphTracer::new("Graph");
    graph_tracer.directed(Some(false));
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&graph_tracer, &logger]));
    graph_tracer.set(&g);
    Tracer::delay();
    // }

    /*
      NOTE: Code assumes NO parallel edges
    */

    let mut timer = 0i64; // adj keeps track of the neighbors of each node
    let mut bridges: Vec<(i64, i64)> = Vec::new();
    let mut adj: Vec<Vec<usize>> = Vec::new();

    let mut disc = vec![-1i64; g.len()];
    let mut low = vec![-1i64; g.len()];

    // PRECOMPUTATION: store every node's neighbor info in auxiliary array for efficient retrieval later
    for config in &g {
        let mut temp: Vec<usize> = Vec::new();
        for (i, &is_edge) in config.iter().enumerate() {
            if is_edge != 0 {
                temp.push(i);
            }
        }
        adj.push(temp);
    }

    // logger {
    logger.println(format!(
        "Initializing: <b>disc</b>: [{}] <b>low</b>: [{}]",
        join(&disc),
        join(&low)
    ));
    logger.println("");
    logger.println("Beginning efficient Bridge Finding");
    logger.println("NOTE: call to util () follows pattern: util (nodeToVisit, disc, low, parent). See code for clarity");
    logger.println("");

    logger.println("Starting the main for loop (for each node)");
    // }
    for v in 0..g.len() {
        if disc[v] == -1 {
            // logger {
            logger.println(format!(
                "{} has not been visited yet. Calling util ({},  [{}], [{}],{}) from the for loop",
                v,
                v,
                join(&disc),
                join(&low),
                v
            ));
            // }
            util(
                &graph_tracer,
                &logger,
                &adj,
                &mut disc,
                &mut low,
                &mut bridges,
                &mut timer,
                v,
                v as i64,
            );
            // logger {
            logger.println(format!(
                "Returned in for loop after util ({}, [{}], [{}], [{}])",
                v,
                join(&disc),
                join(&low),
                v
            ));
            // }
        }
    }

    // logger {
    logger.println(format!("There are {} bridges in the Graph", bridges.len()));
    for i in 0..bridges.len() {
        logger.println(format!("{}-->{}", bridges[i].0, bridges[i].1));
    }
    logger.println("NOTE: All bridges are both ways (just like in the Naive Algorithm) because the Graph is undirected. So, edge A->B and B->A, both are bridges");
    // }
}

fn trace(graph_tracer: &GraphTracer, v: usize, u: usize) {
    // visualize {
    graph_tracer.visit(v as i64, Some(u as i64), None::<i64>);
    Tracer::delay();
    graph_tracer.leave(v as i64, Some(u as i64), None::<i64>);
    Tracer::delay();
    // }
}

#[allow(clippy::too_many_arguments)]
fn util(
    graph_tracer: &GraphTracer,
    logger: &LogTracer,
    adj: &[Vec<usize>],
    disc: &mut Vec<i64>,
    low: &mut Vec<i64>,
    bridges: &mut Vec<(i64, i64)>,
    timer: &mut i64,
    u: usize,
    parent: i64,
) {
    // u is the node that is currently being processed in the DFS (depth-first search)
    // disc is the numbering of the vertices in the DFS, starting at 0
    // low[v] is the lowest numbered vertex that can be reached from vertex v along the DFS
    // parent is the node that u came from
    // visualize {
    logger.println("");
    logger.println(format!("Visiting node {}", u));
    graph_tracer.visit(u as i64, None::<i64>, None::<i64>);
    Tracer::delay();
    graph_tracer.leave(u as i64, None::<i64>, None::<i64>);
    Tracer::delay();
    // }

    // visited [u] = true;
    disc[u] = *timer;
    low[u] = *timer;
    *timer += 1;

    // logger {
    let adjacent = adj[u]
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    logger.println(format!("Nodes adjacent to {} are: [ {} ]", u, adjacent));
    // }

    for index in 0..adj[u].len() {
        let v = adj[u][index];
        if disc[v] > -1 && v as i64 == parent {
            trace(graph_tracer, v, u);
            // logger {
            logger.println(format!("{}'s neighbor {} is u's parent. Not visiting it.", u, v));
            // }
        } else if disc[v] > -1 && v as i64 != parent {
            trace(graph_tracer, v, u);
            // logger {
            logger.println(format!(
                "{}'s neighbor {} is not u's parent. Comparing low[u] with disc[v]",
                u, v
            ));
            // }
            if low[u] > disc[v] {
                // logger {
                logger.println(format!(
                    "low[{}] is greater than disc[{}]. Setting low[{}] to disc[{}]",
                    u, v, u, v
                ));
                // }
                low[u] = disc[v];
            }
        }

        if disc[v] == -1 {
            trace(graph_tracer, v, u);
            // logger {
            logger.println(format!("{}'s neighbor {} has not been visited yet", u, v));

            logger.println(format!(
                "recursively calling util ({}, [{}], [{}],{})",
                v,
                join(disc),
                join(low),
                u
            ));
            // }
            util(graph_tracer, logger, adj, disc, low, bridges, timer, v, u as i64);

            // logger {
            logger.println("--------------------------------------------------------------------");

            logger.println(format!("Setting low [{}] to {}", u, low[u].min(low[v])));
            // }
            low[u] = low[u].min(low[v]);

            if low[v] == disc[v] {
                // logger {
                logger.println(format!(
                    "low [{}] === disc [{}], low[{}]={}, disc[{}]={}",
                    v, v, v, low[v], v, disc[v]
                ));
                logger.println(format!(
                    "{} -> {} is a bridge. Adding {}->{}to the set of bridges found",
                    u, v, u, v
                ));
                // }
                bridges.push((u as i64, v as i64));
            }
        }
    }
}

fn join(values: &[i64]) -> String {
    values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(", ")
}
