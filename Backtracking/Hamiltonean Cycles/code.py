#A Hamiltonian cycle is a cycle in an undirected or directed graph that visits each vertex exactly once.
# import visualization libraries {
from algorithm_visualizer import GraphTracer, LogTracer, Randomize, Tracer, Layout, VerticalLayout
# }

# define tracer variables {
graph_tracer = GraphTracer("GraphTracer")
log_tracer = LogTracer("Console")
# }

n = 8
x = [0] * n
found = 0
vis = [0] * n
adjacency_matrix = None


def ham(k):
    global found
    while True:
        next_val(k)
        if x[k] == -1:
            return
        if k == n - 1:
            graph_tracer.visit(x[0], x[k])
            Tracer.delay()
            found = 1
            # Printint the cycle{
            for i in range(n):
                log_tracer.print("{}  ".format(x[i]))
            log_tracer.println(0)
            # }
            graph_tracer.leave(x[0], x[k])
        else:
            ham(k + 1)


def next_val(k):
    while True:
        i = 0
        if vis[k] == 1:
            graph_tracer.leave(x[k], x[k - 1])
        vis[k] = 0
        x[k] = (x[k] + 1) % (n + 1)
        if x[k] == n:
            x[k] = -1
            return
        graph_tracer.visit(x[k], x[k - 1])
        Tracer.delay()
        vis[k] = 1
        if adjacency_matrix[x[k - 1]][x[k]] == 1:
            while i < k:
                if x[i] == x[k]:
                    break
                i += 1
            if i == k:
                if k < n - 1 or (k == n - 1 and adjacency_matrix[x[k]][x[0]] == 1):
                    return


# initializing{
adjacency_matrix = [[0] * n for _ in range(n)]
x = [0] * n
vis = [0] * n
for i in range(1, n):
    x[i] = -1
# }

# Randomizing adjacancy matrix and displaying on log screen{
log_tracer.println("The adjacancy matrix is")
for i in range(n):
    for j in range(n):
        adjacency_matrix[i][j] = Randomize.Integer(min=0, max=1).create()
        log_tracer.print("{}  ".format(adjacency_matrix[i][j]))
    log_tracer.println("")
# }

# visualize {
Layout.setRoot(VerticalLayout([graph_tracer, log_tracer]))
graph_tracer.set(adjacency_matrix)
# }

log_tracer.println("The possible solutions are")
ham(1)
if found == 0:
    log_tracer.println("No cycles are found Try with a different graph ")
