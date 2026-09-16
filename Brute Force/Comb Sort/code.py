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
N = len(D)
gap = N  # initialize gap size
shrink = 1.3  # set the gap shrink factor
swapped = True

while True:
    # update the gap value for the next comb.
    gap = int(gap / shrink)
    if gap < 1:
        # minimum gap is 1
        gap = 1

    swapped = False  # initialize swapped
    # a single comb over the input list
    i = 0
    while i + gap < N:
        # visualize {
        tracer.select(i)
        tracer.select(i + gap)
        Tracer.delay()
        # }

        if D[i] > D[i + gap]:
            # logger {
            logger.println("swap {} and {}".format(D[i], D[i + gap]))  # log swap event
            # }

            D[i], D[i + gap] = D[i + gap], D[i]

            # visualize {
            tracer.patch(i, D[i])
            tracer.patch(i + gap, D[i + gap])
            Tracer.delay()
            tracer.depatch(i)
            tracer.depatch(i + gap)
            # }

            swapped = True  # Flag swapped has happened and list is not guaranteed sorted
        # visualize {
        tracer.deselect(i)
        tracer.deselect(i + gap)
        # }
        i += 1  # End of combing

    if gap == 1 and not swapped:
        break
