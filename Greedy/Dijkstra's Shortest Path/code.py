# import visualization libraries {
from algorithm_visualizer import Array1DTracer, GraphTracer, LogTracer, Randomize, Tracer, Layout, VerticalLayout
# }

G = Randomize.Graph(N=5, ratio=1, randomizer=Randomize.Integer()).directed(False).weighted(True).create()
MAX_VALUE = 10 ** 9
S = [MAX_VALUE] * len(G)  # S[end] returns the distance from start node to end node

# define tracer variables {
tracer = GraphTracer()
tracer.directed(False)
tracer.weighted()
tracerS = Array1DTracer()
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, tracerS, logger]))
tracer.log(logger)
tracer.set(G)
tracerS.set(S)
Tracer.delay()
# }


def dijkstra(start, end):
    D = [False] * len(G)  # D[i] indicates whether the i-th node is discovered or not
    S[start] = 0  # Starting node is at distance 0 from itself
    # visualize {
    tracerS.patch(start, S[start])
    Tracer.delay()
    tracerS.depatch(start)
    tracerS.select(start)
    # }
    k = len(G)
    while k:
        k -= 1
        # Finding a node with the shortest distance from S[minIndex]
        min_distance = MAX_VALUE
        min_index = -1
        for i in range(len(G)):
            if S[i] < min_distance and not D[i]:
                min_distance = S[i]
                min_index = i
        if min_distance == MAX_VALUE:
            break  # If there is no edge from current node, jump out of loop
        D[min_index] = True
        # visualize {
        tracerS.select(min_index)
        tracer.visit(min_index)
        Tracer.delay()
        # }
        # For every unvisited neighbour of current node, we check
        # whether the path to it is shorter if going over the current node
        for i in range(len(G)):
            if G[min_index][i] and S[i] > S[min_index] + G[min_index][i]:
                S[i] = S[min_index] + G[min_index][i]
                # visualize {
                tracerS.patch(i, S[i])
                tracer.visit(i, min_index, S[i])
                Tracer.delay()
                tracerS.depatch(i)
                tracer.leave(i, min_index)
                Tracer.delay()
                # }
        # visualize {
        tracer.leave(min_index)
        Tracer.delay()
        # }
    # logger {
    if S[end] == MAX_VALUE:
        logger.println("there is no path from {} to {}".format(start, end))
    else:
        logger.println("the shortest path from {} to {} is {}".format(start, end, S[end]))
    # }


s = Randomize.Integer(min=0, max=len(G) - 1).create()  # s = start node
e = s  # e = end node
while e == s:
    e = Randomize.Integer(min=0, max=len(G) - 1).create()
# logger {
logger.println("finding the shortest path from {} to {}".format(s, e))
Tracer.delay()
# }
dijkstra(s, e)
