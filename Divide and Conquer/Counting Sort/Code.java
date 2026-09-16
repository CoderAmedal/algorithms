import org.algorithm_visualizer.*;

class Main {

    private static Array1DTracer arrayTracer = new Array1DTracer("Array");

    private static Array1DTracer countsTracer = new Array1DTracer("Counts");

    private static Array1DTracer sortedArrayTracer = new Array1DTracer("Sorted Array");

    public static void main(String[] args) {
        int N = 20; // the size of an array
        Integer[] array = (Integer[]) new Randomize.Array1D(N, new Randomize.Integer(0, 9)).create();
        Layout.setRoot(new VerticalLayout(new Commander[]{arrayTracer, countsTracer, sortedArrayTracer}));

        // find the maximum value that will decide the size of counts array
        int max = array[0];
        for (int value : array) {
            if (value > max) max = value;
        }
        int[] counts = new int[max + 1];
        arrayTracer.set(array);
        countsTracer.set(counts);
        Tracer.delay();

        // store counts of each number
        for (int i = 0; i < N; i++) {
            int number = array[i];
            counts[number]++;
            arrayTracer.select(i);
            countsTracer.patch(number, counts[number]);
            Tracer.delay();
            countsTracer.depatch(number);
            arrayTracer.deselect(i);
        }

        // calculate the prefix sums
        for (int i = 1; i <= max; i++) {
            counts[i] += counts[i - 1];
            countsTracer.select(i - 1);
            countsTracer.patch(i, counts[i]);
            Tracer.delay();
            countsTracer.depatch(i);
            countsTracer.deselect(i - 1);
        }

        // create a sorted array based on the prefix sums
        Integer[] sortedArray = new Integer[N];
        sortedArrayTracer.set(sortedArray);
        for (int i = N - 1; i >= 0; i--) {
            int number = array[i];
            int count = counts[number];
            sortedArray[count - 1] = number;
            counts[number]--;
            arrayTracer.select(i);
            countsTracer.select(number);
            sortedArrayTracer.patch(count - 1, sortedArray[count - 1]);
            countsTracer.patch(number, counts[number]);
            Tracer.delay();
            sortedArrayTracer.depatch(count - 1);
            countsTracer.depatch(number);
            countsTracer.deselect(number);
            arrayTracer.deselect(i);
        }
    }
}
