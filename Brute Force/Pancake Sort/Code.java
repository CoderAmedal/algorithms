import org.algorithm_visualizer.*;
import java.util.Arrays;

class Main {

    private static ChartTracer chartTracer = new ChartTracer();

    private static LogTracer logTracer = new LogTracer("Console");

    private static Integer[] array = (Integer[]) new Randomize.Array1D(10, new Randomize.Integer(1, 20)).create();

    public static void main(String[] args) {
        int N = array.length;
        Layout.setRoot(new VerticalLayout(new Commander[]{chartTracer, logTracer}));
        logTracer.printf("original array = %s\n", Arrays.toString(array));
        chartTracer.set(array);
        Tracer.delay();

        for (int i = 0; i < N - 1; i++) {
            logTracer.printf("round %d\n", i + 1);
            int currMaxIdx = 0;
            int currMaxVal = array[i];
            for (int idx = 0; idx < N - i; idx++) {
                if (array[i + idx] > currMaxVal) {
                    currMaxIdx = idx;
                    currMaxVal = array[i + idx];
                }
            }
            if (currMaxIdx != 0) { // if currMaxIdx == 0 the max element is already at the bottom, no flip required
                logTracer.printf("flip at %d (step 1)\n", currMaxIdx + i);
                flip(currMaxIdx + i, N);
                logTracer.printf("flip at %d (step 2)\n", i);
                flip(i, N);
            }
        }

        logTracer.printf("sorted array = %s\n", Arrays.toString(array));
    }

    private static void flip(int start, int N) {
        chartTracer.select(start, N - 1);
        Tracer.delay();
        int idx = 0;
        for (int i = start; i < (start + N) / 2; i++) {
            chartTracer.select(i);
            Tracer.delay();
            int temp = array[i];
            array[i] = array[N - idx - 1];
            array[N - idx - 1] = temp;
            chartTracer.patch(i, array[i]);
            chartTracer.patch(N - idx - 1, array[N - idx - 1]);
            Tracer.delay();
            chartTracer.depatch(i);
            chartTracer.depatch(N - idx - 1);
            chartTracer.deselect(i);
            idx++;
        }
        chartTracer.deselect(start, N - 1);
    }
}
