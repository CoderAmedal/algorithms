# import visualization libraries {
from algorithm_visualizer import Array2DTracer, Tracer, Layout, VerticalLayout
# }

G = [
    ['#', '#', '#', '#', '#', '#', '#', '#', '#'],
    ['#', '-', '-', '-', '#', '-', '-', '-', '#'],
    ['#', '-', '-', '-', '#', '-', '-', '-', '#'],
    ['#', '-', '-', '#', '-', '-', '-', '-', '#'],
    ['#', '#', '#', '-', '-', '-', '#', '#', '#'],
    ['#', '-', '-', '-', '-', '#', '-', '-', '#'],
    ['#', '-', '-', '-', '#', '-', '-', '-', '#'],
    ['#', '-', '-', '-', '#', '-', '-', '-', '#'],
    ['#', '#', '#', '#', '#', '#', '#', '#', '#'],
]

# define tracer variables {
tracer = Array2DTracer()
Layout.setRoot(VerticalLayout([tracer]))
tracer.set(G)
Tracer.delay()
# }


def flood_fill(i, j, old_color, new_color):
    if i < 0 or i >= len(G) or j < 0 or j >= len(G[i]):
        return
    if G[i][j] != old_color:
        return

    # set the color of node to newColor
    G[i][j] = new_color

    # visualize {
    tracer.select(i, j)
    Tracer.delay()
    tracer.patch(i, j, G[i][j])
    Tracer.delay()
    # }

    # next step four-way
    flood_fill(i + 1, j, old_color, new_color)
    flood_fill(i - 1, j, old_color, new_color)
    flood_fill(i, j + 1, old_color, new_color)
    flood_fill(i, j - 1, old_color, new_color)


flood_fill(4, 4, '-', 'a')
