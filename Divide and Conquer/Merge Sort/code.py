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


def merge(start, middle, end):
    leftSize = middle - start
    rightSize = end - middle
    maxSize = max(leftSize, rightSize)
    size = end - start
    left = []
    right = []

    for i in range(maxSize):
        if i < leftSize:
            left.append(D[start + i])
            # visualize {
            tracer.select(start + i)
            logger.println("insert value into left array[{}] = {}".format(i, D[start + i]))
            Tracer.delay()
            # }
        if i < rightSize:
            right.append(D[middle + i])
            # visualize {
            tracer.select(middle + i)
            logger.println("insert value into right array[{}] = {}".format(i, D[middle + i]))
            Tracer.delay()
            # }
    # logger {
    logger.println("left array = [{}], right array = [{}]".format(", ".join(map(str, left)), ", ".join(map(str, right))))
    # }

    i = 0
    while i < size:
        if left and right:
            if left[0] > right[0]:
                D[start + i] = right.pop(0)
                # logger {
                logger.println("rewrite from right array[{}] = {}".format(i, D[start + i]))
                # }
            else:
                D[start + i] = left.pop(0)
                # logger {
                logger.println("rewrite from left array[{}] = {}".format(i, D[start + i]))
                # }
        elif left:
            D[start + i] = left.pop(0)
            # logger {
            logger.println("rewrite from left array[{}] = {}".format(i, D[start + i]))
            # }
        else:
            D[start + i] = right.pop(0)
            # logger {
            logger.println("rewrite from right array[{}] = {}".format(i, D[start + i]))
            # }

        # visualize {
        tracer.deselect(start + i)
        tracer.patch(start + i, D[start + i])
        Tracer.delay()
        tracer.depatch(start + i)
        # }
        i += 1

    tempArray = [D[i] for i in range(start, end)]
    # logger {
    logger.println("merged array = [{}]".format(", ".join(map(str, tempArray))))
    # }


def merge_sort(start, end):
    if abs(end - start) <= 1:
        return
    middle = (start + end + 1) // 2

    merge_sort(start, middle)
    merge_sort(middle, end)

    # logger {
    logger.println("divide left[{}, {}], right[{}, {}]".format(start, middle - 1, middle, end - 1))
    # }
    merge(start, middle, end)


merge_sort(0, len(D))
# logger {
logger.println("sorted array = [{}]".format(", ".join(map(str, D))))
# }
