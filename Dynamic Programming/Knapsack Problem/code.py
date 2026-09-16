# import visualization libraries {
from algorithm_visualizer import Array1DTracer, Array2DTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

val = [1, 4, 5, 7]  # The value of all available items
wt = [1, 3, 4, 5]  # The weights of available items
W = 7  # The maximum weight we can carry in our collection
N = len(val)
DP = [[0] * (W + 1) for _ in range(N + 1)]

# define tracer variables {
tracer = Array2DTracer('Knapsack Table')
valuesTracer = Array1DTracer('Values')
weightsTracer = Array1DTracer('Weights')
logger = LogTracer()
Layout.setRoot(VerticalLayout([tracer, valuesTracer, weightsTracer, logger]))
tracer.set(DP)
valuesTracer.set(val)
weightsTracer.set(wt)
Tracer.delay()
# }

for i in range(N + 1):
    for j in range(W + 1):
        if i == 0 or j == 0:
            # If we have no items or maximum weight we can take in collection is 0
            # then the total weight in our collection is 0
            DP[i][0] = 0
            # visualize {
            tracer.patch(i, j, DP[i][j])
            Tracer.delay()
            tracer.depatch(i, j)
            # }
        elif wt[i - 1] <= j:  # take the current item in our collection
            # visualize {
            weightsTracer.select(i - 1)
            valuesTracer.select(i - 1)
            Tracer.delay()
            tracer.select(i - 1, j - wt[i - 1])
            tracer.select(i - 1, j)
            Tracer.delay()
            # }
            A = val[i - 1] + DP[i - 1][j - wt[i - 1]]
            B = DP[i - 1][j]
            # find the maximum of these two values
            # and take which gives us a greater weight
            if A > B:
                DP[i][j] = A
                # visualize {
                tracer.patch(i, j, DP[i][j])
                Tracer.delay()
                # }
            else:
                DP[i][j] = B
                # visualize {
                tracer.patch(i, j, DP[i][j])
                Tracer.delay()
                # }
            # visualize {
            # opt subproblem depatch
            tracer.depatch(i, j)
            tracer.deselect(i - 1, j)
            tracer.deselect(i - 1, j - wt[i - 1])
            valuesTracer.deselect(i - 1)
            weightsTracer.deselect(i - 1)
            # }
        else:  # leave the current item from our collection
            DP[i][j] = DP[i - 1][j]
            # visualize {
            tracer.patch(i, j, DP[i][j])
            Tracer.delay()
            tracer.depatch(i, j)
            # }

# logger {
logger.println(" Best value we can achieve is {}".format(DP[N][W]))
# }
