# import visualization libraries {
from algorithm_visualizer import Array1DTracer, ChartTracer, LogTracer, Randomize, Tracer, Layout, VerticalLayout
# }

# define tracer variables {
chart = ChartTracer()
tracer = Array1DTracer()
logger = LogTracer()
Layout.setRoot(VerticalLayout([chart, tracer, logger]))
D = Randomize.Array1D(N=10).create()
tracer.set(D)
tracer.chart(chart)
Tracer.delay()
# }

# logger {
logger.println("original array = [{}]".format(", ".join(map(str, D))))
# }
N = len(D)


def flip(start):
    # visualize {
    tracer.select(start, N - 1)
    Tracer.delay()
    # }
    idx = 0
    for i in range(start, (start + N) // 2):
        # visualize {
        tracer.select(i)
        Tracer.delay()
        # }
        D[i], D[N - idx - 1] = D[N - idx - 1], D[i]
        # visualize {
        tracer.patch(i, D[i])
        tracer.patch(N - idx - 1, D[N - idx - 1])
        Tracer.delay()
        tracer.depatch(i)
        tracer.depatch(N - idx - 1)
        tracer.deselect(i)
        # }
        idx += 1
    # visualize {
    tracer.deselect(start, N - 1)
    # }


for i in range(N - 1):
    # logger {
    logger.println("round {}".format(i + 1))
    # }
    curr_arr = D[i:N]
    curr_max_idx = 0
    curr_max_val = curr_arr[0]
    for idx, val in enumerate(curr_arr):
        if val > curr_max_val:
            curr_max_idx = idx
            curr_max_val = val
    if curr_max_idx != 0:  # if currMax.idx === 0 the max element is already at the bottom, no flip required
        # logger {
        logger.println("flip at {} (step 1)".format(curr_max_idx + i))
        # }
        flip(curr_max_idx + i)
        # logger {
        logger.println("flip at {} (step 2)".format(i))
        # }
        flip(i)

# logger {
logger.println("sorted array = [{}]".format(", ".join(map(str, D))))
# }
