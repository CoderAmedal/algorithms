import org.algorithm_visualizer.*;

class Main {

    private static Array1DTracer tracer = new Array1DTracer();

    private static LogTracer logger = new LogTracer();

    public static void main(String[] args) {
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        Integer[] D = (Integer[]) new Randomize.Array1D(20, new Randomize.Integer(-5, 5)).create();
        tracer.set(D);
        Tracer.delay();

        int sum = D[0] + D[1] + D[2];
        int max = sum;
        tracer.select(0, 2);
        logger.printf("sum = %d\n", sum);
        Tracer.delay();
        for (int i = 3; i < D.length; i++) {
            sum += D[i] - D[i - 3];
            if (max < sum) max = sum;
            tracer.deselect(i - 3);
            tracer.select(i);
            logger.printf("sum = %d\n", sum);
            Tracer.delay();
        }
        tracer.deselect(D.length - 3, D.length - 1);
        logger.printf("max = %d\n", max);
    }
}
