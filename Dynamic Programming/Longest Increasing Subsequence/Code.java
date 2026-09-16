import org.algorithm_visualizer.*;

class Main {

    public static void main(String[] args) {
        // define tracer variables {
        Array1DTracer tracer = new Array1DTracer();
        LogTracer logger = new LogTracer();
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        Integer[] A = (Integer[]) new Randomize.Array1D(10, new Randomize.Integer(0, 10)).create();
        int[] LIS = new int[A.length];
        tracer.set(A);
        Tracer.delay();
        // }

        // Initialize LIS values for all indexes
        for (int i = 0; i < A.length; i++) {
            LIS[i] = 1;
        }

        // logger {
        logger.println("Calculating Longest Increasing Subsequence values in bottom up manner ");
        // }
        // Compute optimized LIS values in bottom up manner
        for (int i = 1; i < A.length; i++) {
            // visualize {
            tracer.select(i);
            logger.println(" LIS[" + i + "] = " + LIS[i]);
            // }
            for (int j = 0; j < i; j++) {
                // visualize {
                tracer.patch(j);
                Tracer.delay();
                tracer.depatch(j);
                // }
                if (A[i] > A[j] && LIS[i] < LIS[j] + 1) {
                    LIS[i] = LIS[j] + 1;
                    // logger {
                    logger.println(" LIS[" + i + "] = " + LIS[i]);
                    // }
                }
            }
            // visualize {
            tracer.deselect(i);
            // }
        }

        // Pick maximum of all LIS values
        // logger {
        logger.println("Now calculate maximum of all LIS values ");
        // }
        int max = LIS[0];
        for (int i = 1; i < A.length; i++) {
            if (max < LIS[i]) {
                max = LIS[i];
            }
        }
        // logger {
        logger.println("Longest Increasing Subsequence = max of all LIS = " + max);
        // }
    }
}
