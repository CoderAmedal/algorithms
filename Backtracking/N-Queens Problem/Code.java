// import visualization libraries {
import org.algorithm_visualizer.*;
// }

class Main {
    // define tracer variables {
    private static Array2DTracer boardTracer = new Array2DTracer("Board");
    private static Array2DTracer queenTracer = new Array2DTracer("Queen Positions");
    private static LogTracer logger = new LogTracer("Progress");
    // }

    private static final int N = 4; // just change the value of N and the visuals will reflect the configuration!
    private static int[][] board = new int[N][N];
    private static int[][] queens = new int[N][2];

    public static void main(String[] args) {
        for (int i = 0; i < N; i++) {
            for (int j = 0; j < N; j++) {
                board[i][j] = 0;
            }
            queens[i][0] = -1;
            queens[i][1] = -1;
        }

        // visualize {
        Layout.setRoot(new VerticalLayout(new Commander[]{boardTracer, queenTracer, logger}));
        boardTracer.set(board);
        queenTracer.set(queens);
        logger.println("N Queens: " + N + "X" + N + "matrix, " + N + " queens");
        Tracer.delay();
        // }

        // logger {
        logger.println("Starting execution");
        // }
        nQ(0, 0);
        // logger {
        logger.println("DONE");
        // }
    }

    private static boolean validState(int row, int col, int currentQueen) {
        for (int q = 0; q < currentQueen; q++) {
            int[] currentQ = queens[q];
            if (row == currentQ[0] || col == currentQ[1]
                    || (Math.abs(currentQ[0] - row) == Math.abs(currentQ[1] - col))) {
                return false;
            }
        }
        return true;
    }

    private static boolean nQ(int currentQueen, int currentCol) {
        // logger {
        logger.println("Starting new iteration of nQueens () with currentQueen = " + currentQueen + " & currentCol = " + currentCol);
        logger.println("------------------------------------------------------------------");
        // }
        if (currentQueen >= N) {
            // logger {
            logger.println("The recursion has BOTTOMED OUT. All queens have been placed successfully");
            // }
            return true;
        }

        boolean found = false;
        int row = 0;
        while (row < N && !found) {
            // visualize {
            boardTracer.select(row, currentCol);
            Tracer.delay();
            logger.println("Trying queen " + currentQueen + " at row " + row + " & col " + currentCol);
            // }

            if (validState(row, currentCol, currentQueen)) {
                queens[currentQueen][0] = row;
                queens[currentQueen][1] = currentCol;

                // visualize {
                queenTracer.patch(currentQueen, 0, row);
                Tracer.delay();
                queenTracer.patch(currentQueen, 1, currentCol);
                Tracer.delay();
                queenTracer.depatch(currentQueen, 0);
                Tracer.delay();
                queenTracer.depatch(currentQueen, 1);
                Tracer.delay();
                // }

                found = nQ(currentQueen + 1, currentCol + 1);
            }

            if (!found) {
                // visualize {
                boardTracer.deselect(row, currentCol);
                Tracer.delay();
                logger.println("row " + row + " & col " + currentCol + " didn't work out. Going down");
                // }
            }
            row++;
        }

        return found;
    }
}
