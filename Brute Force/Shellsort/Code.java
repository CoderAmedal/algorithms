import org.algorithm_visualizer.*;
import java.util.Arrays;

class Main {

    private static ChartTracer chartTracer = new ChartTracer();

    private static LogTracer logTracer = new LogTracer("Console");

    private static Integer[] array = (Integer[]) new Randomize.Array1D(15, new Randomize.Integer(1, 20)).create();

    public static void main(String[] args) {
        int length = array.length;
        Layout.setRoot(new VerticalLayout(new Commander[]{chartTracer, logTracer}));
        logTracer.printf("Original array = %s\n", Arrays.toString(array));
        chartTracer.set(array);
        Tracer.delay();

        for (int gap = length / 2; gap > 0; gap /= 2) {
            logTracer.printf("\nGap of %d\n", gap);
            for (int i = gap; i < length; i++) {
                chartTracer.select(i);
                chartTracer.select(i - gap);
                Tracer.delay();
                int k = array[i];
                logTracer.printf("Holding: %d\n", k);
                int j = i;
                for (; j >= gap && k < array[j - gap]; j -= gap) {
                    logTracer.printf("%d < %d\n", k, array[j - gap]);
                    array[j] = array[j - gap];
                    chartTracer.patch(j, array[j]);
                    Tracer.delay();
                    chartTracer.depatch(j);
                }
                int old = array[j];
                array[j] = k;
                if (old != k) {
                    chartTracer.patch(j, array[j]);
                    Tracer.delay();
                    chartTracer.depatch(j);
                    logTracer.printf("Swapped %d with %d\n", array[j], old);
                }
                chartTracer.deselect(i);
                chartTracer.deselect(i - gap);
            }
        }

        logTracer.printf("\nSorted array = %s\n", Arrays.toString(array));
    }
}
