import org.algorithm_visualizer.*;

class Main {

    public static void main(String[] args) {
        String string1 = "AGGTAB";
        String string2 = "GXTXAYB";
        int m = string1.length();
        int n = string2.length();
        int[][] A = new int[m + 1][n + 1];

        // define tracer variables {
        Array1DTracer tracer1 = new Array1DTracer("String 1");
        Array1DTracer tracer2 = new Array1DTracer("String 2");
        Array2DTracer tracer3 = new Array2DTracer("Memo Table");
        LogTracer logger = new LogTracer();
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer1, tracer2, tracer3, logger}));
        tracer1.set(string1);
        tracer2.set(string2);
        tracer3.set(A);
        Tracer.delay();
        // }

        int i;
        int j;

        // Build the memo table in bottom up fashion
        for (i = 0; i <= m; i++) {
            for (j = 0; j <= n; j++) {
                if (i == 0 || j == 0) {
                    A[i][j] = 0;
                } else if (string1.charAt(i - 1) == string2.charAt(j - 1)) {
                    // visualize {
                    tracer1.select(i - 1);
                    Tracer.delay();
                    tracer2.select(j - 1);
                    Tracer.delay();
                    tracer3.select(i - 1, j - 1);
                    Tracer.delay();
                    // }

                    A[i][j] = A[i - 1][j - 1] + 1;

                    // visualize {
                    tracer1.deselect(i - 1);
                    tracer2.deselect(j - 1);
                    tracer3.deselect(i - 1, j - 1);
                    // }
                } else {
                    // visualize {
                    tracer3.select(i - 1, j);
                    Tracer.delay();
                    tracer3.select(i, j - 1);
                    Tracer.delay();
                    // }

                    if (A[i - 1][j] > A[i][j - 1]) {
                        A[i][j] = A[i - 1][j];
                    } else {
                        A[i][j] = A[i][j - 1];
                    }

                    // visualize {
                    tracer3.deselect(i - 1, j);
                    tracer3.deselect(i, j - 1);
                    // }
                }
                // visualize {
                tracer3.patch(i, j, A[i][j]);
                Tracer.delay();
                tracer3.depatch(i, j);
                // }
            }
        }

        StringBuilder finalString = new StringBuilder();
        i = m;
        j = n;
        while (i >= 1 && j >= 1) {
            // visualize {
            tracer3.select(i, j);
            Tracer.delay();
            // }
            if (string1.charAt(i - 1) == string2.charAt(j - 1)) {
                // visualize {
                tracer1.select(i - 1);
                Tracer.delay();
                tracer2.select(j - 1);
                Tracer.delay();
                // }

                finalString.insert(0, string1.charAt(i - 1));
                i--;
                j--;
            } else if (A[i - 1][j] > A[i][j - 1]) {
                i--;
            } else {
                j--;
            }
        }

        // logger {
        logger.println("Longest Common Subsequence Length is " + A[m][n]);
        logger.println("Longest Common Subsequence is " + finalString);
        // }
    }
}
