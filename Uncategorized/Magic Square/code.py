# import visualization libraries {
from algorithm_visualizer import Tracer, Array2DTracer, LogTracer, Layout, VerticalLayout
# }

n = 7
A = [[0] * n for _ in range(n)]

# define tracer variables {
tracer = Array2DTracer('Magic Square')
logTracer = LogTracer('Console')
Layout.setRoot(VerticalLayout([tracer, logTracer]))
tracer.set(A)
Tracer.delay()
# }

i = n // 2
j = n - 1

num = 1
while num <= n * n:
    # logger {
    logTracer.println("i = {}".format(i))
    logTracer.println("j = {}".format(j))
    # }

    if i == -1 and j == n:
        j = n - 2
        i = 0

        # logger {
        logTracer.println('Changing : ')
        logTracer.println("i = {}".format(i))
        logTracer.println("j = {}".format(j))
        # }
    else:
        if j == n:
            j = 0
            # logger {
            logTracer.println("Changing : j = {}".format(j))
            # }
        if i < 0:
            i = n - 1
            # logger {
            logTracer.println("Changing : i = {}".format(i))
            # }

    if A[i][j] > 0:
        # logger {
        logTracer.println("Cell already filled : Changing i = {} j = {}".format(i, j))
        # }
        j -= 2
        i += 1
    else:
        A[i][j] = num
        num += 1
        # visualize {
        tracer.patch(i, j, A[i][j])
        Tracer.delay()
        tracer.depatch(i, j)
        tracer.select(i, j)
        Tracer.delay()
        # }
        j += 1
        i -= 1

# logger {
logTracer.println("Magic Constant is {}".format(n * (n * n + 1) // 2))
# }
