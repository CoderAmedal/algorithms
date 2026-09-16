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


def bfs(s):
    W = []  # W[i] indicates the length of the shortest path from start node to the i-th node
    Q = []
    for i in range(len(G)):
        W.append(MAX_VALUE)
        # visualize {
        tracer.updateNode(i, MAX_VALUE)
        # }
    W[s] = 0
    Q.append(s)  # add start node to queue
    # visualize {
    tracer.visit(s, None, 0)
    Tracer.delay()
    # }
    while len(Q) > 0:
        node = Q.pop(0)  # dequeue
        for i in range(len(G[node])):
            if G[node][i]:  # if the edge from current node to the i-th node exists
                if W[i] > W[node] + G[node][i]:  # if current path is shorter than the previously shortest path
                    W[i] = W[node] + G[node][i]  # update the length of the shortest path
                    Q.append(i)  # add child node to queue
                    # visualize {
                    tracer.visit(i, node, W[i])
                    Tracer.delay()
                    # }
    return W[e]


s = Randomize.Integer(min=0, max=len(G) - 1).create()  # s = start node
e = s  # e = start node
while e == s:
    e = Randomize.Integer(min=0, max=len(G) - 1).create()
MAX_VALUE = 0x7fffffff
# logger {
logger.println("finding the shortest path from {} to {}".format(s, e))
# }
minWeight = bfs(s)
# logger {
if minWeight == MAX_VALUE:
    logger.println("there is no path from {} to {}".format(s, e))
else:
    logger.println("the shortest path from {} to {} is {}".format(s, e, minWeight))
# }
