# import visualization libraries {
from algorithm_visualizer import Array1DTracer, GraphTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

G = [
    [0, 1, 0, 1, 1],
    [1, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [1, 0, 1, 0, 0],  # <-- replace latest 0 with 1 to make G not biparted
    [1, 0, 0, 0, 0],
]

# define tracer variables {
tracer = GraphTracer()
tracer.directed(False)
logger = LogTracer()
tracer.log(logger)
tracer.set(G)
colorsTracer = Array1DTracer('Colors')
Layout.setRoot(VerticalLayout([tracer, logger, colorsTracer]))
Tracer.delay()
# }


def bfs_check_bipartiteness(s):
    Q = []

    # Create a new matrix to set colors (0,1)
    Colors = [-1] * len(G)
    # visualize {
    colorsTracer.set(Colors)
    # }

    Colors[s] = 1
    # visualize {
    colorsTracer.patch(s, 1)
    # }

    Q.append(s)  # add start node to queue

    while len(Q) > 0:
        node = Q.pop(0)  # dequeue
        # visualize {
        tracer.visit(node)
        Tracer.delay()
        # }

        for i in range(len(G[node])):
            if G[node][i]:
                if Colors[i] == -1:
                    Colors[i] = 1 - Colors[node]
                    # visualize {
                    colorsTracer.patch(i, 1 - Colors[node])
                    # }

                    Q.append(i)
                    # visualize {
                    tracer.visit(i, node)
                    Tracer.delay()
                    # }
                elif Colors[i] == Colors[node]:
                    # logger {
                    logger.println('Graph is not biparted')
                    # }
                    return False

    # logger {
    logger.println('Graph is biparted')
    # }
    return True


bfs_check_bipartiteness(0)
