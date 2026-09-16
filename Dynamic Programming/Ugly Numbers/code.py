# import visualization libraries {
from algorithm_visualizer import Array1DTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

N = 15
A = [0] * N
A[0] = 1  # By convention 1 is an ugly number

M = [2, 3, 5]  # multiples of 2, 3, 5 respectively
I = [0, 0, 0]  # iterators of 2, 3, 5 respectively

# define tracer variables {
tracer = Array1DTracer('Ugly Numbers')
tracer2 = Array1DTracer('Multiples of 2, 3, 5')
tracer3 = Array1DTracer(' Iterators I0, I1, I2 ')
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, tracer2, tracer3, logger]))
tracer.set(A)
tracer2.set(M)
tracer3.set(I)
Tracer.delay()
# }

for i in range(1, N):
    # next is minimum of m2, m3 and m5
    next_value = M[0] if (M[0] <= M[1] and M[0] <= M[2]) else (M[1] if M[1] <= M[2] else M[2])
    # logger {
    logger.println(" Minimum of {}, {}, {} : {}".format(M[0], M[1], M[2], next_value))
    # }
    A[i] = next_value

    # visualize {
    tracer.patch(i, A[i])
    Tracer.delay()
    tracer.depatch(i)
    # }

    if next_value == M[0]:
        I[0] += 1
        M[0] = A[I[0]] * 2
        # visualize {
        tracer2.patch(0, M[0])
        Tracer.delay()
        tracer3.patch(0, I[0])
        Tracer.delay()
        tracer2.depatch(0)
        tracer3.depatch(0)
        # }
    if next_value == M[1]:
        I[1] += 1
        M[1] = A[I[1]] * 3
        # visualize {
        tracer2.patch(1, M[1])
        Tracer.delay()
        tracer3.patch(1, I[1])
        Tracer.delay()
        tracer2.depatch(1)
        tracer3.depatch(1)
        # }
    if next_value == M[2]:
        I[2] += 1
        M[2] = A[I[2]] * 5
        # visualize {
        tracer2.patch(2, M[2])
        Tracer.delay()
        tracer3.patch(2, I[2])
        Tracer.delay()
        tracer2.depatch(2)
        tracer3.depatch(2)
        # }
