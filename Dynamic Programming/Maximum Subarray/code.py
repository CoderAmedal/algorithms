# import visualization libraries {
from algorithm_visualizer import Array1DTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

D = [-2, -3, 4, -1, -2, 1, 5, -3]

# define tracer variables {
tracer = Array1DTracer()
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, logger]))
tracer.set(D)
Tracer.delay()
# }


def max_subarray(array):
    maxSoFar = 0
    maxEndingHere = 0

    # logger {
    logger.println('Initializing maxSoFar = 0 & maxEndingHere = 0')
    # }

    for i in range(len(array)):
        # visualize {
        tracer.select(i)
        # }
        # logger {
        logger.println("{} + {}".format(maxEndingHere, array[i]))
        # }
        maxEndingHere += array[i]
        # logger {
        logger.println("=> {}".format(maxEndingHere))
        # }

        if maxEndingHere < 0:
            # logger {
            logger.println('maxEndingHere is negative, set to 0')
            # }
            maxEndingHere = 0

        if maxSoFar < maxEndingHere:
            # logger {
            logger.println("maxSoFar < maxEndingHere, setting maxSoFar to maxEndingHere ({})".format(maxEndingHere))
            # }
            maxSoFar = maxEndingHere

        # visualize {
        Tracer.delay()
        tracer.deselect(i)
        # }

    return maxSoFar


maxSubarraySum = max_subarray(D)

# logger {
logger.println("Maximum Subarray's Sum is: {}".format(maxSubarraySum))
# }
