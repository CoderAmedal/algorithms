# import visualization libraries {
from algorithm_visualizer import GraphTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

G = [
    [0, 1, 0, 0, 1, 0],
    [1, 0, 0, 0, 1, 0],
    [0, 0, 0, 1, 0, 0],
    [0, 0, 1, 0, 1, 1],
    [1, 1, 0, 1, 0, 0],
    [0, 0, 0, 1, 0, 0],
]

# define tracer variables {
graphTracer = GraphTracer()
graphTracer.directed(False)
logger = LogTracer()
Layout.setRoot(VerticalLayout([graphTracer, logger]))
graphTracer.set(G)
Tracer.delay()
# }

# NOTE: Code assumes NO parallel edges

timer = 0  # adj keeps track of the neighbors of each node

bridges = []
adj = []


def util(u, disc, low, parent):
    global timer
    # u is the node that is currently being processed in the DFS (depth-first search)
    # disc is the numbering of the vertices in the DFS, starting at 0
    # low[v] is the lowest numbered vertex that can be reached from vertex v along the DFS
    # parent is the node that u came from
    # visualize {
    logger.println('')
    logger.println("Visiting node {}".format(u))
    graphTracer.visit(u)
    Tracer.delay()
    graphTracer.leave(u)
    Tracer.delay()
    # }

    # visited [u] = true;
    disc[u] = low[u] = timer
    timer += 1

    # logger {
    logger.println("Nodes adjacent to {} are: [ {} ]".format(u, ", ".join(map(str, adj[u]))))
    # }
    # adj [u].forEach (function (v) {
    #   graphTracer.visit (v, u).delay ();
    #   graphTracer.leave (v, u).delay ();
    # });

    def trace(v):
        # visualize {
        graphTracer.visit(v, u)
        Tracer.delay()
        graphTracer.leave(v, u)
        Tracer.delay()
        # }

    for v in adj[u]:
        if disc[v] > -1 and v == parent:
            trace(v)
            # logger {
            logger.println("{}'s neighbor {} is u's parent. Not visiting it.".format(u, v))
            # }
        elif disc[v] > -1 and v != parent:
            trace(v)
            # logger {
            logger.println("{}'s neighbor {} is not u's parent. Comparing low[u] with disc[v]".format(u, v))
            # }
            if low[u] > disc[v]:
                # logger {
                logger.println("low[{}] is greater than disc[{}]. Setting low[{}] to disc[{}]".format(u, v, u, v))
                # }
                low[u] = disc[v]

        if disc[v] == -1:
            trace(v)
            # logger {
            logger.println("{}'s neighbor {} has not been visited yet".format(u, v))

            logger.println("recursively calling util ({}, [{}], [{}],{})".format(v, disc, low, u))
            # }
            util(v, disc, low, u)

            # logger {
            logger.println('--------------------------------------------------------------------')

            logger.println("Setting low [{}] to {}".format(u, min(low[u], low[v])))
            # }
            low[u] = min(low[u], low[v])

            if low[v] == disc[v]:
                # logger {
                logger.println("low [{}] === disc [{}], low[{}]={}, disc[{}]={}".format(v, v, v, low[v], v, disc[v]))
                logger.println("{} -> {} is a bridge. Adding {}->{}to the set of bridges found".format(u, v, u, v))
                # }
                bridges.append([u, v])


def find_bridges(graph):
    disc = [-1] * len(graph)
    low = [-1] * len(graph)

    # PRECOMPUTATION: store every node's neighbor info in auxiliary array for efficient retrieval later
    for config in graph:
        temp = []
        for i in range(len(config)):
            if config[i]:
                temp.append(i)
        adj.append(temp)

    # logger {
    logger.println("Initializing: <b>disc</b>: {} <b>low</b>: {}".format(disc, low))
    logger.println('')
    logger.println('Beginning efficient Bridge Finding')
    logger.println('NOTE: call to util () follows pattern: util (nodeToVisit, disc, low, parent). See code for clarity')
    logger.println('')

    logger.println('Starting the main for loop (for each node)')
    # }
    for v in range(len(graph)):
        if disc[v] == -1:
            # logger {
            logger.println("{} has not been visited yet. Calling util ({},  [{}], [{}],{}) from the for loop".format(v, v, disc, low, v))
            # }
            util(v, disc, low, v)
            # logger {
            logger.println("Returned in for loop after util ({}, [{}], [{}], [{}])".format(v, disc, low, v))
            # }


find_bridges(G)

# logger {
logger.println("There are {} bridges in the Graph".format(len(bridges)))
for i in range(len(bridges)):
    logger.println("{}-->{}".format(bridges[i][0], bridges[i][1]))
logger.println('NOTE: All bridges are both ways (just like in the Naive Algorithm) because the Graph is undirected. So, edge A->B and B->A, both are bridges')
# }
