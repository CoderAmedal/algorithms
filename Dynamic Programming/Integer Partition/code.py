# import visualization libraries {
from algorithm_visualizer import Array2DTracer, LogTracer, Randomize, Tracer, Layout, VerticalLayout
# }

# define tracer variables {
tracer = Array2DTracer()
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, logger]))
integer = Randomize.Integer(min=5, max=14).create()
D = [[0] * (integer + 1) for _ in range(integer + 1)]
A = ""
for i in range(integer + 1):
    D[i][0] = 1
tracer.set(D)
Tracer.delay()
# }


def partition(A, n, p):
    # logger {
    if p == 0:
        logger.println("[{}]".format(", ".join(A)))
    # }
    else:
        if n > 1:
            partition(A, n - 1, p)
        if n <= p:
            partition(str(n) + A, n, p - n)


def integer_partition(n):
    # cycle through each cell of matrix
    for i in range(1, n + 1):
        for j in range(1, n + 1):
            if i > j:
                # visualize {
                tracer.select(i, j)
                Tracer.delay()
                # }
                # set cell to cell above it
                D[i][j] = D[i - 1][j]
                # visualize {
                tracer.patch(i, j, D[i][j])
                Tracer.delay()
                tracer.depatch(i, j)
                tracer.deselect(i, j)
                # }
            else:
                # visualize {
                tracer.select(i, j)
                Tracer.delay()
                # }
                # grab above cell and add it to previous cell
                above = D[i - 1][j]
                left = D[i][j - i]
                D[i][j] = above + left
                # visualize {
                tracer.patch(i, j, D[i][j])
                Tracer.delay()
                tracer.depatch(i, j)
                tracer.deselect(i, j)
                # }
    return D[n][n]


# logger {
logger.println("Partitioning: {}".format(integer))
# }
partition(A, integer, integer)
part = integer_partition(integer)
# logger {
logger.println(part)
# }
