import org.algorithm_visualizer.*;
import java.util.Arrays;

class Main {

    private static GraphTracer tracer = new GraphTracer();

    private static LogTracer logger = new LogTracer();

    private static Integer[][] G;

    private static final int MAX_VALUE = 0x7fffffff;

    public static void main(String[] args) {
        G = (Integer[][]) new Randomize.Graph(5, 0.5, new Randomize.Integer(-2, 5)).weighted(true).create();

        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.weighted();
        tracer.log(logger);
        tracer.set(G);
        Tracer.delay();

        int src = new Randomize.Integer(0, G.length - 1).create();
        int dest;
        do {
            dest = new Randomize.Integer(0, G.length - 1).create();
        } while (src == dest);

        logger.printf("finding the shortest path from %d to %d\n", src, dest);

        int minWeight = bellmanFord(src, dest);

        if (minWeight == MAX_VALUE) {
            logger.printf("there is no path from %d to %d\n", src, dest);
        } else {
            logger.printf("the shortest path from %d to %d is %d\n", src, dest, minWeight);
        }
    }

    private static int bellmanFord(int src, int dest) {
        int[] weights = new int[G.length];
        int i;
        int j;

        for (i = 0; i < G.length; i++) {
            weights[i] = MAX_VALUE;
            tracer.updateNode(i, weights[i]);
        }
        weights[src] = 0;
        tracer.updateNode(src, 0);

        logger.printf("Initializing weights to: %s\n", Arrays.toString(weights));
        logger.println("");

        // begin BF algorithm execution
        int k = G.length;
        while (k-- > 0) {
            logger.printf("Iteration: %d\n", G.length - k);
            logger.println("------------------------------------------------------------------");

            for (i = 0; i < G.length; i++) {
                for (j = 0; j < G.length; j++) {
                    if (G[i][j] != 0) { // proceed to relax Edges only if a particular weight != 0 (0 represents no edge)
                        if (weights[j] > (weights[i] + G[i][j])) {
                            weights[j] = weights[i] + G[i][j];
                            logger.printf("weights[%d] = weights[%d] + %d\n", j, i, G[i][j]);
                        }
                        tracer.visit(j, i, weights[j]);
                        Tracer.delay();
                        tracer.leave(j, i);
                        Tracer.delay();
                    }
                }
            }

            logger.printf("updated weights: %s\n", Arrays.toString(weights));
            logger.println("");
        }

        // check for cycle
        logger.println("checking for cycle");
        for (i = 0; i < G.length; i++) {
            for (j = 0; j < G.length; j++) {
                if (G[i][j] != 0) {
                    if (weights[j] > (weights[i] + G[i][j])) {
                        logger.printf("A cycle was detected: weights[%d] > weights[%d] + %d\n", j, i, G[i][j]);
                        return MAX_VALUE;
                    }
                }
            }
        }

        logger.printf("No cycles detected. Final weights for the source %d are: %s\n", src, Arrays.toString(weights));

        return weights[dest];
    }
}
