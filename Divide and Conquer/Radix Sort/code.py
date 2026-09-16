# import visualization libraries {
from algorithm_visualizer import Array2DTracer, LogTracer, Randomize, Tracer, Layout, VerticalLayout
# }

# define tracer variables {
tracer = Array2DTracer()
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, logger]))
k = Randomize.Array1D(N=10, randomizer=Randomize.Integer(min=1, max=999)).create()
D = [
    k,
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
]
tracer.set(D)
Tracer.delay()
# }

# logger {
logger.println("original array = [{}]".format(", ".join(map(str, D[0]))))
# }


def pow10(expo):
    ans = 1
    for _ in range(expo):
        ans *= 10
    return ans


def digit(i, exp):
    return int(D[0][i] / pow10(exp) % 10)


for exp in range(3):
    # logger {
    logger.println("Digit: {}".format(exp))
    # }
    for i in range(len(D[0])):
        d = digit(i, exp)
        # visualize {
        tracer.select(0, i)
        Tracer.delay()
        # }
        D[2][d] += 1
        # visualize {
        tracer.patch(2, d, D[2][d])
        Tracer.delay()
        tracer.depatch(2, d)
        tracer.deselect(0, i)
        # }
    for i in range(1, 10):
        # visualize {
        tracer.select(2, i - 1)
        Tracer.delay()
        # }
        D[2][i] += D[2][i - 1]
        # visualize {
        tracer.patch(2, i, D[2][i])
        Tracer.delay()
        tracer.depatch(2, i)
        tracer.deselect(2, i - 1)
        # }
    for i in range(len(D[0]) - 1, -1, -1):
        d = digit(i, exp)
        # visualize {
        tracer.select(0, i)
        Tracer.delay()
        # }
        D[2][d] -= 1
        # visualize {
        tracer.patch(2, d, D[2][d])
        Tracer.delay()
        tracer.depatch(2, d)
        # }
        D[1][D[2][d]] = D[0][i]
        # visualize {
        tracer.patch(1, D[2][d], D[1][D[2][d]])
        Tracer.delay()
        tracer.depatch(1, D[2][d])
        tracer.deselect(0, i)
        # }
    for i in range(len(D[0])):
        # visualize {
        tracer.select(1, i)
        Tracer.delay()
        # }
        D[0][i] = D[1][i]
        # visualize {
        tracer.patch(0, i, D[0][i])
        Tracer.delay()
        tracer.depatch(0, i)
        tracer.deselect(1, i)
        # }
    for i in range(10):
        D[2][i] = 0
        # visualize {
        tracer.patch(2, i, D[2][i])
        Tracer.delay()
        tracer.depatch(2, i)
        # }
# logger {
logger.println("sorted array = [{}]".format(", ".join(map(str, D[0]))))
# }
