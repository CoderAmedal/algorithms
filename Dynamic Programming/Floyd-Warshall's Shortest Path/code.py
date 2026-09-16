# import visualization libraries {
from algorithm_visualizer import GraphTracer, LogTracer, Randomize, Tracer, Layout, VerticalLayout
# }

# define tracer variables {
tracer = GraphTracer()
tracer.weighted()
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, logger]))
tracer.log(logger)
G = Randomize.Graph(N=5, ratio=1).weighted(True).create()
tracer.set(G)
Tracer.delay()
# }

MAX_VALUE = float('inf')


def floyd_warshall():
    # Finds the shortest path between all nodes
    S = [[0] * len(G) for _ in range(len(G))]
    for i in range(len(G)):
        for j in range(len(G)):
            # Distance to self is always 0
            if i == j:
                S[i][i] = 0
            # Distance between connected nodes is their weight
            elif G[i][j] > 0:
                S[i][j] = G[i][j]
            # Else we don't know the distance and we set it to infinity
            else:
                S[i][j] = MAX_VALUE
    # If there is a shorter path using k, use it instead
    for k in range(len(G)):
        for i in range(len(G)):
            if k == i:
                continue
            # visualize {
            tracer.visit(k, i)
            Tracer.delay()
            # }
            for j in range(len(G)):
                if i == j or j == k:
                    continue
                # visualize {
                tracer.visit(j, k)
                Tracer.delay()
                # }
                if S[i][j] > S[i][k] + S[k][j]:
                    # visualize {
                    tracer.visit(j, i, S[i][j])
                    Tracer.delay()
                    # }
                    S[i][j] = S[i][k] + S[k][j]
                    # visualize {
                    tracer.leave(j, i, S[i][j])
                    # }
                # visualize {
                tracer.leave(j, k)
                # }
            # visualize {
            tracer.leave(k, i)
            Tracer.delay()
            # }
    # logger {
    for i in range(len(G)):
        for j in range(len(G)):
            if S[i][j] == MAX_VALUE:
                logger.println("there is no path from {} to {}".format(i, j))
            else:
                logger.println("the shortest path from {} to {} is {}".format(i, j, S[i][j]))
    # }


# logger {
logger.println("finding the shortest paths from and to all nodes")
# }
floyd_warshall()
