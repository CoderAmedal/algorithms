# import visualization libraries {
from algorithm_visualizer import Tracer, Array1DTracer, LogTracer, Layout, VerticalLayout
# }

a = []
a.append(465)
a.append(255)

# define tracer variables {
tracer = Array1DTracer('Euclidean Algorithm')
tracer.set(a)
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, logger]))
Tracer.delay()
# }

# logger {
logger.println("Finding the greatest common divisor of {} and {}".format(a[0], a[1]))

logger.println('Checking if first number is at most the second number')
# }

if a[0] > a[1]:
    tmp = a[0]
    a[0] = a[1]
    a[1] = tmp
    # logger {
    logger.println('The first number is bigger than the second number. Switching the numbers.')
    # }
    # visualize {
    tracer.set(a)
    Tracer.delay()
    # }

while a[0] > 0:
    # logger {
    logger.println("{} % {} = {}".format(a[1], a[0], a[1] % a[0]))
    logger.println('Switching a[1] with a[1]%a[0]')
    # }
    a[1] %= a[0]
    # visualize {
    tracer.patch(1, a[1])
    Tracer.delay()
    # }
    # logger {
    logger.println('Now switching the two values to keep a[0] < a[1]')
    # }
    tmp = a[0]
    a[0] = a[1]
    a[1] = tmp
    # visualize {
    tracer.patch(0, a[0])
    tracer.patch(1, a[1])
    Tracer.delay()
    tracer.depatch(0)
    tracer.depatch(1)
    # }

# logger {
logger.println("The greatest common divisor is {}".format(a[1]))
# }
