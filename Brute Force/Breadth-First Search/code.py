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


def bfs(s):  # s = start node
    Q = []
    Q.append(s)  # add start node to queue
    # visualize {
    tracer.visit(s)
    Tracer.delay()
    # }
    while len(Q) > 0:
        node = Q.pop(0)  # dequeue
        for i in range(len(G[node])):
            if G[node][i]:  # if current node has the i-th node as a child
                Q.append(i)  # add child node to queue
                # visualize {
                tracer.visit(i, node)
                Tracer.delay()
                # }


bfs(0)
