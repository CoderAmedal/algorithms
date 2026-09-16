import org.algorithm_visualizer.*;

class Main {

    private static Array1DTracer tracer = new Array1DTracer();

    private static LogTracer logger = new LogTracer();

    private static final int[] D = {-2, -3, 4, -1, -2, 1, 5, -3};

    public static void main(String[] args) {
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.set(D);
        Tracer.delay();

        int maxSubarraySum = maxSubarray(D);

        logger.printf("Maximum Subarray's Sum is: %d\n", maxSubarraySum);
    }

    private static int maxSubarray(int[] array) {
        int maxSoFar = 0;
        int maxEndingHere = 0;

        logger.println("Initializing maxSoFar = 0 & maxEndingHere = 0");

        for (int i = 0; i < array.length; i++) {
            tracer.select(i);
            logger.printf("%d + %d\n", maxEndingHere, array[i]);
            maxEndingHere += array[i];
            logger.printf("=> %d\n", maxEndingHere);

            if (maxEndingHere < 0) {
                logger.println("maxEndingHere is negative, set to 0");
                maxEndingHere = 0;
            }

            if (maxSoFar < maxEndingHere) {
                logger.printf("maxSoFar < maxEndingHere, setting maxSoFar to maxEndingHere (%d)\n", maxEndingHere);
                maxSoFar = maxEndingHere;
            }

            Tracer.delay();
            tracer.deselect(i);
        }

        return maxSoFar;
    }
}
