# import visualization libraries {
from algorithm_visualizer import Tracer, Array1DTracer, Layout, VerticalLayout
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


def fact(num):
    if num < 0:
        return

    if num == 0:
        return 1

    res = num * fact(num - 1)

    D[num - 1] = res

    # visualize {
    tracer.select(num - 1)
    Tracer.delay()
    tracer.patch(num - 1, D[num - 1])
    Tracer.delay()
    tracer.depatch(num - 1)
    tracer.deselect(num - 1)
    # }

    return res


fact(index)
