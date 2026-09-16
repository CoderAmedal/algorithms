import org.algorithm_visualizer.*;

class Main {

    private static Array1DTracer tracer = new Array1DTracer("Sequence");

    public static void main(String[] args) {
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer}));
        int index = 15;
        long[] D = new long[index];
        D[0] = 1;
        D[1] = 1;
        tracer.set(D);
        Tracer.delay();

        for (int i = 2; i < index; i++) {
            D[i] = D[i - 2] + D[i - 1];
            tracer.select(i - 2, i - 1);
            Tracer.delay();
            tracer.patch(i, D[i]);
            Tracer.delay();
            tracer.depatch(i);
            tracer.deselect(i - 2, i - 1);
        }
    }
}
