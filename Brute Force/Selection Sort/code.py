# import visualization libraries {
from algorithm_visualizer import Array1DTracer, ChartTracer, LogTracer, Randomize, Tracer, Layout, VerticalLayout
# }

# define tracer variables {
chart = ChartTracer()
tracer = Array1DTracer()
logger = LogTracer()
Layout.setRoot(VerticalLayout([chart, tracer, logger]))
D = Randomize.Array1D(N=15).create()
tracer.set(D)
tracer.chart(chart)
Tracer.delay()
# }

# logger {
logger.println("original array = [{}]".format(", ".join(map(str, D))))
# }
for i in range(len(D) - 1):
    minJ = i
    # visualize {
    tracer.select(i)
    Tracer.delay()
    # }
    for j in range(i + 1, len(D)):
        # visualize {
        tracer.select(j)
        Tracer.delay()
        # }
        if D[j] < D[minJ]:
            minJ = j
            # visualize {
            tracer.patch(j)
            Tracer.delay()
            tracer.depatch(j)
            # }
        # visualize {
        tracer.deselect(j)
        # }
    if minJ != i:
        # logger {
        logger.println("swap {} and {}".format(D[i], D[minJ]))
        # }
        D[i], D[minJ] = D[minJ], D[i]
        # visualize {
        tracer.patch(i, D[i])
        tracer.patch(minJ, D[minJ])
        Tracer.delay()
        tracer.depatch(i)
        tracer.depatch(minJ)
        # }
    # visualize {
    tracer.deselect(i)
    # }
# logger {
logger.println("sorted array = [{}]".format(", ".join(map(str, D))))
# }
