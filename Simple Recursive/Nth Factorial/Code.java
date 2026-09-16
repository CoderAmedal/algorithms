import org.algorithm_visualizer.*;

class Main {

    private static Array1DTracer tracer = new Array1DTracer("Sequence");

    private static long[] D;

    private static int index = 15;

    public static void main(String[] args) {
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer}));
        D = new long[index];
        D[0] = 1;
        tracer.set(D);
        Tracer.delay();

        fact(index);
    }

    private static long fact(int num) {
        if (num < 0) {
            return 0;
        }

        if (num == 0) {
            return 1;
        }

        long res = num * fact(num - 1);

        D[num - 1] = res;

        tracer.select(num - 1);
        Tracer.delay();
        tracer.patch(num - 1, D[num - 1]);
        Tracer.delay();
        tracer.depatch(num - 1);
        tracer.deselect(num - 1);

        return res;
    }
}
