# import visualization libraries {
from algorithm_visualizer import Array2DTracer, LogTracer, Tracer, Layout, VerticalLayout
# }

N = 4  # just change the value of N and the visuals will reflect the configuration!
board = [[0] * N for _ in range(N)]
queens = [[-1, -1] for _ in range(N)]

# define tracer variables {
board_tracer = Array2DTracer('Board')
queen_tracer = Array2DTracer('Queen Positions')
logger = LogTracer('Progress')
Layout.setRoot(VerticalLayout([board_tracer, queen_tracer, logger]))

board_tracer.set(board)
queen_tracer.set(queens)
logger.println("N Queens: {}X{} matrix, {} queens".format(N, N, N))
Tracer.delay()
# }


def valid_state(row, col, current_queen):
    for q in range(current_queen):
        current_q = queens[q]
        if row == current_q[0] or col == current_q[1] or abs(current_q[0] - row) == abs(current_q[1] - col):
            return False
    return True


def nq(current_queen, current_col):
    # logger {
    logger.println("Starting new iteration of nQueens () with currentQueen = {} & currentCol = {}".format(current_queen, current_col))
    logger.println('------------------------------------------------------------------')
    # }
    if current_queen >= N:
        # logger {
        logger.println('The recursion has BOTTOMED OUT. All queens have been placed successfully')
        # }
        return True

    found = False
    row = 0
    while row < N and not found:
        # visualize {
        board_tracer.select(row, current_col)
        Tracer.delay()
        logger.println("Trying queen {} at row {} & col {}".format(current_queen, row, current_col))
        # }

        if valid_state(row, current_col, current_queen):
            queens[current_queen][0] = row
            queens[current_queen][1] = current_col

            # visualize {
            queen_tracer.patch(current_queen, 0, row)
            Tracer.delay()
            queen_tracer.patch(current_queen, 1, current_col)
            Tracer.delay()
            queen_tracer.depatch(current_queen, 0)
            Tracer.delay()
            queen_tracer.depatch(current_queen, 1)
            Tracer.delay()
            # }

            found = nq(current_queen + 1, current_col + 1)

        if not found:
            # visualize {
            board_tracer.deselect(row, current_col)
            Tracer.delay()
            logger.println("row {} & col {} didn't work out. Going down".format(row, current_col))
            # }
        row += 1

    return found


# logger {
logger.println('Starting execution')
# }
nq(0, 0)
# logger {
logger.println('DONE')
# }
