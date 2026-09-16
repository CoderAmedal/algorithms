# import visualization libraries {
from algorithm_visualizer import GraphTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

G = [
    [0, 1, 0, 0, 0, 0],
    [1, 0, 0, 1, 1, 0],
    [0, 0, 0, 1, 0, 0],
    [0, 1, 1, 0, 1, 1],
    [0, 1, 0, 1, 0, 0],
    [0, 0, 0, 1, 0, 0],
]

# define tracer variables {
tracer = GraphTracer()
tracer.directed(False)
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, logger]))
tracer.set(G)
Tracer.delay()
# }

# Depth First Search Exploration Algorithm to test connectedness of the Graph (see Graph Algorithms/DFS/exploration), without the tracer & logger commands


def dfs_explore(graph, source):
    stack = [[source, None]]
    visited = {}
    while len(stack) > 0:
        temp = stack.pop()
        node = temp[0]
        prev = temp[1]

        if node not in visited:
            visited[node] = True

            for i in range(len(graph)):
                if graph[node][i]:
                    stack.append([i, node])

    return visited


def find_bridges(graph):
    bridges = []
    for i in range(len(graph)):
        for j in range(len(graph)):
            if graph[i][j]:  # check if an edge exists
                # visualize {
                logger.println("Deleting edge {}->{} and calling DFSExplore ()".format(i, j))
                tracer.visit(j, i)
                Tracer.delay()
                tracer.leave(j, i)
                Tracer.delay()
                # }

                tempGraph = [row[:] for row in graph]
                tempGraph[i][j] = 0
                tempGraph[j][i] = 0
                visited = dfs_explore(tempGraph, 0)

                if len(visited) == len(graph):
                    # logger {
                    logger.println('Graph is CONNECTED. Edge is NOT a bridge')
                    # }
                else:
                    # logger {
                    logger.println('Graph is DISCONNECTED. Edge IS a bridge')
                    # }
                    bridges.append([i, j])

    return bridges


bridges = find_bridges(G)

# logger {
logger.println('The bridges are: ')
for i in range(len(bridges)):
    logger.println("{} to {}".format(bridges[i][0], bridges[i][1]))
logger.println('NOTE: A bridge is both ways, i.e., from A to B and from B to A, because this is an Undirected Graph')
# }
