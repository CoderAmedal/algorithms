# import visualization libraries {
from algorithm_visualizer import Array2DTracer, Tracer, Layout, VerticalLayout
# }

N = 9
A = [[None] * N for _ in range(N)]

# define tracer variables {
tracer = Array2DTracer("Pascal's Triangle")
Layout.setRoot(VerticalLayout([tracer]))
tracer.set(A)
Tracer.delay()
# }

for i in range(N):
    for j in range(i + 1):
        if j == i or j == 0:  # First and last values in every row are 1
            A[i][j] = 1

            # visualize {
            tracer.patch(i, j, A[i][j])
            Tracer.delay()
            tracer.depatch(i, j)
            # }
        else:  # Other values are sum of values just above and left of above
            # visualize {
            tracer.select(i - 1, j - 1)
            Tracer.delay()
            tracer.select(i - 1, j)
            Tracer.delay()
            # }

            A[i][j] = A[i - 1][j - 1] + A[i - 1][j]

            # visualize {
            tracer.patch(i, j, A[i][j])
            Tracer.delay()
            tracer.depatch(i, j)
            tracer.deselect(i - 1, j - 1)
            tracer.deselect(i - 1, j)
            # }
