import org.algorithm_visualizer.*;

class Main {

    public static void main(String[] args) {
        int[] val = {1, 4, 5, 7}; // The value of all available items
        int[] wt = {1, 3, 4, 5}; // The weights of available items
        int W = 7; // The maximum weight we can carry in our collection
        int N = val.length;
        int[][] DP = new int[N + 1][W + 1];

        // define tracer variables {
        Array2DTracer tracer = new Array2DTracer("Knapsack Table");
        Array1DTracer valuesTracer = new Array1DTracer("Values");
        Array1DTracer weightsTracer = new Array1DTracer("Weights");
        LogTracer logger = new LogTracer();
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, valuesTracer, weightsTracer, logger}));
        tracer.set(DP);
        valuesTracer.set(val);
        weightsTracer.set(wt);
        Tracer.delay();
        // }

        for (int i = 0; i <= N; i++) {
            for (int j = 0; j <= W; j++) {
                if (i == 0 || j == 0) {
                    /*
                    If we have no items or maximum weight we can take in collection is 0
                    then the total weight in our collection is 0
                    */
                    DP[i][0] = 0;
                    // visualize {
                    tracer.patch(i, j, DP[i][j]);
                    Tracer.delay();
                    tracer.depatch(i, j);
                    // }
                } else if (wt[i - 1] <= j) { // take the current item in our collection
                    // visualize {
                    weightsTracer.select(i - 1);
                    valuesTracer.select(i - 1);
                    Tracer.delay();
                    tracer.select(i - 1, j - wt[i - 1]);
                    tracer.select(i - 1, j);
                    Tracer.delay();
                    // }
                    int A = val[i - 1] + DP[i - 1][j - wt[i - 1]];
                    int B = DP[i - 1][j];
                    /*
                    find the maximum of these two values
                    and take which gives us a greater weight
                    */
                    if (A > B) {
                        DP[i][j] = A;
                        // visualize {
                        tracer.patch(i, j, DP[i][j]);
                        Tracer.delay();
                        // }
                    } else {
                        DP[i][j] = B;
                        // visualize {
                        tracer.patch(i, j, DP[i][j]);
                        Tracer.delay();
                        // }
                    }
                    // visualize {
                    // opt subproblem depatch
                    tracer.depatch(i, j);
                    tracer.deselect(i - 1, j);
                    tracer.deselect(i - 1, j - wt[i - 1]);
                    valuesTracer.deselect(i - 1);
                    weightsTracer.deselect(i - 1);
                    // }
                } else { // leave the current item from our collection
                    DP[i][j] = DP[i - 1][j];
                    // visualize {
                    tracer.patch(i, j, DP[i][j]);
                    Tracer.delay();
                    tracer.depatch(i, j);
                    // }
                }
            }
        }

        // logger {
        logger.println(" Best value we can achieve is " + DP[N][W]);
        // }
    }
}
