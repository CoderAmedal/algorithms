import org.algorithm_visualizer.*;

class Main {

    public static void main(String[] args) {
        final int N = 10;
        int[] A = new int[N + 1];

        // define tracer variables {
        Array1DTracer tracer = new Array1DTracer(" Catalan Numbers ");
        LogTracer logger = new LogTracer();
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.set(A);
        Tracer.delay();
        // }

        A[0] = 1;
        // visualize {
        tracer.patch(0, A[0]);
        Tracer.delay();
        tracer.depatch(0);
        // }
        A[1] = 1;
        // visualize {
        tracer.patch(1, A[1]);
        Tracer.delay();
        tracer.depatch(1);
        // }

        for (int i = 2; i <= N; i++) {
            for (int j = 0; j < i; j++) {
                A[i] += A[j] * A[i - j - 1];
                // visualize {
                tracer.select(j);
                Tracer.delay();
                tracer.select(i - j - 1);
                Tracer.delay();
                tracer.patch(i, A[i]);
                Tracer.delay();
                tracer.deselect(j);
                tracer.deselect(i - j - 1);
                tracer.depatch(i);
                // }
            }
        }

        // visualize {
        logger.println(" The " + N + "th Catalan Number is " + A[N]);
        tracer.select(N);
        Tracer.delay();
        // }
    }
}
