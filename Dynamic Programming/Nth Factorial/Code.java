import org.algorithm_visualizer.*;

class Main {

    private static Array1DTracer tracer = new Array1DTracer("Sequence");

    public static void main(String[] args) {
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer}));
        int index = 15;
        long[] D = new long[index];
        D[0] = 1;
        tracer.set(D);
        Tracer.delay();

        for (int i = 1; i < index; i++) {
            D[i] = D[i - 1] * i;
            tracer.select(i - 1);
            Tracer.delay();
            tracer.patch(i, D[i]);
            Tracer.delay();
            tracer.depatch(i);
            tracer.deselect(i - 1);
        }
    }
}
