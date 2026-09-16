import org.algorithm_visualizer.*;

class Main {

    public static void main(String[] args) {
        String seq = "BBABCBCAB";
        int N = seq.length();

        int[][] L = new int[N][N];

        for (int i = 0; i < N; i++) {
            L[i][i] = 1;
        }

        // define tracer variables {
        Array1DTracer tracer = new Array1DTracer("Input Text");
        Array2DTracer matrix = new Array2DTracer("Matrix");
        LogTracer logger = new LogTracer();
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, matrix, logger}));
        tracer.set(seq);
        matrix.set(L);
        Tracer.delay();
        // }

        // logger {
        logger.println("LPS for any string with length = 1 is 1");
        // }
        for (int i = 2; i <= N; i++) {
            // logger {
            logger.println("--------------------------------------------------");
            logger.println("Considering a sub-string of length " + i);
            logger.println("--------------------------------------------------");
            // }
            for (int j = 0; j < N - i + 1; j++) {
                int k = j + i - 1;
                // visualize {
                tracer.select(j);
                Tracer.delay();
                tracer.patch(k);
                Tracer.delay();
                // }

                // logger {
                logger.println("Comparing " + seq.charAt(j) + " and " + seq.charAt(k));
                // }

                if (seq.charAt(j) == seq.charAt(k) && i == 2) {
                    // logger {
                    logger.println("They are equal and size of the string in the interval" + j + " to " + k + " is 2, so the Longest Palindromic Subsequence in the Given range is 2");
                    // }

                    // visualize {
                    matrix.patch(j, k);
                    Tracer.delay();
                    // }

                    L[j][k] = 2;
                    // visualize {
                    matrix.set(L);

                    matrix.depatch(j, k);
                    Tracer.delay();
                    // }
                } else if (seq.charAt(j) == seq.charAt(k)) {
                    // logger {
                    logger.println("They are equal, so the Longest Palindromic Subsequence in the Given range is 2 + the Longest Increasing Subsequence between the indices " + (j + 1) + " to " + (k - 1));
                    // }

                    // visualize {
                    matrix.patch(j, k);
                    Tracer.delay();
                    matrix.select(j + 1, k - 1);
                    Tracer.delay();
                    // }

                    L[j][k] = L[j + 1][k - 1] + 2;
                    // visualize {
                    matrix.set(L);

                    matrix.depatch(j, k);
                    Tracer.delay();
                    matrix.deselect(j + 1, k - 1);
                    Tracer.delay();
                    // }
                } else {
                    // logger {
                    logger.println("They are NOT equal, so the Longest Palindromic Subsequence in the Given range is the maximum Longest Increasing Subsequence between the indices " + (j + 1) + " to " + k + " and " + j + " to " + (k - 1));
                    // }
                    // visualize {
                    matrix.patch(j, k);
                    Tracer.delay();
                    matrix.select(j + 1, k);
                    Tracer.delay();
                    matrix.select(j, k - 1);
                    Tracer.delay();
                    // }

                    L[j][k] = Math.max(L[j + 1][k], L[j][k - 1]);
                    // visualize {
                    matrix.set(L);

                    matrix.depatch(j, k);
                    Tracer.delay();
                    matrix.deselect(j + 1, k);
                    Tracer.delay();
                    matrix.deselect(j, k - 1);
                    Tracer.delay();
                    // }
                }
                // logger {
                logger.println("--------------------------------------------------");
                // }
                // visualize {
                tracer.deselect(j);
                Tracer.delay();
                tracer.depatch(k);
                Tracer.delay();
                // }
            }
        }
        // logger {
        logger.println("Longest Increasing Subsequence of the given string = L[0][" + (N - 1) + "]=" + L[0][N - 1]);
        // }
    }
}
