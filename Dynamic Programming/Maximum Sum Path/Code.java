import org.algorithm_visualizer.*;

class Main {

    private static Array2DTracer dataViewer = new Array2DTracer();

    private static Array2DTracer tracer = new Array2DTracer("Results Table");

    private static LogTracer logger = new LogTracer();

    private static Integer[][] D = (Integer[][]) new Randomize.Array2D(5, 5, new Randomize.Integer(1, 5)).create();

    private static Integer[][] DP;

    public static void main(String[] args) {
        Layout.setRoot(new VerticalLayout(new Commander[]{dataViewer, tracer, logger}));
        dataViewer.set(D);
        DP = new Integer[D.length][D[0].length];
        for (int i = 0; i < D.length; i++) {
            for (int j = 0; j < D[i].length; j++) {
                DP[i][j] = Integer.MAX_VALUE;
            }
        }
        tracer.set(DP);
        Tracer.delay();

        int N = DP.length;
        int M = DP[0].length;

        for (int i = 0; i < N; i++) {
            for (int j = 0; j < M; j++) {
                if (i == 0 && j == 0) {
                    update(i, j, D[i][j]);
                } else if (i == 0) {
                    tracer.select(i, j - 1);
                    update(i, j, DP[i][j - 1] + D[i][j]);
                    tracer.deselect(i, j - 1);
                } else if (j == 0) {
                    tracer.select(i - 1, j);
                    update(i, j, DP[i - 1][j] + D[i][j]);
                    tracer.deselect(i - 1, j);
                } else {
                    tracer.select(i, j - 1);
                    tracer.select(i - 1, j);
                    update(i, j, Math.max(DP[i][j - 1], DP[i - 1][j]) + D[i][j]);
                    tracer.deselect(i, j - 1);
                    tracer.deselect(i - 1, j);
                }
            }
        }
        logger.printf("max = %d\n", DP[N - 1][M - 1]);
    }

    private static void update(int i, int j, int value) {
        DP[i][j] = value;
        dataViewer.select(i, j);
        Tracer.delay();
        tracer.patch(i, j, DP[i][j]);
        Tracer.delay();
        tracer.depatch(i, j);
        dataViewer.deselect(i, j);
    }
}
