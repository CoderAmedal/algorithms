# import visualization libraries {
from algorithm_visualizer import Array1DTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

N = 30
a = []
b = [0] * (N + 1)
for i in range(1, N + 1):
    a.append(i)

# define tracer variables {
tracer = Array1DTracer('Sieve')
tracer.set(a)
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, logger]))
Tracer.delay()
# }

# visualize {
logger.println('1 is not prime')
tracer.select(0)
Tracer.delay()
# }
for i in range(2, N + 1):
    if b[i] == 0:
        # visualize {
        logger.println("{} is not marked, so it is prime".format(i))
        # a[i-1] is prime mark by red indicators
        tracer.patch(i - 1)
        Tracer.delay()
        # }
        for j in range(i + i, N + 1, i):
            b[j] = 1  # a[j-1] is not prime, mark by blue indicators
            # visualize {
            logger.println("{} is a multiple of {} so it is marked as composite".format(j, i))
            tracer.select(j - 1)
            Tracer.delay()
            # }
        # visualize {
        tracer.depatch(i - 1)
        # }
# logger {
logger.println("The unmarked numbers are the prime numbers from 1 to {}".format(N))
# }
