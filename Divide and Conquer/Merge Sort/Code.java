import org.algorithm_visualizer.*;
import java.util.ArrayList;
import java.util.Arrays;

class Main {

    private static ChartTracer chart = new ChartTracer();

    private static Array1DTracer tracer = new Array1DTracer();

    private static LogTracer logger = new LogTracer();

    private static Integer[] D = (Integer[]) new Randomize.Array1D(15, new Randomize.Integer(1, 20)).create();

    public static void main(String[] args) {
        Layout.setRoot(new VerticalLayout(new Commander[]{chart, tracer, logger}));
        tracer.set(D);
        tracer.chart(chart);
        Tracer.delay();

        logger.printf("original array = %s\n", Arrays.toString(D));

        mergeSort(0, D.length);

        logger.printf("sorted array = %s\n", Arrays.toString(D));
    }

    private static void mergeSort(int start, int end) {
        if (Math.abs(end - start) <= 1) return;
        int middle = (int) Math.ceil((start + end) / 2.0);

        mergeSort(start, middle);
        mergeSort(middle, end);

        logger.printf("divide left[%d, %d], right[%d, %d]\n", start, middle - 1, middle, end - 1);
        merge(start, middle, end);
    }

    private static void merge(int start, int middle, int end) {
        int leftSize = middle - start;
        int rightSize = end - middle;
        int maxSize = Math.max(leftSize, rightSize);
        int size = end - start;
        ArrayList<Integer> left = new ArrayList<>();
        ArrayList<Integer> right = new ArrayList<>();

        for (int i = 0; i < maxSize; i++) {
            if (i < leftSize) {
                left.add(D[start + i]);
                tracer.select(start + i);
                logger.printf("insert value into left array[%d] = %d\n", i, D[start + i]);
                Tracer.delay();
            }
            if (i < rightSize) {
                right.add(D[middle + i]);
                tracer.select(middle + i);
                logger.printf("insert value into right array[%d] = %d\n", i, D[middle + i]);
                Tracer.delay();
            }
        }
        logger.printf("left array = %s, right array = %s\n", left, right);

        int i = 0;
        while (i < size) {
            if (!left.isEmpty() && !right.isEmpty()) {
                if (left.get(0) > right.get(0)) {
                    D[start + i] = right.remove(0);
                    logger.printf("rewrite from right array[%d] = %d\n", i, D[start + i]);
                } else {
                    D[start + i] = left.remove(0);
                    logger.printf("rewrite from left array[%d] = %d\n", i, D[start + i]);
                }
            } else if (!left.isEmpty()) {
                D[start + i] = left.remove(0);
                logger.printf("rewrite from left array[%d] = %d\n", i, D[start + i]);
            } else {
                D[start + i] = right.remove(0);
                logger.printf("rewrite from right array[%d] = %d\n", i, D[start + i]);
            }

            tracer.deselect(start + i);
            tracer.patch(start + i, D[start + i]);
            Tracer.delay();
            tracer.depatch(start + i);
            i++;
        }

        ArrayList<Integer> tempArray = new ArrayList<>();
        for (i = start; i < end; i++) tempArray.add(D[i]);
        logger.printf("merged array = %s\n", tempArray);
    }
}
