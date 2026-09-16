import org.algorithm_visualizer.*;
import java.util.Arrays;

class Main {

    private static Array2DTracer tracer = new Array2DTracer();

    private static LogTracer logger = new LogTracer();

    private static Integer[] k = (Integer[]) new Randomize.Array1D(10, new Randomize.Integer(1, 999)).create();

    private static Integer[][] D = new Integer[3][];

    public static void main(String[] args) {
        D[0] = k;
        D[1] = new Integer[10];
        D[2] = new Integer[10];
        for (int i = 0; i < 10; i++) {
            D[1][i] = 0;
            D[2][i] = 0;
        }
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.set(D);
        Tracer.delay();

        logger.printf("original array = %s\n", Arrays.toString(D[0]));

        for (int exp = 0; exp < 3; exp++) {
            logger.printf("Digit: %d\n", exp);
            for (int i = 0; i < D[0].length; i++) {
                int d = digit(i, exp);
                tracer.select(0, i);
                Tracer.delay();
                D[2][d] += 1;
                tracer.patch(2, d, D[2][d]);
                Tracer.delay();
                tracer.depatch(2, d);
                tracer.deselect(0, i);
            }
            for (int i = 1; i < 10; i++) {
                tracer.select(2, i - 1);
                Tracer.delay();
                D[2][i] += D[2][i - 1];
                tracer.patch(2, i, D[2][i]);
                Tracer.delay();
                tracer.depatch(2, i);
                tracer.deselect(2, i - 1);
            }
            for (int i = D[0].length - 1; i >= 0; i--) {
                int d = digit(i, exp);
                tracer.select(0, i);
                Tracer.delay();
                D[2][d] -= 1;
                tracer.patch(2, d, D[2][d]);
                Tracer.delay();
                tracer.depatch(2, d);
                D[1][D[2][d]] = D[0][i];
                tracer.patch(1, D[2][d], D[1][D[2][d]]);
                Tracer.delay();
                tracer.depatch(1, D[2][d]);
                tracer.deselect(0, i);
            }
            for (int i = 0; i < D[0].length; i++) {
                tracer.select(1, i);
                Tracer.delay();
                D[0][i] = D[1][i];
                tracer.patch(0, i, D[0][i]);
                Tracer.delay();
                tracer.depatch(0, i);
                tracer.deselect(1, i);
            }
            for (int i = 0; i < 10; i++) {
                D[2][i] = 0;
                tracer.patch(2, i, D[2][i]);
                Tracer.delay();
                tracer.depatch(2, i);
            }
        }

        logger.printf("sorted array = %s\n", Arrays.toString(D[0]));
    }

    private static int digit(int i, int exp) {
        return (int) (D[0][i] / Math.pow(10, exp) % 10);
    }
}
