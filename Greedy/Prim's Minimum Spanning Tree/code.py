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
G = Randomize.Graph(N=10, ratio=.4).directed(False).weighted(True).create()
tracer.set(G)
Tracer.delay()
# }


def prim():
    # Finds a tree so that there exists a path between
    # every two nodes while keeping the cost minimal
    sum_ = 0
    D = [0] * len(G)
    D[0] = 1  # First node is visited
    for k in range(len(G) - 1):  # Searching for k edges
        minD = float('inf')
        minI = -1
        minJ = -1
        for i in range(len(G)):
            if D[i]:  # First node in an edge must be visited
                for j in range(len(G)):
                    if not D[j] and G[i][j]:
                        # visualize {
                        tracer.visit(i, j)
                        Tracer.delay()
                        # }
                        # Second node must not be visited and must be connected to first node
                        if G[i][j] < minD:
                            # Searching for cheapest edge which satisfies requirements
                            minD = G[i][j]
                            minI = i
                            minJ = j
                        # visualize {
                        tracer.leave(i, j)
                        Tracer.delay()
                        # }
        # visualize {
        tracer.visit(minI, minJ)
        Tracer.delay()
        # }
        D[minJ] = 1  # Visit second node and insert it into or tree
        sum_ += G[minI][minJ]
    # logger {
    logger.println("The sum of all edges is: {}".format(sum_))
    # }


# logger {
logger.println('nodes that belong to minimum spanning tree are: ')
# }
prim()
