# import visualization libraries {
from algorithm_visualizer import GraphTracer, LogTracer, Randomize, Tracer, Layout, VerticalLayout
# }

# define tracer variables {
tracer = GraphTracer()
tracer.directed(False)
tracer.weighted()
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, logger]))
tracer.log(logger)
G = Randomize.Graph(N=5, ratio=1).directed(False).weighted(True).create()
tracer.set(G)
Tracer.delay()
# }

D = None  # D[i] indicates whether the i-th node is discovered or not


def dfs(node, parent, weight):  # node = current node, parent = previous node
    # visualize {
    tracer.visit(node, parent, weight)
    Tracer.delay()
    # }
    D[node] = True  # label current node as discovered
    for i in range(len(G[node])):
        if G[node][i]:  # if the edge from current node to the i-th node exists
            if not D[i]:  # if the i-th node is not labeled as discovered
                dfs(i, node, weight + G[node][i])  # recursively call DFS
    D[node] = False  # label current node as undiscovered
    # visualize {
    tracer.leave(node, parent, 0)
    Tracer.delay()
    # }


for i in range(len(G)):  # start from every node
    # logger {
    logger.println("start from {}".format(i))
    # }
    D = [False] * len(G)
    dfs(i, None, 0)
