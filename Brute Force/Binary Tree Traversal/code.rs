// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let g = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    ];

    let t: Vec<Vec<i64>> = vec![
        vec![-1, -1],
        vec![0, 2],
        vec![-1, -1],
        vec![1, 4],
        vec![-1, -1],
        vec![3, 8],
        vec![-1, 7],
        vec![-1, -1],
        vec![6, 10],
        vec![-1, -1],
        vec![9, -1],
    ];

    // define tracer variables {
    let tree_tracer = GraphTracer::new("Traversal Pre-order");
    let array_tracer = Array1DTracer::new("Print Pre-order");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tree_tracer, &array_tracer, &logger]));
    tree_tracer.set(&g);
    tree_tracer.layout_tree(Some(5i64), None::<bool>);
    let dashes: Vec<String> = (0..t.len()).map(|_| "-".to_string()).collect();
    array_tracer.set(&dashes);
    Tracer::delay();
    // }

    let index = std::cell::Cell::new(0i64);
    pre_order(&tree_tracer, &array_tracer, &logger, &t, 5, None, &index);
    // logger {
    logger.println("Finished");
    // }
}

fn pre_order(
    tree_tracer: &GraphTracer,
    array_tracer: &Array1DTracer,
    logger: &LogTracer,
    t: &[Vec<i64>],
    root: i64,
    parent: Option<i64>,
    index: &std::cell::Cell<i64>,
) {
    if root == -1 {
        // logger {
        logger.println("No more nodes. Backtracking.");
        Tracer::delay();
        // }
        return;
    }

    // visualize {
    logger.println(format!("Reached {}", root));
    tree_tracer.visit(root, parent, None::<i64>);
    Tracer::delay();

    logger.println(format!("Printing {}", root));
    tree_tracer.leave(root, None::<i64>, None::<i64>);
    array_tracer.patch(index.get(), Some(root));
    index.set(index.get() + 1);
    Tracer::delay();

    logger.println(format!(" Going left from {}", root));
    Tracer::delay();
    // }
    pre_order(tree_tracer, array_tracer, logger, t, t[root as usize][0], Some(root), index);

    // logger {
    logger.println(format!(" Going right from {}", root));
    Tracer::delay();
    // }
    pre_order(tree_tracer, array_tracer, logger, t, t[root as usize][1], Some(root), index);
}
