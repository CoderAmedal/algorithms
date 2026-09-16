import org.algorithm_visualizer.*;
import java.util.ArrayList;

class Main {

    private static ChartTracer chartTracer = new ChartTracer("Chart");

    private static Array1DTracer arrayTracer = new Array1DTracer("Array");

    private static Array2DTracer bucketsTracer = new Array2DTracer("Buckets");

    public static void main(String[] args) {
        int N = 25; // the size of an array
        int K = 5; // the number of buckets
        Integer[] array = (Integer[]) new Randomize.Array1D(N, new Randomize.Integer(0, 999)).create();
        Layout.setRoot(new VerticalLayout(new Commander[]{chartTracer, arrayTracer, bucketsTracer}));

        // create K buckets
        ArrayList<ArrayList<Integer>> buckets = new ArrayList<>();
        for (int i = 0; i < K; i++) {
            buckets.add(new ArrayList<>());
        }

        arrayTracer.chart(chartTracer);
        arrayTracer.set(array);
        bucketsTracer.set(buckets);
        Tracer.delay();

        // find the maximum value that will be used for distribution
        int max = array[0];
        for (int value : array) {
            if (value > max) max = value;
        }

        // distribute the elements into the buckets
        for (int i = 0; i < N; i++) {
            int number = array[i];
            int bucketIndex = (int) Math.floor((double) number / (max + 1) * K);
            ArrayList<Integer> bucket = buckets.get(bucketIndex);
            bucket.add(number);
            arrayTracer.select(i);
            bucketsTracer.patch(bucketIndex, bucket.size() - 1, number);
            Tracer.delay();
            bucketsTracer.depatch(bucketIndex, bucket.size() - 1);

            // insertion sort within the bucket
            int j = bucket.size() - 1;
            while (j > 0 && bucket.get(j - 1) > bucket.get(j)) {
                int temp = bucket.get(j - 1);
                bucket.set(j - 1, bucket.get(j));
                bucket.set(j, temp);
                bucketsTracer.patch(bucketIndex, j - 1, bucket.get(j - 1));
                bucketsTracer.patch(bucketIndex, j, bucket.get(j));
                Tracer.delay();
                bucketsTracer.depatch(bucketIndex, j - 1);
                bucketsTracer.depatch(bucketIndex, j);
                j--;
            }
            arrayTracer.deselect(i);
        }

        // concatenate the buckets back into the array
        int i = 0;
        for (int bucketIndex = 0; bucketIndex < K; bucketIndex++) {
            ArrayList<Integer> bucket = buckets.get(bucketIndex);
            for (int j = 0; j < bucket.size(); j++) {
                array[i] = bucket.get(j);
                arrayTracer.patch(i, array[i]);
                bucketsTracer.select(bucketIndex, j);
                Tracer.delay();
                bucketsTracer.deselect(bucketIndex, j);
                arrayTracer.depatch(i);
                i++;
            }
        }
    }
}
