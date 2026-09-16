// import visualization libraries {
import org.algorithm_visualizer.*;
// }

class Main {

    private static final int n = 7;

    private static int[][] A = new int[n][n];

    // define tracer variables {
    private static Array2DTracer tracer = new Array2DTracer("Magic Square");

    private static LogTracer logTracer = new LogTracer("Console");
    // }

    public static void main(String[] args) {
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logTracer}));
        tracer.set(A);
        Tracer.delay();

        int i = n / 2;
        int j = n - 1;

        for (int num = 1; num <= n * n;) {
            // logger {
            logTracer.println("i = " + i);
            logTracer.println("j = " + j);
            // }

            if (i == -1 && j == n) {
                j = n - 2;
                i = 0;

                // logger {
                logTracer.println("Changing : ");
                logTracer.println("i = " + i);
                logTracer.println("j = " + j);
                // }
            } else {
                if (j == n) {
                    j = 0;
                    // logger {
                    logTracer.println("Changing : j = " + j);
                    // }
                }
                if (i < 0) {
                    i = n - 1;
                    // logger {
                    logTracer.println("Changing : i = " + i);
                    // }
                }
            }

            if (A[i][j] > 0) {
                // logger {
                logTracer.println("Cell already filled : Changing i = " + i + " j = " + j);
                // }
                j -= 2;
                i++;
            } else {
                A[i][j] = num++;
                // visualize {
                tracer.patch(i, j, A[i][j]);
                Tracer.delay();
                tracer.depatch(i, j);
                tracer.select(i, j);
                Tracer.delay();
                // }
                j++;
                i--;
            }
        }

        // logger {
        logTracer.println("Magic Constant is " + (n * (n * n + 1) / 2));
        // }
    }
}
