# import visualization libraries {
from algorithm_visualizer import Tracer, GraphTracer, LogTracer, Layout, VerticalLayout
# }

# G[i][j] indicates whether the path from the i-th node to the j-th node exists or not. NOTE: The graph must be Directed-Acyclic
G = [
    [0, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0],
    [0, 0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [1, 0, 0, 1, 0, 0],
    [1, 1, 0, 0, 0, 0],
]

# define tracer variables {
tracer = GraphTracer()
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, logger]))
tracer.log(logger)
tracer.set(G)
Tracer.delay()
# }

inDegrees = [0] * len(G)  # create an Array of G.length number of 0s
Q = []
iter = 0

# logger {
logger.println('Calculating in-degrees for each Node...')
# }

for currNode in range(len(G)):
    for currNodeNeighbor in range(len(G)):
        if G[currNode][currNodeNeighbor]:
            # visualize {
            logger.println("{} has an incoming edge from {}".format(currNodeNeighbor, currNode))
            tracer.visit(currNodeNeighbor, currNode)
            Tracer.delay()
            # }
            inDegrees[currNodeNeighbor] += 1
            # visualize {
            tracer.leave(currNodeNeighbor, currNode)
            Tracer.delay()
            # }
# logger {
logger.println("Done. In-Degrees are: [ {} ]".format(", ".join(map(str, inDegrees))))
logger.println('')

logger.println('Initializing queue with all the sources (nodes with no incoming edges)')
# }
for node in range(len(inDegrees)):
    # visualize {
    tracer.visit(node)
    Tracer.delay()
    # }
    if not inDegrees[node]:
        # logger {
        logger.println("{} is a source".format(node))
        # }
        Q.append(node)
    # visualize {
    tracer.leave(node)
    Tracer.delay()
    # }
# logger {
logger.println("Done. Initial State of Queue: [ {} ]".format(", ".join(map(str, Q))))
logger.println('')
# }

# begin topological sort (kahn)
while Q:
    # logger {
    logger.println("Iteration #{}. Queue state: [ {} ]".format(iter, ", ".join(map(str, Q))))
    # }
    currNode = Q.pop(0)
    # visualize {
    tracer.visit(currNode)
    Tracer.delay()
    # }

    for i in range(len(G)):
        if G[currNode][i]:
            # visualize {
            logger.println("{} has an incoming edge from {}. Decrementing {}'s in-degree by 1.".format(i, currNode, i))
            tracer.visit(i, currNode)
            Tracer.delay()
            # }
            inDegrees[i] -= 1
            # visualize {
            tracer.leave(i, currNode)
            Tracer.delay()
            # }

            if not inDegrees[i]:
                # logger {
                logger.println("{}'s in-degree is now 0. Enqueuing {}".format(i, i))
                # }
                Q.append(i)
    # visualize {
    tracer.leave(currNode)
    Tracer.delay()
    # }
    # logger {
    logger.println("In-degrees are: [{} ]".format(", ".join(map(str, inDegrees))))
    logger.println('-------------------------------------------------------------------')
    # }

    iter += 1
