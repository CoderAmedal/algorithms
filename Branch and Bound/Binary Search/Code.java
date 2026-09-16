// import visualization libraries {
import org.algorithm_visualizer.*;
import java.util.*;
// }

class Main {

    private static ChartTracer chart = new ChartTracer();

    private static Array1DTracer tracer = new Array1DTracer();

    private static LogTracer logger = new LogTracer();

    private static Integer[] D = (Integer[]) new Randomize.Array1D(15, new Randomize.Integer(0, 50)).sorted().create();

    public static void main(String[] args) {
        // define tracer variables {
        Layout.setRoot(new VerticalLayout(new Commander[]{chart, tracer, logger}));
        tracer.set(D);
        tracer.chart(chart);
        Tracer.delay();
        // }

        int element = D[new Randomize.Integer(0, D.length - 1).create()];

        // logger {
        logger.printf("Using iterative binary search to find %d\n", element);
        // }
        binarySearch(D, element);
    }

    // array = sorted array, element = element to be found
    private static int binarySearch(Integer[] array, int element) {
        int minIndex = 0;
        int maxIndex = array.length - 1;

        while (minIndex <= maxIndex) {
            int middleIndex = (minIndex + maxIndex) / 2;
            int testElement = array[middleIndex];

            // visualize {
            tracer.select(minIndex, maxIndex);
            Tracer.delay();
            tracer.patch(middleIndex);
            logger.printf("Searching at index: %d\n", middleIndex);
            Tracer.delay();
            tracer.depatch(middleIndex);
            tracer.deselect(minIndex, maxIndex);
            // }

            if (testElement < element) {
                // logger {
                logger.printf("Going right.\n");
                // }
                minIndex = middleIndex + 1;
            } else if (testElement > element) {
                // logger {
                logger.printf("Going left.\n");
                // }
                maxIndex = middleIndex - 1;
            } else {
                // visualize {
                logger.printf("%d is found at position %d!\n", element, middleIndex);
                tracer.select(middleIndex);
                // }

                return middleIndex;
            }
        }

        // logger {
        logger.printf("%d is not found!\n", element);
        // }
        return -1;
    }
}
