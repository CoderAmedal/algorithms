# import visualization libraries {
from algorithm_visualizer import GraphTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

G = [  # G[i][j] indicates whether the path from the i-th node to the j-th node exists or not
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
]

T = [  # mapping to G as a binary tree , [i][0] indicates left child, [i][1] indicates right child
    [-1, -1],
    [-1, 7],
    [-1, -1],
    [6, 1],
    [-1, -1],
    [3, 8],
    [0, 2],
    [-1, -1],
    [10, 4],
    [-1, -1],
    [9, -1],
]

# define tracer variables {
treeTracer = GraphTracer(' Traversal Pre-order ')
logger = LogTracer(' Log ')
Layout.setRoot(VerticalLayout([treeTracer, logger]))
treeTracer.set(G)
treeTracer.layoutTree(5)
Tracer.delay()
# }


def lca_bt(parent, root, a, b):
    # logger {
    logger.println("Beginning new Iteration of lcaBT () with parent: {}, current root: {}".format(parent, root))
    # }
    if root == -1:
        # logger {
        logger.println('Reached end of path & target node(s) not found')
        # }
        return None

    # visualize {
    if parent is not None:
        treeTracer.visit(root, parent)
    else:
        treeTracer.visit(root)
    Tracer.delay()
    # }

    if root == a or root == b:
        return root

    left = lca_bt(root, T[root][0], a, b)
    right = lca_bt(root, T[root][1], a, b)

    if left is not None and right is not None:
        return root
    if left is None and right is None:
        # visualize {
        treeTracer.leave(root, parent)
        Tracer.delay()
        # }

    return left if left is not None else right


a = 7
b = 2
# logger {
logger.println("Lowest common ancestor of {} & {} is: {}".format(a, b, lca_bt(None, 5, a, b)))
# }
