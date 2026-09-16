# import visualization libraries {
from algorithm_visualizer import Array1DTracer, GraphTracer, LogTracer, Randomize, Tracer, Layout, VerticalLayout
# }

# define tracer variables {
graphTracer = GraphTracer()
graphTracer.directed(False)
visitedTracer = Array1DTracer('visited')
logger = LogTracer()
Layout.setRoot(VerticalLayout([graphTracer, visitedTracer, logger]))
graphTracer.log(logger)
G = Randomize.Graph(N=8, ratio=0.3).directed(False).create()
graphTracer.set(G)
Tracer.delay()
# }


def dfs_explore(graph, source):
    stack = [[source, None]]
    visited = []
    for i in range(len(graph)):
        visited.append(False)
    # visualize {
    visitedTracer.set(visited)
    # }

    while len(stack) > 0:
        temp = stack.pop()
        node = temp[0]
        prev = temp[1]

        if not visited[node]:
            visited[node] = True
            # visualize {
            visitedTracer.patch(node, visited[node])

            if prev is not None and graph[node][prev]:
                graphTracer.visit(node, prev)
                Tracer.delay()
            else:
                graphTracer.visit(node)
                Tracer.delay()
            # }

            for i in range(len(graph)):
                if graph[node][i]:
                    stack.append([i, node])

    return visited


visited = dfs_explore(G, 0)
check = True
for i in range(len(visited)):
    check = check and visited[i]
# logger {
if check:
    logger.println('The Graph is CONNECTED')
else:
    logger.println('The Graph is NOT CONNECTED')
# }
