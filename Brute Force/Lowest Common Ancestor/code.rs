// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let g = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
        vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    ];

    let t: Vec<Vec<i64>> = vec![
        vec![-1, -1],
        vec![-1, 7],
        vec![-1, -1],
        vec![6, 1],
        vec![-1, -1],
        vec![3, 8],
        vec![0, 2],
        vec![-1, -1],
        vec![10, 4],
        vec![-1, -1],
        vec![9, -1],
    ];

    // define tracer variables {
    let tree_tracer = GraphTracer::new(" Traversal Pre-order ");
    let logger = LogTracer::new(" Log ");
    Layout::set_root(&VerticalLayout::new(layout![&tree_tracer, &logger]));
    tree_tracer.set(&g);
    tree_tracer.layout_tree(Some(5i64), None::<bool>);
    Tracer::delay();
    // }

    let a = 7i64;
    let b = 2i64;
    // logger {
    let result = lca_bt(&tree_tracer, &logger, &t, None, 5, a, b);
    logger.println(format!(
        "Lowest common ancestor of {} & {} is: {}",
        a,
        b,
        result.map_or("null".to_string(), |value| value.to_string())
    ));
    // }
}

fn lca_bt(
    tree_tracer: &GraphTracer,
    logger: &LogTracer,
    t: &[Vec<i64>],
    parent: Option<i64>,
    root: i64,
    a: i64,
    b: i64,
) -> Option<i64> {
    // logger {
    logger.println(format!(
        "Beginning new Iteration of lcaBT () with parent: {}, current root: {}",
        parent.map_or("null".to_string(), |value| value.to_string()),
        root
    ));
    // }
    if root == -1 {
        // logger {
        logger.println("Reached end of path & target node(s) not found");
        // }
        return None;
    }

    // visualize {
    if let Some(p) = parent {
        tree_tracer.visit(root, Some(p), None::<i64>);
    } else {
        tree_tracer.visit(root, None::<i64>, None::<i64>);
    }
    Tracer::delay();
    // }

    if root == a || root == b {
        return Some(root);
    }

    let left = lca_bt(tree_tracer, logger, t, Some(root), t[root as usize][0], a, b);
    let right = lca_bt(tree_tracer, logger, t, Some(root), t[root as usize][1], a, b);

    if left.is_some() && right.is_some() {
        return Some(root);
    }
    if left.is_none() && right.is_none() {
        // visualize {
        tree_tracer.leave(root, parent, None::<i64>);
        Tracer::delay();
        // }
    }

    if left.is_some() {
        left
    } else {
        right
    }
}
