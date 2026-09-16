# import visualization libraries {
from algorithm_visualizer import Array1DTracer, LogTracer, Randomize, Tracer, Layout, VerticalLayout
# }

# define tracer variables {
tracer = Array1DTracer()
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, logger]))
D = Randomize.Array1D(N=20, randomizer=Randomize.Integer(min=-5, max=5)).create()
tracer.set(D)
Tracer.delay()
# }

sum_value = D[0] + D[1] + D[2]
max_value = sum_value
# visualize {
tracer.select(0, 2)
logger.println("sum = {}".format(sum_value))
Tracer.delay()
# }
for i in range(3, len(D)):
    sum_value += D[i] - D[i - 3]
    if max_value < sum_value:
        max_value = sum_value
    # visualize {
    tracer.deselect(i - 3)
    tracer.select(i)
    logger.println("sum = {}".format(sum_value))
    Tracer.delay()
    # }
# visualize {
tracer.deselect(len(D) - 3, len(D) - 1)
logger.println("max = {}".format(max_value))
# }
