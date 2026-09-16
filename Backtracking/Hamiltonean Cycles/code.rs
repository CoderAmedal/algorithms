//A Hamiltonian cycle is a cycle in an undirected or directed graph that visits each vertex exactly once.
// import visualization libraries {
use algorithm_visualizer::*;
// }

fn ham(
    graph_tracer: &GraphTracer,
    log_tracer: &LogTracer,
    adjacency_matrix: &[Vec<i64>],
    x: &mut [i64],
    vis: &mut [i64],
    found: &mut bool,
    n: usize,
    k: usize,
) {
    loop {
        next_val(graph_tracer, adjacency_matrix, x, vis, n, k);
        if x[k] == -1 {
            return;
        }
        if k == n - 1 {
            graph_tracer.visit(x[0], Some(x[k]), None::<i64>);
            Tracer::delay();
            *found = true;
            // Printint the cycle{
            for i in 0..n {
                log_tracer.print(format!("{}  ", x[i]));
            }
            log_tracer.println(0);
            // }
            graph_tracer.leave(x[0], Some(x[k]), None::<i64>);
        } else {
            ham(graph_tracer, log_tracer, adjacency_matrix, x, vis, found, n, k + 1);
        }
    }
}

fn next_val(
    graph_tracer: &GraphTracer,
    adjacency_matrix: &[Vec<i64>],
    x: &mut [i64],
    vis: &mut [i64],
    n: usize,
    k: usize,
) {
    loop {
        let mut i = 0usize;
        if vis[k] == 1 {
            graph_tracer.leave(x[k], Some(x[k - 1]), None::<i64>);
        }
        vis[k] = 0;
        x[k] = (x[k] + 1) % (n as i64 + 1);
        if x[k] == n as i64 {
            x[k] = -1;
            return;
        }
        graph_tracer.visit(x[k], Some(x[k - 1]), None::<i64>);
        Tracer::delay();
        vis[k] = 1;
        if adjacency_matrix[x[k - 1] as usize][x[k] as usize] == 1 {
            while i < k {
                if x[i] == x[k] {
                    break;
                }
                i += 1;
            }
            if i == k {
                if k < n - 1 || (k == n - 1 && adjacency_matrix[x[k] as usize][x[0] as usize] == 1) {
                    return;
                }
            }
        }
    }
}

fn main() {
    // define tracer variables {
    let graph_tracer = GraphTracer::new("GraphTracer");
    let log_tracer = LogTracer::new("Console");
    // }

    let n = 8usize;

    // initializing{
    let mut adjacency_matrix = vec![vec![0i64; n]; n];
    let mut x = vec![0i64; n];
    let mut vis = vec![0i64; n];
    for i in 1..n {
        x[i] = -1;
    }
    // }

    // Randomizing adjacancy matrix and displaying on log screen{
    log_tracer.println("The adjacancy matrix is");
    for i in 0..n {
        for j in 0..n {
            adjacency_matrix[i][j] = Integer::new(0, 1).create_int();
            log_tracer.print(format!("{}  ", adjacency_matrix[i][j]));
        }
        log_tracer.println("");
    }
    // }

    // visualize {
    Layout::set_root(&VerticalLayout::new(layout![&graph_tracer, &log_tracer]));
    graph_tracer.set(&adjacency_matrix);
    // }

    let mut found = false;
    log_tracer.println("The possible solutions are");
    ham(
        &graph_tracer,
        &log_tracer,
        &adjacency_matrix,
        &mut x,
        &mut vis,
        &mut found,
        n,
        1,
    );
    if !found {
        log_tracer.println("No cycles are found Try with a different graph ");
    }
}
