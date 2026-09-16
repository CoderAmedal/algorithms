# import visualization libraries {
from algorithm_visualizer import Array2DTracer, LogTracer, Randomize, Tracer, Layout, VerticalLayout
# }

# define tracer variables {
D = Randomize.Array2D(N=5, M=5, randomizer=Randomize.Integer(min=1, max=5)).create()
dataViewer = Array2DTracer()
tracer = Array2DTracer('Results Table')
logger = LogTracer()
Layout.setRoot(VerticalLayout([dataViewer, tracer, logger]))
dataViewer.set(D)
DP = []
for i in range(len(D)):
    DP.append([])
    for j in range(len(D[i])):
        DP[i].append(float('inf'))
tracer.set(DP)
Tracer.delay()
# }

N = len(DP)
M = len(DP[0])


def update(i, j, value):
    DP[i][j] = value
    # visualize {
    dataViewer.select(i, j)
    Tracer.delay()
    tracer.patch(i, j, DP[i][j])
    Tracer.delay()
    tracer.depatch(i, j)
    dataViewer.deselect(i, j)
    # }


for i in range(N):
    for j in range(M):
        if i == 0 and j == 0:
            update(i, j, D[i][j])
        elif i == 0:
            # visualize {
            tracer.select(i, j - 1)
            # }
            update(i, j, DP[i][j - 1] + D[i][j])
            # visualize {
            tracer.deselect(i, j - 1)
            # }
        elif j == 0:
            # visualize {
            tracer.select(i - 1, j)
            # }
            update(i, j, DP[i - 1][j] + D[i][j])
            # visualize {
            tracer.deselect(i - 1, j)
            # }
        else:
            # visualize {
            tracer.select(i, j - 1)
            tracer.select(i - 1, j)
            # }
            update(i, j, max(DP[i][j - 1], DP[i - 1][j]) + D[i][j])
            # visualize {
            tracer.deselect(i, j - 1)
            tracer.deselect(i - 1, j)
            # }
# logger {
logger.println("max = {}".format(DP[N - 1][M - 1]))
# }
