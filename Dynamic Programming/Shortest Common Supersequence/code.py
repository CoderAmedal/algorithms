# import visualization libraries {
from algorithm_visualizer import Array1DTracer, Array2DTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

string1 = 'AGGTAB'
string2 = 'GXTXAYB'
m = len(string1)
n = len(string2)
A = [[0] * (n + 1) for _ in range(m + 1)]

# define tracer variables {
tracer1 = Array1DTracer('String 1')
tracer2 = Array1DTracer('String 2')
tracer3 = Array2DTracer('Memo Table')
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer1, tracer2, tracer3, logger]))
tracer1.set(string1)
tracer2.set(string2)
tracer3.set(A)
Tracer.delay()
# }

# Fill memo table in bottom up manner
for i in range(m + 1):
    for j in range(n + 1):
        if i == 0:
            A[i][j] = j
        elif j == 0:
            A[i][j] = i
        elif string1[i - 1] == string2[j - 1]:
            # visualize {
            tracer1.select(i - 1)
            Tracer.delay()
            tracer2.select(j - 1)
            Tracer.delay()
            tracer3.select(i - 1, j - 1)
            Tracer.delay()
            # }

            A[i][j] = A[i - 1][j - 1] + 1

            # visualize {
            tracer1.deselect(i - 1)
            tracer2.deselect(j - 1)
            tracer3.deselect(i - 1, j - 1)
            # }
        else:
            # visualize {
            tracer3.select(i - 1, j)
            Tracer.delay()
            tracer3.select(i, j - 1)
            Tracer.delay()
            # }

            if A[i - 1][j] < A[i][j - 1]:
                A[i][j] = 1 + A[i - 1][j]
            else:
                A[i][j] = 1 + A[i][j - 1]

            # visualize {
            tracer3.deselect(i - 1, j)
            tracer3.deselect(i, j - 1)
            # }
        # visualize {
        tracer3.patch(i, j, A[i][j])
        Tracer.delay()
        tracer3.depatch(i, j)
        # }

# logger {
logger.println("Shortest Common Supersequence is {}".format(A[m][n]))
# }
