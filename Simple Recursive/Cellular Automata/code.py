# import visualization libraries {
import random
from algorithm_visualizer import Array2DTracer, Tracer, Layout, VerticalLayout
# }

gridSize = 10
generations = 4
fillChance = 0.55

G = []
for i in range(gridSize):
    G.append([])
    for j in range(gridSize):
        if random.random() < fillChance or i == 0 or j == 0 or i == gridSize - 1 or j == gridSize - 1:
            G[i].append('#')
        else:
            G[i].append('.')

# define tracer variables {
tracer = Array2DTracer()
Layout.setRoot(VerticalLayout([tracer]))
tracer.set(G)
Tracer.delay()
# }

# visualize {
for gi in range(len(G)):
    for gj in range(len(G[gi])):
        if G[gi][gj] == '#':
            tracer.patch(gi, gj, G[gi][gj])
# }


def CellularAutomata(fillShape, emptyShape):
    nextGrid = []

    for i in range(len(G)):
        nextGrid.append([])
        for j in range(len(G[i])):
            adjCount = 0
            twoAwayCount = 0
            # look at the states of the neighboring cells
            for x in range(-2, 3):
                for y in range(-2, 3):
                    if (i + x >= 0 and i + x < len(G)) and (j + y >= 0 and j + y < len(G[i])):
                        if not (x != 0 and y != 0) and G[i + x][j + y] == emptyShape:
                            if x == -2 or x == 2 or y == -2 or y == 2:
                                twoAwayCount += 1
                            else:
                                adjCount += 1
            # change the current cell's state according to these rules
            if adjCount >= 5:
                nextGrid[i].append(fillShape)
            elif adjCount <= 1:
                if twoAwayCount < 3:
                    nextGrid[i].append(fillShape)
                else:
                    nextGrid[i].append(emptyShape)
            else:
                nextGrid[i].append(emptyShape)

    for i in range(len(nextGrid)):
        for j in range(len(nextGrid[i])):
            # visualize {
            tracer.depatch(i, j)
            tracer.select(i, j)
            Tracer.delay()
            # }
            G[i][j] = nextGrid[i][j]
            # visualize {
            if G[i][j] == fillShape:
                tracer.patch(i, j, G[i][j])
            else:
                tracer.patch(i, j, G[i][j])
                tracer.depatch(i, j)
                tracer.deselect(i, j)
            # }


for iter in range(generations):
    CellularAutomata('#', '.')
