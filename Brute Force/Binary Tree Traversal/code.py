# import visualization libraries {
from algorithm_visualizer import Array1DTracer, GraphTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

G = [  # G[i][j] indicates whether the path from the i-th node to the j-th node exists or not
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
]

T = [  # mapping to G as a binary tree , [i][0] indicates left child, [i][1] indicates right child
    [-1, -1],
    [0, 2],
    [-1, -1],
    [1, 4],
    [-1, -1],
    [3, 8],
    [-1, 7],
    [-1, -1],
    [6, 10],
    [-1, -1],
    [9, -1],
]

# define tracer variables {
treeTracer = GraphTracer('Traversal Pre-order')
arrayTracer = Array1DTracer('Print Pre-order')
logger = LogTracer('Log')
Layout.setRoot(VerticalLayout([treeTracer, arrayTracer, logger]))
treeTracer.set(G)
treeTracer.layoutTree(5)
arrayTracer.set(['-'] * len(T))
Tracer.delay()
# }

index = 0


def pre_order(root, parent):
    global index
    if root == -1:
        # logger {
        logger.println('No more nodes. Backtracking.')
        Tracer.delay()
        # }
        return

    # visualize {
    logger.println("Reached {}".format(root))
    treeTracer.visit(root, parent)
    Tracer.delay()

    logger.println("Printing {}".format(root))
    treeTracer.leave(root)
    arrayTracer.patch(index, root)
    index += 1
    Tracer.delay()

    logger.println(" Going left from {}".format(root))
    Tracer.delay()
    # }
    pre_order(T[root][0], root)

    # logger {
    logger.println(" Going right from {}".format(root))
    Tracer.delay()
    # }
    pre_order(T[root][1], root)


pre_order(5, None)
# logger {
logger.println('Finished')
# }
