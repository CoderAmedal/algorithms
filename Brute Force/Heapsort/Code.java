import org.algorithm_visualizer.*;
import java.util.Arrays;

class Main {

    private static ChartTracer chartTracer = new ChartTracer();

    private static LogTracer logTracer = new LogTracer("Console");

    private static Integer[] array = (Integer[]) new Randomize.Array1D(10, new Randomize.Integer(1, 20)).create();

    public static void main(String[] args) {
        Layout.setRoot(new VerticalLayout(new Commander[]{chartTracer, logTracer}));
        logTracer.printf("Original array = %s\n", Arrays.toString(array));
        chartTracer.set(array);
        Tracer.delay();

        heapSort(array, array.length);

        logTracer.printf("Final array = %s\n", Arrays.toString(array));
    }

    private static void heapSort(Integer[] array, int size) {
        for (int i = size / 2 - 1; i >= 0; i--) {
            heapify(array, size, i);
        }

        for (int j = size - 1; j >= 0; j--) {
            int temp = array[0];
            array[0] = array[j];
            array[j] = temp;

            chartTracer.patch(0, array[0]);
            chartTracer.patch(j, array[j]);
            logTracer.printf("Swapping elements : %d & %d\n", array[0], array[j]);
            Tracer.delay();
            chartTracer.depatch(0);
            chartTracer.depatch(j);
            chartTracer.select(j);
            Tracer.delay();

            heapify(array, j, 0);

            chartTracer.deselect(j);
        }
    }

    private static void heapify(Integer[] array, int size, int root) {
        int largest = root;
        int left = 2 * root + 1;
        int right = 2 * root + 2;

        if (left < size && array[left] > array[largest]) {
            largest = left;
        }

        if (right < size && array[right] > array[largest]) {
            largest = right;
        }

        if (largest != root) {
            int temp = array[root];
            array[root] = array[largest];
            array[largest] = temp;

            chartTracer.patch(root, array[root]);
            chartTracer.patch(largest, array[largest]);
            logTracer.printf("Swapping elements : %d & %d\n", array[root], array[largest]);
            Tracer.delay();
            chartTracer.depatch(root);
            chartTracer.depatch(largest);

            heapify(array, size, largest);
        }
    }
}
