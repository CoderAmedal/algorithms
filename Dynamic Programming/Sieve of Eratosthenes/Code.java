import org.algorithm_visualizer.*;

class Main {

    private static Array1DTracer tracer = new Array1DTracer("Sieve");

    private static LogTracer logger = new LogTracer();

    public static void main(String[] args) {
        int N = 30;
        int[] a = new int[N];
        int[] b = new int[N + 1];
        for (int i = 1; i <= N; i++) {
            a[i - 1] = i;
        }

        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.set(a);
        Tracer.delay();

        logger.println("1 is not prime");
        tracer.select(0);
        Tracer.delay();

        for (int i = 2; i <= N; i++) {
            if (b[i] == 0) {
                logger.printf("%d is not marked, so it is prime\n", i);
                // a[i-1] is prime mark by red indicators
                tracer.patch(i - 1);
                Tracer.delay();

                for (int j = i + i; j <= N; j += i) {
                    b[j] = 1; // a[j-1] is not prime, mark by blue indicators
                    logger.printf("%d is a multiple of %d so it is marked as composite\n", j, i);
                    tracer.select(j - 1);
                    Tracer.delay();
                }

                tracer.depatch(i - 1);
            }
        }
        logger.printf("The unmarked numbers are the prime numbers from 1 to %d\n", N);
    }
}
