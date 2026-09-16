# import visualization libraries {
from algorithm_visualizer import Array1DTracer, Array2DTracer, LogTracer, Randomize, Tracer, Layout, VerticalLayout
# }

A = Randomize.Array1D(N=7).create()
N = len(A)

# define tracer variables {
tracer1 = Array1DTracer('Array')
tracer2 = Array2DTracer('Holes')
logTracer = LogTracer('Console')
Layout.setRoot(VerticalLayout([tracer1, tracer2, logTracer]))
tracer1.set(A)
Tracer.delay()
# }

minimum = A[0]
maximum = A[0]

for i in range(1, N):
    if A[i] < minimum:
        minimum = A[i]
    if A[i] > maximum:
        maximum = A[i]
range_ = maximum - minimum + 1

holes = [[] for _ in range(range_)]
# visualize {
tracer2.set(holes)
# }

# logger {
logTracer.println('Filling up holes')
# }
for i in range(N):
    # visualize {
    tracer1.select(i)
    Tracer.delay()
    # }

    holes[A[i] - minimum].append(A[i])

    # visualize {
    tracer2.set(holes)
    tracer1.deselect(i)
    # }

# logger {
logTracer.println('Building sorted array')
# }
k = 0
for i in range(range_):
    for j in range(len(holes[i])):
        # visualize {
        tracer2.select(i, j)
        Tracer.delay()
        # }
        A[k] = holes[i][j]
        k += 1
        # visualize {
        tracer1.patch(k - 1, A[k - 1])
        Tracer.delay()
        tracer2.deselect(i, j)
        tracer1.depatch(k - 1)
        # }

# logger {
logTracer.println("Sorted array is {}".format(A))
# }
