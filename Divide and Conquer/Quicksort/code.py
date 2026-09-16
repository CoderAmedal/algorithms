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


def partition(D, low, high):
    while high > low:
        i = low
        j = high
        s = D[low]
        while i < j:
            # visualize {
            tracer.select(high)
            tracer.select(low)
            Tracer.delay()
            # }
            while D[j] > s:
                # visualize {
                tracer.select(j)
                Tracer.delay()
                tracer.deselect(j)
                # }
                j -= 1
            D[i] = D[j]
            # visualize {
            tracer.patch(i, D[j])
            Tracer.delay()
            tracer.depatch(i)
            # }
            while s >= D[i] and i < j:
                # visualize {
                tracer.select(i)
                Tracer.delay()
                tracer.deselect(i)
                # }
                i += 1
            D[j] = D[i]
            # visualize {
            tracer.patch(j, D[i])
            Tracer.delay()
            tracer.depatch(j)
            tracer.deselect(high)
            tracer.deselect(low)
            # }
        D[i] = s
        # visualize {
        tracer.patch(i, s)
        Tracer.delay()
        tracer.depatch(i)
        # }
        partition(D, low, i - 1)
        low = i + 1


def quicksort(D):
    partition(D, 0, len(D) - 1)


quicksort(D)
# logger {
logger.println("sorted array = [{}]".format(", ".join(map(str, D))))
# }
