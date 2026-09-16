# import visualization libraries {
from algorithm_visualizer import Array1DTracer, Array2DTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

# /*
# For N>3 the time taken by this algorithm is sufficiently high
# Also it is not possible to visualise for N>6 due to stack overflow
# caused by large number of recursive calls
# */
N = 3
board = [[-1] * N for _ in range(N)]

# /*
# Define the next move of the knight
# */
X = [2, 1, -1, -2, -2, -1, 1, 2]
Y = [1, 2, 2, 1, -1, -2, -2, -1]

pos = [-1, -1]

# define tracer variables {
board_tracer = Array2DTracer('Board')
pos_tracer = Array1DTracer('Knight Position')
log_tracer = LogTracer('Console')
board_tracer.set(board)
pos_tracer.set(pos)
Layout.setRoot(VerticalLayout([board_tracer, pos_tracer, log_tracer]))
Tracer.delay()
# }


def knight_tour(x, y, move_num):
    if move_num == N * N:
        return True

    for i in range(8):
        next_x = x + X[i]
        next_y = y + Y[i]

        # visualize {
        pos_tracer.patch(0, next_x)
        Tracer.delay()
        pos_tracer.patch(1, next_y)
        Tracer.delay()
        pos_tracer.depatch(0)
        pos_tracer.depatch(1)
        # }
        # /*
        # Check if knight is still in the board
        # Check that knight does not visit an already visited square
        # */
        if 0 <= next_x < N and 0 <= next_y < N and board[next_x][next_y] == -1:
            board[next_x][next_y] = move_num

            # visualize {
            log_tracer.println("Move to {},{}".format(next_x, next_y))
            board_tracer.patch(next_x, next_y, move_num)
            Tracer.delay()
            board_tracer.depatch(next_x, next_y)
            board_tracer.select(next_x, next_y)
            # }

            next_move_num = move_num + 1
            if knight_tour(next_x, next_y, next_move_num) is True:
                return True

            # logger {
            log_tracer.println("No place to move from {},{}: Backtrack".format(next_x, next_y))
            # }
            board[next_x][next_y] = -1  # backtrack
            # visualize {
            board_tracer.patch(next_x, next_y, -1)
            Tracer.delay()
            board_tracer.depatch(next_x, next_y)
            board_tracer.deselect(next_x, next_y)
            # }
        else:
            # logger {
            log_tracer.println("{},{} is not a valid move".format(next_x, next_y))
            # }
    return False


board[0][0] = 0  # start from this position
pos[0] = 0
pos[0] = 0

# visualize {
board_tracer.patch(0, 0, 0)
Tracer.delay()
pos_tracer.patch(0, 0)
Tracer.delay()
pos_tracer.patch(1, 0)
Tracer.delay()
board_tracer.depatch(0, 0)
board_tracer.depatch(0, 0)
pos_tracer.depatch(0)
pos_tracer.depatch(1)
# }

# logger {
if knight_tour(0, 0, 1) is False:
    log_tracer.println('Solution does not exist')
else:
    log_tracer.println('Solution found')
# }
