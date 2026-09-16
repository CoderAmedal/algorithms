// import visualization libraries {
use algorithm_visualizer::*;
use std::collections::HashMap;
// }

#[derive(Default)]
struct Node {
    left: Option<i64>,
    right: Option<i64>,
}

fn main() {
    // define tracer variables {
    let graph_tracer = GraphTracer::new(" BST - Elements marked red indicates the current status of tree ");
    let elem_tracer = Array1DTracer::new(" Elements ");
    let logger = LogTracer::new(" Log ");
    Layout::set_root(&VerticalLayout::new(layout![&graph_tracer, &elem_tracer, &logger]));
    let elements = vec![5i64, 8, 10, 3, 1, 6, 9, 7, 2, 0, 4]; // item to be inserted
    elem_tracer.set(&elements);
    graph_tracer.log(&logger);
    Tracer::delay();
    // }

    let mut tree: HashMap<i64, Node> = HashMap::new();

    let root = elements[0]; // take first element as root
    tree.insert(root, Node::default());
    // visualize {
    graph_tracer.add_node(root, None::<i64>, None, None);
    graph_tracer.layout_tree(Some(root), Some(true));
    logger.println(format!("{} Inserted as root of tree ", root));
    // }

    for i in 1..elements.len() {
        // visualize {
        elem_tracer.select(i as i64, None);
        Tracer::delay();
        // }
        bst_insert(&graph_tracer, &logger, &mut tree, root, elements[i], None); // insert ith element
        // visualize {
        elem_tracer.deselect(i as i64, None);
        Tracer::delay();
        // }
    }

    let key = elements[Integer::new(0, elements.len() as i64 - 1).create_int() as usize]; // item to be searched

    // logger {
    logger.println(format!("Finding number {}", key));
    // }
    bst(&graph_tracer, &logger, &tree, key, root, None); // node with key root is the root
}

fn bst_insert(
    graph_tracer: &GraphTracer,
    logger: &LogTracer,
    tree: &mut HashMap<i64, Node>,
    root: i64,
    element: i64,
    parent: Option<i64>,
) {
    // root = current node , parent = previous node
    // visualize {
    graph_tracer.visit(root, parent, None::<i64>);
    Tracer::delay();
    // }

    let mut prop_name = 0;
    if element < root {
        prop_name = 1;
    } else if element > root {
        prop_name = 2;
    }
    if prop_name != 0 {
        let child = if prop_name == 1 {
            tree[&root].left
        } else {
            tree[&root].right
        };
        match child {
            None => {
                // insert as child of root
                if prop_name == 1 {
                    tree.get_mut(&root).unwrap().left = Some(element);
                } else {
                    tree.get_mut(&root).unwrap().right = Some(element);
                }
                tree.insert(element, Node::default());
                // visualize {
                graph_tracer.add_node(element, None::<i64>, None, None);
                graph_tracer.add_edge(root, element, None::<i64>);
                graph_tracer.select(element, Some(root));
                Tracer::delay();
                graph_tracer.deselect(element, Some(root));
                logger.println(format!("{} Inserted", element));
                // }
            }
            Some(existing) => {
                bst_insert(graph_tracer, logger, tree, existing, element, Some(root));
            }
        }
    }
    // visualize {
    graph_tracer.leave(root, parent, None::<i64>);
    Tracer::delay();
    // }
}

fn bst(
    graph_tracer: &GraphTracer,
    logger: &LogTracer,
    tree: &HashMap<i64, Node>,
    item: i64,
    node: i64,
    parent: Option<i64>,
) {
    // node = current node , parent = previous node
    // visualize {
    graph_tracer.visit(node, parent, None::<i64>);
    Tracer::delay();
    // }
    if item == node {
        // key found
        // logger {
        logger.println(" Match Found ");
        // }
    } else if item < node {
        // key less than value of current node
        match tree[&node].left {
            None => {
                // logger {
                logger.println(" Not Found ");
                // }
            }
            Some(child) => bst(graph_tracer, logger, tree, item, child, Some(node)),
        }
    } else {
        // key greater than value of current node
        match tree[&node].right {
            None => {
                // logger {
                logger.println(" Not Found ");
                // }
            }
            Some(child) => bst(graph_tracer, logger, tree, item, child, Some(node)),
        }
    }
}
