# import visualization libraries {
from algorithm_visualizer import Array1DTracer, Tracer, Layout, VerticalLayout
# }

# define tracer variables {
tracer = Array1DTracer('Sequence')
Layout.setRoot(VerticalLayout([tracer]))
index = 15
D = [1]
for i in range(1, index):
    D.append(0)
tracer.set(D)
Tracer.delay()
# }

for i in range(1, index):
    D[i] = D[i - 1] * i
    # visualize {
    tracer.select(i - 1)
    Tracer.delay()
    tracer.patch(i, D[i])
    Tracer.delay()
    tracer.depatch(i)
    tracer.deselect(i - 1)
    # }
