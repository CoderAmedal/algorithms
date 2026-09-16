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
logger.println("Original array = [{}]".format(", ".join(map(str, D))))
# }


def heapify(array, size, root):
    largest = root
    left = 2 * root + 1
    right = 2 * root + 2

    if left < size and array[left] > array[largest]:
        largest = left

    if right < size and array[right] > array[largest]:
        largest = right

    if largest != root:
        array[root], array[largest] = array[largest], array[root]

        # visualize {
        tracer.patch(root, array[root])
        tracer.patch(largest, array[largest])
        logger.println("Swapping elements : {} & {}".format(array[root], array[largest]))
        Tracer.delay()
        tracer.depatch(root)
        tracer.depatch(largest)
        # }

        heapify(array, size, largest)


def heap_sort(array, size):
    for i in range(size // 2 - 1, -1, -1):
        heapify(array, size, i)

    for j in range(size - 1, -1, -1):
        array[0], array[j] = array[j], array[0]

        # visualize {
        tracer.patch(0, array[0])
        tracer.patch(j, array[j])
        logger.println("Swapping elements : {} & {}".format(array[0], array[j]))
        Tracer.delay()
        tracer.depatch(0)
        tracer.depatch(j)
        tracer.select(j)
        Tracer.delay()
        # }

        heapify(array, j, 0)

        # visualize {
        tracer.deselect(j)
        # }


heap_sort(D, len(D))

# logger {
logger.println("Final array = [{}]".format(", ".join(map(str, D))))
# }
