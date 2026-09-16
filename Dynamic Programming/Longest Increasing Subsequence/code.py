# import visualization libraries {
from algorithm_visualizer import Array1DTracer, LogTracer, Randomize, Tracer, Layout, VerticalLayout
# }

# define tracer variables {
tracer = Array1DTracer()
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, logger]))
A = Randomize.Array1D(N=10, randomizer=Randomize.Integer(min=0, max=10)).create()
LIS = [1] * len(A)
tracer.set(A)
Tracer.delay()
# }

# Initialize LIS values for all indexes
for i in range(len(A)):
    LIS[i] = 1

# logger {
logger.println('Calculating Longest Increasing Subsequence values in bottom up manner ')
# }
# Compute optimized LIS values in bottom up manner
for i in range(1, len(A)):
    # visualize {
    tracer.select(i)
    logger.println(" LIS[{}] = {}".format(i, LIS[i]))
    # }
    for j in range(i):
        # visualize {
        tracer.patch(j)
        Tracer.delay()
        tracer.depatch(j)
        # }
        if A[i] > A[j] and LIS[i] < LIS[j] + 1:
            LIS[i] = LIS[j] + 1
            # logger {
            logger.println(" LIS[{}] = {}".format(i, LIS[i]))
            # }
    # visualize {
    tracer.deselect(i)
    # }

# Pick maximum of all LIS values
# logger {
logger.println('Now calculate maximum of all LIS values ')
# }
max_value = LIS[0]
for i in range(1, len(A)):
    if max_value < LIS[i]:
        max_value = LIS[i]
# logger {
logger.println("Longest Increasing Subsequence = max of all LIS = {}".format(max_value))
# }
