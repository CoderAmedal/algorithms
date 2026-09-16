import org.algorithm_visualizer.*;

class Main {

    private static Array1DTracer tracer1 = new Array1DTracer("String 1");

    private static Array1DTracer tracer2 = new Array1DTracer("String 2");

    private static Array2DTracer tracer3 = new Array2DTracer("Memo Table");

    private static LogTracer logger = new LogTracer();

    public static void main(String[] args) {
        String string1 = "AGGTAB";
        String string2 = "GXTXAYB";
        int m = string1.length();
        int n = string2.length();
        int[][] A = new int[m + 1][n + 1];

        Layout.setRoot(new VerticalLayout(new Commander[]{tracer1, tracer2, tracer3, logger}));
        tracer1.set(string1);
        tracer2.set(string2);
        tracer3.set(A);
        Tracer.delay();

        // Fill memo table in bottom up manner
        for (int i = 0; i <= m; i++) {
            for (int j = 0; j <= n; j++) {
                if (i == 0) {
                    A[i][j] = j;
                } else if (j == 0) {
                    A[i][j] = i;
                } else if (string1.charAt(i - 1) == string2.charAt(j - 1)) {
                    tracer1.select(i - 1);
                    Tracer.delay();
                    tracer2.select(j - 1);
                    Tracer.delay();
                    tracer3.select(i - 1, j - 1);
                    Tracer.delay();

                    A[i][j] = A[i - 1][j - 1] + 1;

                    tracer1.deselect(i - 1);
                    tracer2.deselect(j - 1);
                    tracer3.deselect(i - 1, j - 1);
                } else {
                    tracer3.select(i - 1, j);
                    Tracer.delay();
                    tracer3.select(i, j - 1);
                    Tracer.delay();

                    if (A[i - 1][j] < A[i][j - 1]) {
                        A[i][j] = 1 + A[i - 1][j];
                    } else {
                        A[i][j] = 1 + A[i][j - 1];
                    }

                    tracer3.deselect(i - 1, j);
                    tracer3.deselect(i, j - 1);
                }
                tracer3.patch(i, j, A[i][j]);
                Tracer.delay();
                tracer3.depatch(i, j);
            }
        }

        logger.printf("Shortest Common Supersequence is %d\n", A[m][n]);
    }
}
