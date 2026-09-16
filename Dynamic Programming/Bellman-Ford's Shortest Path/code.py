# import visualization libraries {
from algorithm_visualizer import GraphTracer, LogTracer, Randomize, Tracer, Layout, VerticalLayout
# }

# define tracer variables {
tracer = GraphTracer()
tracer.weighted()
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, logger]))
tracer.log(logger)
G = Randomize.Graph(N=5, ratio=0.5, randomizer=Randomize.Integer(min=-2, max=5)).weighted(True).create()
tracer.set(G)
Tracer.delay()
# }

MAX_VALUE = 0x7fffffff


def bellman_ford(src, dest):
    weights = [MAX_VALUE] * len(G)
    for i in range(len(G)):
        weights[i] = MAX_VALUE
        # visualize {
        tracer.updateNode(i, weights[i])
        # }
    weights[src] = 0
    # visualize {
    tracer.updateNode(src, 0)
    # }

    # logger {
    logger.println("Initializing weights to: [{}]".format(", ".join(map(str, weights))))
    logger.println("")
    # }

    # begin BF algorithm execution
    k = len(G)
    while k > 0:
        k -= 1
        # logger {
        logger.println("Iteration: {}".format(len(G) - k))
        logger.println("------------------------------------------------------------------")
        # }

        for i in range(len(G)):
            for j in range(len(G)):
                if G[i][j]:  # proceed to relax Edges only if a particular weight != 0 (0 represents no edge)
                    if weights[j] > weights[i] + G[i][j]:
                        weights[j] = weights[i] + G[i][j]
                        # logger {
                        logger.println("weights[{}] = weights[{}] + {}".format(j, i, G[i][j]))
                        # }
                    # visualize {
                    tracer.visit(j, i, weights[j])
                    Tracer.delay()
                    tracer.leave(j, i)
                    Tracer.delay()
                    # }

        # logger {
        logger.println("updated weights: [{}]".format(", ".join(map(str, weights))))
        logger.println("")
        # }

    # check for cycle
    logger.println("checking for cycle")
    for i in range(len(G)):
        for j in range(len(G)):
            if G[i][j]:
                if weights[j] > weights[i] + G[i][j]:
                    # logger {
                    logger.println("A cycle was detected: weights[{}] > weights[{}] + {}".format(j, i, G[i][j]))
                    # }
                    return MAX_VALUE

    # logger {
    logger.println("No cycles detected. Final weights for the source {} are: [{}]".format(src, ", ".join(map(str, weights))))
    # }

    return weights[dest]


src = Randomize.Integer(min=0, max=len(G) - 1).create()
dest = src
while dest == src:
    dest = Randomize.Integer(min=0, max=len(G) - 1).create()

# logger {
logger.println("finding the shortest path from {} to {}".format(src, dest))
# }

minWeight = bellman_ford(src, dest)

# logger {
if minWeight == MAX_VALUE:
    logger.println("there is no path from {} to {}".format(src, dest))
else:
    logger.println("the shortest path from {} to {} is {}".format(src, dest, minWeight))
# }
