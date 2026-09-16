import org.algorithm_visualizer.*;

class Main {

    private static Array2DTracer tracer = new Array2DTracer();

    private static LogTracer logger = new LogTracer();

    private static int[][] D;

    public static void main(String[] args) {
        int integer = new Randomize.Integer(5, 14).create();

        D = new int[integer + 1][integer + 1];
        String A = "";
        for (int i = 0; i <= integer; i++) {
            D[i][0] = 1;
        }

        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.set(D);
        Tracer.delay();

        logger.println("Partitioning: " + integer);
        partition(A, integer, integer);
        int part = integerPartition(integer);
        logger.println(part);
    }

    private static void partition(String A, int n, int p) {
        if (p == 0) {
            logger.println("[" + String.join(", ", A.split("")) + "]");
        } else {
            if (n > 1) partition(A, n - 1, p);
            if (n <= p) partition(n + A, n, p - n);
        }
    }

    private static int integerPartition(int n) {
        // cycle through each cell of matrix
        for (int i = 1; i <= n; i++) {
            for (int j = 1; j <= n; j++) {
                if (i > j) {
                    tracer.select(i, j);
                    Tracer.delay();
                    // set cell to cell above it
                    D[i][j] = D[i - 1][j];
                    tracer.patch(i, j, D[i][j]);
                    Tracer.delay();
                    tracer.depatch(i, j);
                    tracer.deselect(i, j);
                } else {
                    tracer.select(i, j);
                    Tracer.delay();
                    // grab above cell and add it to previous cell
                    int above = D[i - 1][j];
                    int left = D[i][j - i];
                    D[i][j] = above + left;
                    tracer.patch(i, j, D[i][j]);
                    Tracer.delay();
                    tracer.depatch(i, j);
                    tracer.deselect(i, j);
                }
            }
        }
        return D[n][n];
    }
}
