import org.algorithm_visualizer.*;

class Main {

    private static Array1DTracer tracer = new Array1DTracer("Euclidean Algorithm");

    private static LogTracer logger = new LogTracer();

    private static int[] a = {465, 255};

    public static void main(String[] args) {
        tracer.set(a);
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        Tracer.delay();

        logger.printf("Finding the greatest common divisor of %d and %d\n", a[0], a[1]);

        logger.printf("Checking if first number is at most the second number\n");

        if (a[0] > a[1]) {
            int tmp = a[0];
            a[0] = a[1];
            a[1] = tmp;
            logger.printf("The first number is bigger than the second number. Switching the numbers.\n");
            tracer.set(a);
            Tracer.delay();
        }

        while (a[0] > 0) {
            logger.printf("%d %% %d = %d\n", a[1], a[0], a[1] % a[0]);
            logger.printf("Switching a[1] with a[1]%%a[0]\n");
            a[1] %= a[0];
            tracer.patch(1, a[1]);
            Tracer.delay();
            logger.printf("Now switching the two values to keep a[0] < a[1]\n");
            int tmp = a[0];
            a[0] = a[1];
            a[1] = tmp;
            tracer.patch(0, a[0]);
            tracer.patch(1, a[1]);
            Tracer.delay();
            tracer.depatch(0);
            tracer.depatch(1);
        }

        logger.printf("The greatest common divisor is %d\n", a[1]);
    }
}
