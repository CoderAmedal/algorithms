# import visualization libraries {
from algorithm_visualizer import Tracer, GraphTracer, LogTracer, Layout, VerticalLayout
# }

G = [  # G[i][j] indicates whether the path from the i-th node to the j-th node exists or not
    [0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
]

# define tracer variables {
tracer = GraphTracer()
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, logger]))
tracer.log(logger)
tracer.set(G)
tracer.layoutTree(0)
Tracer.delay()
# }


# This is a sample DLS applications where
# we try to find number of descendant of root within some depth
def DLSCount(limit, node, parent=None):  # node = current node, parent = previous node
    # visualize {
    if parent is None:
        tracer.visit(node)
    else:
        tracer.visit(node, parent)
    Tracer.delay()
    # }
    child = 0
    if limit > 0:  # cut off the search
        for i in range(len(G[node])):
            if G[node][i]:  # if current node has the i-th node as a child
                child += 1 + DLSCount(limit - 1, i, node)  # recursively call DLS
        return child
    return child


# logger {
logger.println("Number of descendant is {}".format(DLSCount(2, 0)))
# }
