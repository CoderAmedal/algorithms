# import visualization libraries {
from algorithm_visualizer import Array1DTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

N = 10
A = [0] * (N + 1)

# define tracer variables {
tracer = Array1DTracer(' Catalan Numbers ')
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, logger]))
tracer.set(A)
Tracer.delay()
# }

A[0] = 1
# visualize {
tracer.patch(0, A[0])
Tracer.delay()
tracer.depatch(0)
# }
A[1] = 1
# visualize {
tracer.patch(1, A[1])
Tracer.delay()
tracer.depatch(1)
# }

for i in range(2, N + 1):
    for j in range(i):
        A[i] += A[j] * A[i - j - 1]
        # visualize {
        tracer.select(j)
        Tracer.delay()
        tracer.select(i - j - 1)
        Tracer.delay()
        tracer.patch(i, A[i])
        Tracer.delay()
        tracer.deselect(j)
        tracer.deselect(i - j - 1)
        tracer.depatch(i)
        # }

# visualize {
logger.println(" The {}th Catalan Number is {}".format(N, A[N]))
tracer.select(N)
Tracer.delay()
# }
