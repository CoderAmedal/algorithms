import org.algorithm_visualizer.*;
import java.util.ArrayList;
import java.util.Arrays;

class Main {

    private static Array1DTracer tracer1 = new Array1DTracer("Array");

    private static Array2DTracer tracer2 = new Array2DTracer("Holes");

    private static LogTracer logTracer = new LogTracer("Console");

    public static void main(String[] args) {
        Integer[] A = (Integer[]) new Randomize.Array1D(7, new Randomize.Integer(1, 9)).create();
        int N = A.length;
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer1, tracer2, logTracer}));
        tracer1.set(A);
        Tracer.delay();

        int min = A[0];
        int max = A[0];
        for (int i = 1; i < N; i++) {
            if (A[i] < min) min = A[i];
            if (A[i] > max) max = A[i];
        }
        int range = max - min + 1;

        ArrayList<ArrayList<Integer>> holes = new ArrayList<>();
        for (int i = 0; i < range; i++) {
            holes.add(new ArrayList<>());
        }
        tracer2.set(holes);

        logTracer.println("Filling up holes");
        for (int i = 0; i < N; i++) {
            tracer1.select(i);
            Tracer.delay();

            holes.get(A[i] - min).add(A[i]);

            tracer2.set(holes);
            tracer1.deselect(i);
        }

        logTracer.println("Building sorted array");
        int k = 0;
        for (int i = 0; i < range; i++) {
            for (int j = 0; j < holes.get(i).size(); j++) {
                tracer2.select(i, j);
                Tracer.delay();

                A[k++] = holes.get(i).get(j);

                tracer1.patch(k - 1, A[k - 1]);
                Tracer.delay();
                tracer2.deselect(i, j);
                tracer1.depatch(k - 1);
            }
        }

        logTracer.println("Sorted array is " + Arrays.toString(A));
    }
}
