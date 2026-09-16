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
for i in range(1, len(D)):
    key = D[i]
    # visualize {
    logger.println("insert {}".format(key))
    tracer.select(i)
    Tracer.delay()
    # }
    j = i - 1
    while j >= 0 and D[j] > key:
        D[j + 1] = D[j]
        # visualize {
        tracer.patch(j + 1, D[j + 1])
        Tracer.delay()
        tracer.depatch(j + 1)
        # }
        j -= 1
    D[j + 1] = key
    # visualize {
    tracer.patch(j + 1, D[j + 1])
    Tracer.delay()
    tracer.depatch(j + 1)
    tracer.deselect(i)
    # }
# logger {
logger.println("sorted array = [{}]".format(", ".join(map(str, D))))
# }
