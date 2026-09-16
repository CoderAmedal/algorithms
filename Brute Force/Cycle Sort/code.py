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
writes = 0  # number of writing performed
for cycleStart in range(0, N - 1):
    item = D[cycleStart]

    # find where to put the item
    pos = cycleStart
    # visualize {
    tracer.select(cycleStart)
    # }

    for i in range(cycleStart + 1, N):
        # visualize {
        tracer.select(i)
        Tracer.delay()
        tracer.deselect(i)
        # }
        if D[i] < item:
            pos += 1

    # if the item is already there, this is not a circle
    if pos == cycleStart:
        # visualize {
        tracer.deselect(cycleStart)
        # }
        continue

    # otherwise put the item there or right after any duplicates
    while item == D[pos]:
        pos += 1

    # write item to new index and increment writes
    D[pos], item = item, D[pos]
    writes += 1

    # logger {
    if pos != cycleStart:
        logger.println("Rewrite {} to index {}; the next value to rewrite is {}".format(D[pos], pos, item))
    else:
        logger.println("Rewrite {} to index {}".format(D[pos], pos))
    # }
    # visualize {
    tracer.select(pos)
    Tracer.delay()
    tracer.deselect(pos)
    tracer.patch(pos, D[pos])
    tracer.patch(cycleStart, D[cycleStart])
    Tracer.delay()
    tracer.depatch(pos)
    tracer.depatch(cycleStart)
    # }

    # rotate the rest of the cycle
    while pos != cycleStart:
        pos = cycleStart

        for i in range(cycleStart + 1, N):
            # visualize {
            tracer.select(i)
            Tracer.delay()
            tracer.deselect(i)
            # }
            if D[i] < item:
                pos += 1

        while item == D[pos]:
            pos += 1

        D[pos], item = item, D[pos]

        # logger {
        if pos != cycleStart:
            logger.println("Rewrite {} to index {}; the next value to rewrite is {}".format(D[pos], pos, item))
        else:
            logger.println("Rewrite {} to index {}".format(D[pos], pos))
        # }
        # visualize {
        tracer.select(pos)
        Tracer.delay()
        tracer.deselect(pos)
        tracer.patch(pos, D[pos])
        tracer.patch(cycleStart, D[cycleStart])
        Tracer.delay()
        tracer.depatch(pos)
        tracer.depatch(cycleStart)
        # }

        writes += 1

# logger {
logger.println("Number of writes performed is {}".format(writes))
# }
