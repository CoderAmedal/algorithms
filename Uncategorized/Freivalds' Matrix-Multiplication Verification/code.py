# import visualization libraries {
from algorithm_visualizer import Tracer, Array1DTracer, Array2DTracer, LogTracer, Layout, VerticalLayout
import random
# }

A = [[2, 3], [3, 4]]
B = [[1, 0], [1, 2]]
C = [[6, 5], [8, 7]]

# define tracer variables {
matrixATracer = Array2DTracer('Matrix A')
matrixBTracer = Array2DTracer('Matrix B')
matrixCTracer = Array2DTracer('Matrix C')
randomVectorTracer = Array1DTracer('Random Vector')
resultVectorTracer = Array1DTracer('Result Vector')
logger = LogTracer()
Layout.setRoot(VerticalLayout([matrixATracer, matrixBTracer, matrixCTracer, randomVectorTracer, resultVectorTracer, logger]))
matrixATracer.set(A)
matrixBTracer.set(B)
matrixCTracer.set(C)
Tracer.delay()
# }


def FreivaldsAlgorithm():
    k = 5
    n = len(A)

    while k > 0:
        k -= 1
        # logger {
        logger.println("Iterations remained: #{}".format(k))
        # }

        # Generate random vector
        r = []
        P = []
        for i in range(n):
            P.append(-1)
            r.append(1 if random.random() < 0.5 else 0)
        # visualize {
        randomVectorTracer.set(r)
        Tracer.delay()
        # }

        # Compute Br, Cr
        Br = []
        Cr = []
        for i in range(n):
            tmpB = 0
            tmpC = 0
            for j in range(n):
                tmpB += r[j] * B[j][i]
                tmpC += r[j] * C[j][i]
            Br.append(tmpB)
            Cr.append(tmpC)

        # Compute A * Br - Cr
        P = []
        for i in range(n):
            tmp = 0
            for j in range(n):
                tmp += (A[i][j] * Br[i]) - Cr[i]
            P.append(tmp)
        # visualize {
        resultVectorTracer.set(P)
        Tracer.delay()
        # }

        for i in range(n):
            if P[i] != 0:
                # logger {
                logger.println("P[{}] !== 0 ({}), exit".format(i, P[i]))
                # }
                return False

        # logger {
        logger.println('Result vector is identity, continue...')
        # }

    return True


FreivaldsAlgorithm()
