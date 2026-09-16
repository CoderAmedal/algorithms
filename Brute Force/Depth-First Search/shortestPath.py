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

D = []
minWeight = None
e = None


def dfs(node, parent, weight):  # node = current node, parent = previous node
    global minWeight
    if minWeight < weight:
        return
    if node == e:
        # visualize {
        tracer.visit(node, parent, weight)
        Tracer.delay()
        # }
        if minWeight > weight:
            minWeight = weight
        # visualize {
        tracer.leave(node, parent, minWeight)
        Tracer.delay()
        # }
        return
    D[node] = True  # label current node as discovered
    # visualize {
    tracer.visit(node, parent, weight)
    Tracer.delay()
    # }
    for i in range(len(G[node])):
        if G[node][i]:  # if the path from current node to the i-th node exists
            if not D[i]:  # if the i-th node is not labeled as discovered
                dfs(i, node, weight + G[node][i])  # recursively call DFS
    D[node] = False  # label current node as undiscovered
    # visualize {
    tracer.leave(node, parent, 0)
    Tracer.delay()
    # }


s = Randomize.Integer(min=0, max=len(G) - 1).create()  # s = start node
e = s  # e = end node
while e == s:
    e = Randomize.Integer(min=0, max=len(G) - 1).create()
MAX_VALUE = float('inf')
minWeight = MAX_VALUE
# logger {
logger.println("finding the shortest path from {} to {}".format(s, e))
# }
D = [False] * len(G)  # D[i] indicates whether the i-th node is discovered or not
dfs(s, None, 0)
# logger {
if minWeight == MAX_VALUE:
    logger.println("there is no path from {} to {}".format(s, e))
else:
    logger.println("the shortest path from {} to {} is {}".format(s, e, minWeight))
# }
