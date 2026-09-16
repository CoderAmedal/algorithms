// import visualization libraries {
import org.algorithm_visualizer.*;
import java.util.ArrayList;
import java.util.List;
// }

class Main {

    private static Integer[][] A = {{2, 3}, {3, 4}};

    private static Integer[][] B = {{1, 0}, {1, 2}};

    private static Integer[][] C = {{6, 5}, {8, 7}};

    // define tracer variables {
    private static Array2DTracer matrixATracer = new Array2DTracer("Matrix A");

    private static Array2DTracer matrixBTracer = new Array2DTracer("Matrix B");

    private static Array2DTracer matrixCTracer = new Array2DTracer("Matrix C");

    private static Array1DTracer randomVectorTracer = new Array1DTracer("Random Vector");

    private static Array1DTracer resultVectorTracer = new Array1DTracer("Result Vector");

    private static LogTracer logger = new LogTracer();
    // }

    public static void main(String[] args) {
        Layout.setRoot(new VerticalLayout(new Commander[]{
                matrixATracer, matrixBTracer, matrixCTracer, randomVectorTracer, resultVectorTracer, logger}));
        matrixATracer.set(A);
        matrixBTracer.set(B);
        matrixCTracer.set(C);
        Tracer.delay();

        FreivaldsAlgorithm();
    }

    private static boolean FreivaldsAlgorithm() {
        int k = 5;
        int n = A.length;

        while (k-- > 0) {
            // logger {
            logger.printf("Iterations remained: #%d\n", k);
            // }

            // Generate random vector
            List<Integer> r = new ArrayList<>();
            List<Integer> P = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                P.add(-1);
                r.add((Math.random() < 0.5) ? 1 : 0);
            }
            // visualize {
            randomVectorTracer.set(r);
            Tracer.delay();
            // }

            // Compute Br, Cr
            List<Integer> Br = new ArrayList<>();
            List<Integer> Cr = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                int tmpB = 0;
                int tmpC = 0;
                for (int j = 0; j < n; j++) {
                    tmpB += r.get(j) * B[j][i];
                    tmpC += r.get(j) * C[j][i];
                }
                Br.add(tmpB);
                Cr.add(tmpC);
            }

            // Compute A * Br - Cr
            P = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                int tmp = 0;
                for (int j = 0; j < n; j++) {
                    tmp += (A[i][j] * Br.get(i)) - Cr.get(i);
                }
                P.add(tmp);
            }
            // visualize {
            resultVectorTracer.set(P);
            Tracer.delay();
            // }

            for (int i = 0; i < n; i++) {
                if (P.get(i) != 0) {
                    // logger {
                    logger.printf("P[%d] !== 0 (%d), exit\n", i, P.get(i));
                    // }
                    return false;
                }
            }

            // logger {
            logger.println("Result vector is identity, continue...");
            // }
        }

        return true;
    }
}
