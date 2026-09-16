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
logger.println("Original array = [{}]".format(", ".join(map(str, D))))
# }
N = len(D)

gap = N // 2
while gap > 0:
    # logger {
    logger.println('')
    logger.println("Gap of {}".format(gap))
    # }
    for i in range(gap, N):
        # visualize {
        tracer.select(i)
        tracer.select(i - gap)
        Tracer.delay()
        # }
        k = D[i]
        # logger {
        logger.println("Holding: {}".format(k))
        # }
        j = i
        while j >= gap and k < D[j - gap]:
            # logger {
            logger.println("{} < {}".format(k, D[j - gap]))
            # }
            D[j] = D[j - gap]
            # visualize {
            tracer.patch(j, D[j])
            Tracer.delay()
            tracer.depatch(j)
            # }
            j -= gap
        old = D[j]
        D[j] = k
        # visualize {
        if old != k:
            tracer.patch(j, D[j])
            Tracer.delay()
            tracer.depatch(j)
            logger.println("Swapped {} with {}".format(D[j], old))

        tracer.deselect(i)
        tracer.deselect(i - gap)
        # }
    gap //= 2
# logger {
logger.println('')
logger.println("Sorted array = [{}]".format(", ".join(map(str, D))))
# }
