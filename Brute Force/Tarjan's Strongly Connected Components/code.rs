// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let g = vec![
        vec![0, 0, 1, 1, 0, 0],
        vec![1, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 1, 0],
    ];

    let mut disc = vec![-1i64; g.len()];
    let mut low = vec![-1i64; g.len()];
    let mut stack_member = vec![false; g.len()];
    let mut st: Vec<i64> = Vec::new();
    let mut time = 0i64;

    // define tracer variables {
    let graph_tracer = GraphTracer::new("Graph");
    graph_tracer.set(&g);
    let disc_tracer = Array1DTracer::new("Disc");
    let low_tracer = Array1DTracer::new("Low");
    let stack_member_tracer = Array1DTracer::new("stackMember");
    let st_tracer = Array1DTracer::new("st");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![
        &graph_tracer,
        &disc_tracer,
        &low_tracer,
        &stack_member_tracer,
        &st_tracer,
        &logger
    ]));
    disc_tracer.set(&disc);
    low_tracer.set(&low);
    stack_member_tracer.set(&stack_member);
    st_tracer.set(&st);
    Tracer::delay();
    // }

    for i in 0..g.len() {
        if disc[i] == -1 {
            scc_vertex(
                &graph_tracer,
                &disc_tracer,
                &low_tracer,
                &stack_member_tracer,
                &st_tracer,
                &logger,
                &g,
                &mut disc,
                &mut low,
                &mut st,
                &mut stack_member,
                &mut time,
                i,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn scc_vertex(
    graph_tracer: &GraphTracer,
    disc_tracer: &Array1DTracer,
    low_tracer: &Array1DTracer,
    stack_member_tracer: &Array1DTracer,
    st_tracer: &Array1DTracer,
    logger: &LogTracer,
    g: &[Vec<i64>],
    disc: &mut Vec<i64>,
    low: &mut Vec<i64>,
    st: &mut Vec<i64>,
    stack_member: &mut Vec<bool>,
    time: &mut i64,
    u: usize,
) {
    // visualize {
    graph_tracer.visit(u as i64, None::<i64>, None::<i64>);
    Tracer::delay();
    // }

    *time += 1;
    disc[u] = *time;
    // visualize {
    disc_tracer.patch(u as i64, Some(*time));
    Tracer::delay();
    // }

    low[u] = *time;
    // visualize {
    low_tracer.patch(u as i64, Some(*time));
    Tracer::delay();
    // }

    st.push(u as i64);
    // visualize {
    st_tracer.set(&*st);
    Tracer::delay();
    // }

    stack_member[u] = true;
    // visualize {
    stack_member_tracer.patch(u as i64, Some(true));
    Tracer::delay();
    // }

    // Go through all vertices adjacent to this
    for v in 0..g[u].len() {
        if g[u][v] != 0 {
            // If v is not visited yet, then recur for it
            if disc[v] == -1 {
                scc_vertex(
                    graph_tracer,
                    disc_tracer,
                    low_tracer,
                    stack_member_tracer,
                    st_tracer,
                    logger,
                    g,
                    disc,
                    low,
                    st,
                    stack_member,
                    time,
                    v,
                );

                // Check if the subtree rooted with 'v' has a
                // connection to one of the ancestors of 'u'
                low[u] = low[u].min(low[v]);
                // visualize {
                low_tracer.patch(u as i64, Some(low[u]));
                Tracer::delay();
                // }
            }
            // Update low value of 'u' only of 'v' is still in stack
            // (i.e. it's a back edge, not cross edge).
            else if stack_member[v] {
                low[u] = low[u].min(disc[v]);
                // visualize {
                low_tracer.patch(u as i64, Some(low[u]));
                Tracer::delay();
                // }
            }
        }
    }

    // head node found, pop the stack and print an SCC
    let mut w = 0i64; // To store stack extracted vertices
    if low[u] == disc[u] {
        while *st.last().unwrap() != u as i64 {
            w = st.pop().unwrap();
            // visualize {
            st_tracer.set(&*st);
            Tracer::delay();

            logger.println(w);
            Tracer::delay();
            // }

            stack_member[w as usize] = false;
            // visualize {
            stack_member_tracer.patch(w, Some(false));
            Tracer::delay();
            // }
        }

        w = st.pop().unwrap();
        // visualize {
        st_tracer.set(&*st);
        Tracer::delay();

        logger.println(w);
        Tracer::delay();
        logger.println("------");
        // }

        stack_member[w as usize] = false;
        // visualize {
        stack_member_tracer.patch(w, Some(false));
        Tracer::delay();
        // }
    }
}
