# import visualization libraries {
from algorithm_visualizer import Array1DTracer, Array2DTracer, GraphTracer, LogTracer, Randomize, Tracer, Layout, VerticalLayout
# }


def filled_array(length, value):
    return [value] * length


# define tracer variables {
G = Randomize.Graph(N=5, ratio=0.4).directed(True).weighted(False).create()
ranks = None
outgoingEdgeCounts = filled_array(len(G), 0)
incomingNodes = None
graphTracer = GraphTracer('Web Page inter-connections')
rankTracer = Array1DTracer('Web Page Ranks')
oecTracer = Array1DTracer('Outgoing Edge Counts')
inTracer = Array2DTracer('Incoming Nodes')

logger = LogTracer()
Layout.setRoot(VerticalLayout([graphTracer, rankTracer, oecTracer, inTracer, logger]))

graphTracer.set(G)
oecTracer.set(outgoingEdgeCounts)

incomingNodes = []
while len(incomingNodes) < len(G):
    incomingNodes.append(filled_array(len(G), -1))
inTracer.set(incomingNodes)
Tracer.delay()
# }

# PageRank Algorithm Version 2
# Equation:
#   PR (X) = ( (1 - D)/N ) + D (Summation i->X (PR (I) / Out (i)))
# NOTE: Algorithm uses the recommended damping factor (D). Number of iterations is small because only a small Web of 5 Pages is simulated


def array_sum(array):
    return sum(1 for curr in array if curr)  # if curr is 0 (no edge) or undefined (loop not allowed), sum remains unchanged


def show_outgoing_edges(i):
    for j in range(len(G[i])):
        if G[i][j]:
            # visualize {
            graphTracer.visit(j, i)
            Tracer.delay()
            graphTracer.leave(j, i)
            Tracer.delay()
            # }


# PRECOMPUTATIONS

# logger {
logger.println('Calculate Outgoing Edge Count for each Node')
# }


def calculate_oec():
    for i in range(len(G)):
        outgoingEdgeCounts[i] = array_sum(G[i])
        show_outgoing_edges(i)

        # visualize {
        oecTracer.patch(i, outgoingEdgeCounts[i])
        Tracer.delay()
        oecTracer.depatch(i)
        Tracer.delay()
        # }


calculate_oec()

# logger {
logger.println('determine incoming nodes for each node')
# }


def determine_in():
    for i in range(len(G)):
        for j in range(len(G)):
            if G[i][j]:
                # there's an edge FROM i TO j
                # visualize {
                graphTracer.visit(j, i)
                Tracer.delay()
                # }

                nextPos = incomingNodes[j].index(-1)
                incomingNodes[j][nextPos] = i
                # visualize {
                inTracer.patch(j, nextPos, i)
                Tracer.delay()
                inTracer.depatch(j, nextPos)
                Tracer.delay()

                graphTracer.leave(j, i)
                Tracer.delay()
                # }

    # logger.println ('All -1s will be removed from incoming node records, they are irrelevant');
    for arr in incomingNodes:
        idx = arr.index(-1) if -1 in arr else -1
        del arr[idx:]


determine_in()


def update_rank(node_index):
    inNodeSummation = 0
    result = 0

    # logger {
    logger.println("Updating rank of {}".format(node_index))
    logger.println("The incoming Nodes of {} are being highlighted".format(node_index))
    # }

    for i in range(len(incomingNodes[node_index])):
        incoming = incomingNodes[node_index][i]
        # visualize {
        inTracer.select(node_index, i)
        Tracer.delay()
        logger.println("Outgoing edge count of {} is {}".format(incoming, outgoingEdgeCounts[incoming]))
        oecTracer.select(incoming)
        Tracer.delay()
        # }

        inNodeSummation += (ranks[incoming] / outgoingEdgeCounts[incoming])

        # visualize {
        oecTracer.deselect(incoming)
        Tracer.delay()
        inTracer.deselect(node_index, i)
        Tracer.delay()
        # }
    # logger {
    logger.println("In-Node summation of {} = {}".format(node_index, inNodeSummation))
    # }

    result = ((1 - damping) / len(G)) + (damping * inNodeSummation)  # notice the subtle difference between equations of Basic PR & PR version 2 (divide by N)
    # logger {
    logger.println("Therefore, using Equation, new rank of {} = {}".format(node_index, result))
    # }
    return result


damping = 0.85
iterations = 7
initialRank = 1.0

# logger {
logger.println("Initialized all Page ranks to {}".format(initialRank))
# }
ranks = filled_array(len(G), initialRank)

# visualize {
rankTracer.set(ranks)
# }
# logger {
logger.println('Begin execution of PageRank Version #1')
logger.println('Equation used: PR (X) = (1 - D) + D (In-Node-Summation i->X (PR (I) / Out (i)))')
logger.println('D = Damping Factor, PR (X) = Page rank of Node X, i = the ith In-Node of X, Out (i) = outgoing Edge Count of i')
logger.println('')
# }

while iterations > 0:
    iterations -= 1
    for node in range(len(ranks)):
        ranks[node] = update_rank(node)
        # visualize {
        rankTracer.patch(node, ranks[node])
        Tracer.delay()
        rankTracer.patch(node)
        Tracer.delay()
        # }

# logger {
logger.println('Page Ranks have been converged to.')
for node in range(len(ranks)):
    logger.println("Rank of Node #{} = {}".format(node, ranks[node]))
logger.println('Done')
# }
