# import visualization libraries {
from algorithm_visualizer import Tracer, Array1DTracer, ChartTracer, LogTracer, Randomize, Layout, VerticalLayout
# }

# define tracer variables {
chart = ChartTracer()
tracer = Array1DTracer()
logger = LogTracer()
Layout.setRoot(VerticalLayout([chart, tracer, logger]))
D = Randomize.Array1D(N=15, randomizer=Randomize.Integer(min=0, max=50)).sorted().create()
tracer.set(D)
tracer.chart(chart)
Tracer.delay()
# }


def BinarySearch(array, element):  # array = sorted array, element = element to be found
    minIndex = 0
    maxIndex = len(array) - 1

    while minIndex <= maxIndex:
        middleIndex = (minIndex + maxIndex) // 2
        testElement = array[middleIndex]

        # visualize {
        tracer.select(minIndex, maxIndex)
        Tracer.delay()
        tracer.patch(middleIndex)
        logger.println("Searching at index: {}".format(middleIndex))
        Tracer.delay()
        tracer.depatch(middleIndex)
        tracer.deselect(minIndex, maxIndex)
        # }

        if testElement < element:
            # logger {
            logger.println('Going right.')
            # }
            minIndex = middleIndex + 1
        elif testElement > element:
            # logger {
            logger.println('Going left.')
            # }
            maxIndex = middleIndex - 1
        else:
            # visualize {
            logger.println("{} is found at position {}!".format(element, middleIndex))
            tracer.select(middleIndex)
            # }

            return middleIndex

    # logger {
    logger.println("{} is not found!".format(element))
    # }
    return -1


element = D[Randomize.Integer(min=0, max=len(D) - 1).create()]

# logger {
logger.println("Using iterative binary search to find {}".format(element))
# }
BinarySearch(D, element)
