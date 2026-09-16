import org.algorithm_visualizer.*;

class Main {

    private static GraphTracer tracer = new GraphTracer();

    private static LogTracer logger = new LogTracer();

    private static Integer[][] G;

    private static final int MAX_VALUE = Integer.MAX_VALUE / 4;

    public static void main(String[] args) {
        G = (Integer[][]) new Randomize.Graph(5, 1, new Randomize.Integer()).weighted(true).create();

        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.weighted();
        tracer.log(logger);
        tracer.set(G);
        Tracer.delay();

        logger.println("finding the shortest paths from and to all nodes");
        floydWarshall();
    }

    private static void floydWarshall() {
        // Finds the shortest path between all nodes
        int[][] S = new int[G.length][G.length];
        for (int i = 0; i < G.length; i++) {
            for (int j = 0; j < G.length; j++) {
                // Distance to self is always 0
                if (i == j) S[i][i] = 0;
                // Distance between connected nodes is their weight
                else if (G[i][j] > 0) S[i][j] = G[i][j];
                // Else we don't know the distance and we set it to infinity
                else S[i][j] = MAX_VALUE;
            }
        }
        // If there is a shorter path using k, use it instead
        for (int k = 0; k < G.length; k++) {
            for (int i = 0; i < G.length; i++) {
                if (k == i) continue;
                tracer.visit(k, i);
                Tracer.delay();
                for (int j = 0; j < G.length; j++) {
                    if (i == j || j == k) continue;
                    tracer.visit(j, k);
                    Tracer.delay();
                    if (S[i][j] > S[i][k] + S[k][j]) {
                        tracer.visit(j, i, S[i][j]);
                        Tracer.delay();
                        S[i][j] = S[i][k] + S[k][j];
                        tracer.leave(j, i, S[i][j]);
                    }
                    tracer.leave(j, k);
                }
                tracer.leave(k, i);
                Tracer.delay();
            }
        }
        for (int i = 0; i < G.length; i++) {
            for (int j = 0; j < G.length; j++) {
                if (S[i][j] == MAX_VALUE) logger.printf("there is no path from %d to %d\n", i, j);
                else logger.printf("the shortest path from %d to %d is %d\n", i, j, S[i][j]);
            }
        }
    }
}
