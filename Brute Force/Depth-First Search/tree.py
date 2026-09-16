# import visualization libraries {
from algorithm_visualizer import GraphTracer, LogTracer, Tracer, Layout, VerticalLayout
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
tracer.log(logger)
Layout.setRoot(VerticalLayout([tracer, logger]))
tracer.set(G)
tracer.layoutTree(0)
Tracer.delay()
# }


def dfs(node, parent):  # node = current node, parent = previous node
    # visualize {
    tracer.visit(node, parent)
    Tracer.delay()
    # }
    for i in range(len(G[node])):
        if G[node][i]:  # if current node has the i-th node as a child
            dfs(i, node)  # recursively call DFS


dfs(0, None)
