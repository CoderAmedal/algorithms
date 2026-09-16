import org.algorithm_visualizer.*;

class Main {

    private static Array1DTracer tracer = new Array1DTracer("Ugly Numbers");

    private static Array1DTracer tracer2 = new Array1DTracer("Multiples of 2, 3, 5");

    private static Array1DTracer tracer3 = new Array1DTracer(" Iterators I0, I1, I2 ");

    private static LogTracer logger = new LogTracer();

    public static void main(String[] args) {
        int N = 15;
        int[] A = new int[N];
        A[0] = 1; // By convention 1 is an ugly number

        int[] M = {2, 3, 5}; // multiples of 2, 3, 5 respectively
        int[] I = {0, 0, 0}; // iterators of 2, 3, 5 respectively

        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, tracer2, tracer3, logger}));
        tracer.set(A);
        tracer2.set(M);
        tracer3.set(I);
        Tracer.delay();

        for (int i = 1; i < N; i++) {
            // next is minimum of m2, m3 and m5
            int next = (M[0] <= M[1]) ? (M[0] <= M[2] ? M[0] : M[2]) : (M[1] <= M[2] ? M[1] : M[2]);
            logger.printf(" Minimum of %d, %d, %d : %d\n", M[0], M[1], M[2], next);
            A[i] = next;

            tracer.patch(i, A[i]);
            Tracer.delay();
            tracer.depatch(i);

            if (next == M[0]) {
                I[0]++;
                M[0] = A[I[0]] * 2;
                tracer2.patch(0, M[0]);
                Tracer.delay();
                tracer3.patch(0, I[0]);
                Tracer.delay();
                tracer2.depatch(0);
                tracer3.depatch(0);
            }
            if (next == M[1]) {
                I[1]++;
                M[1] = A[I[1]] * 3;
                tracer2.patch(1, M[1]);
                Tracer.delay();
                tracer3.patch(1, I[1]);
                Tracer.delay();
                tracer2.depatch(1);
                tracer3.depatch(1);
            }
            if (next == M[2]) {
                I[2]++;
                M[2] = A[I[2]] * 5;
                tracer2.patch(2, M[2]);
                Tracer.delay();
                tracer3.patch(2, I[2]);
                Tracer.delay();
                tracer2.depatch(2);
                tracer3.depatch(2);
            }
        }
    }
}
