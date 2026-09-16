# import visualization libraries {
from algorithm_visualizer import Array1DTracer, Tracer, Layout, VerticalLayout
# }

# define tracer variables {
tracer = Array1DTracer('Sequence')
Layout.setRoot(VerticalLayout([tracer]))
index = 15
D = [1, 1]
for i in range(2, index):
    D.append(0)
tracer.set(D)
Tracer.delay()
# }

for i in range(2, index):
    D[i] = D[i - 2] + D[i - 1]
    # visualize {
    tracer.select(i - 2, i - 1)
    Tracer.delay()
    tracer.patch(i, D[i])
    Tracer.delay()
    tracer.depatch(i)
    tracer.deselect(i - 2, i - 1)
    # }
