// import visualization libraries {
import org.algorithm_visualizer.*;
// }

/*
For N>3 the time taken by this algorithm is sufficiently high
Also it is not possible to visualise for N>6 due to stack overflow
caused by large number of recursive calls
*/
class Main {
    private static final int N = 3;
    private static int[][] board = new int[N][N];

    /*
    Define the next move of the knight
    */
    private static final int[] X = {2, 1, -1, -2, -2, -1, 1, 2};
    private static final int[] Y = {1, 2, 2, 1, -1, -2, -2, -1};

    private static int[] pos = new int[2];

    // define tracer variables {
    private static Array2DTracer boardTracer = new Array2DTracer("Board");
    private static Array1DTracer posTracer = new Array1DTracer("Knight Position");
    private static LogTracer logTracer = new LogTracer("Console");
    // }

    public static void main(String[] args) {
        for (int i = 0; i < N; i++) {
            for (int j = 0; j < N; j++) {
                board[i][j] = -1;
            }
        }
        pos[0] = -1;
        pos[1] = -1;

        // visualize {
        boardTracer.set(board);
        posTracer.set(pos);
        Layout.setRoot(new VerticalLayout(new Commander[]{boardTracer, posTracer, logTracer}));
        Tracer.delay();
        // }

        board[0][0] = 0; // start from this position
        pos[0] = 0;
        pos[0] = 0;

        // visualize {
        boardTracer.patch(0, 0, 0);
        Tracer.delay();
        posTracer.patch(0, 0);
        Tracer.delay();
        posTracer.patch(1, 0);
        Tracer.delay();
        boardTracer.depatch(0, 0);
        boardTracer.depatch(0, 0);
        posTracer.depatch(0);
        posTracer.depatch(1);
        // }

        // logger {
        if (knightTour(0, 0, 1) == false) {
            logTracer.println("Solution does not exist");
        } else {
            logTracer.println("Solution found");
        }
        // }
    }

    private static boolean knightTour(int x, int y, int moveNum) {
        if (moveNum == N * N) {
            return true;
        }

        for (int i = 0; i < 8; i++) {
            int nextX = x + X[i];
            int nextY = y + Y[i];

            // visualize {
            posTracer.patch(0, nextX);
            Tracer.delay();
            posTracer.patch(1, nextY);
            Tracer.delay();
            posTracer.depatch(0);
            posTracer.depatch(1);
            // }
            /*
            Check if knight is still in the board
            Check that knight does not visit an already visited square
            */
            if (nextX >= 0 && nextX < N && nextY >= 0 && nextY < N && board[nextX][nextY] == -1) {
                board[nextX][nextY] = moveNum;

                // visualize {
                logTracer.println("Move to " + nextX + "," + nextY);
                boardTracer.patch(nextX, nextY, moveNum);
                Tracer.delay();
                boardTracer.depatch(nextX, nextY);
                boardTracer.select(nextX, nextY);
                // }

                int nextMoveNum = moveNum + 1;
                if (knightTour(nextX, nextY, nextMoveNum) == true) {
                    return true;
                }

                // logger {
                logTracer.println("No place to move from " + nextX + "," + nextY + ": Backtrack");
                // }
                board[nextX][nextY] = -1; // backtrack
                // visualize {
                boardTracer.patch(nextX, nextY, -1);
                Tracer.delay();
                boardTracer.depatch(nextX, nextY);
                boardTracer.deselect(nextX, nextY);
                // }
            } else {
                // logger {
                logTracer.println(nextX + "," + nextY + " is not a valid move");
                // }
            }
        }
        return false;
    }
}
