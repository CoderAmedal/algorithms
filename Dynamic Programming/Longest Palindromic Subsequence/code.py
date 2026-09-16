# import visualization libraries {
from algorithm_visualizer import Array1DTracer, Array2DTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

seq = 'BBABCBCAB'
N = len(seq)

L = [[0] * N for _ in range(N)]

for i in range(N):
    L[i][i] = 1

# define tracer variables {
tracer = Array1DTracer('Input Text')
matrix = Array2DTracer('Matrix')
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, matrix, logger]))
tracer.set(seq)
matrix.set(L)
Tracer.delay()
# }


def max_value(a, b):
    if a > b:
        return a
    return b


# logger {
logger.println('LPS for any string with length = 1 is 1')
# }
for i in range(2, N + 1):
    # logger {
    logger.println('--------------------------------------------------')
    logger.println("Considering a sub-string of length {}".format(i))
    logger.println('--------------------------------------------------')
    # }
    for j in range(N - i + 1):
        k = j + i - 1
        # visualize {
        tracer.select(j)
        Tracer.delay()
        tracer.patch(k)
        Tracer.delay()
        # }

        # logger {
        logger.println("Comparing {} and {}".format(seq[j], seq[k]))
        # }

        if seq[j] == seq[k] and i == 2:
            # logger {
            logger.println("They are equal and size of the string in the interval{} to {} is 2, so the Longest Palindromic Subsequence in the Given range is 2".format(j, k))
            # }

            # visualize {
            matrix.patch(j, k)
            Tracer.delay()
            # }

            L[j][k] = 2
            # visualize {
            matrix.set(L)

            matrix.depatch(j, k)
            Tracer.delay()
            # }
        elif seq[j] == seq[k]:
            # logger {
            logger.println("They are equal, so the Longest Palindromic Subsequence in the Given range is 2 + the Longest Increasing Subsequence between the indices {} to {}".format(j + 1, k - 1))
            # }

            # visualize {
            matrix.patch(j, k)
            Tracer.delay()
            matrix.select(j + 1, k - 1)
            Tracer.delay()
            # }

            L[j][k] = L[j + 1][k - 1] + 2
            # visualize {
            matrix.set(L)

            matrix.depatch(j, k)
            Tracer.delay()
            matrix.deselect(j + 1, k - 1)
            Tracer.delay()
            # }
        else:
            # logger {
            logger.println("They are NOT equal, so the Longest Palindromic Subsequence in the Given range is the maximum Longest Increasing Subsequence between the indices {} to {} and {} to {}".format(j + 1, k, j, k - 1))
            # }
            # visualize {
            matrix.patch(j, k)
            Tracer.delay()
            matrix.select(j + 1, k)
            Tracer.delay()
            matrix.select(j, k - 1)
            Tracer.delay()
            # }

            L[j][k] = max_value(L[j + 1][k], L[j][k - 1])
            # visualize {
            matrix.set(L)

            matrix.depatch(j, k)
            Tracer.delay()
            matrix.deselect(j + 1, k)
            Tracer.delay()
            matrix.deselect(j, k - 1)
            Tracer.delay()
            # }
        # logger {
        logger.println('--------------------------------------------------')
        # }
        # visualize {
        tracer.deselect(j)
        Tracer.delay()
        tracer.depatch(k)
        Tracer.delay()
        # }
# logger {
logger.println("Longest Increasing Subsequence of the given string = L[0][{}]={}".format(N - 1, L[0][N - 1]))
# }
