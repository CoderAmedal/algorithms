import org.algorithm_visualizer.*;

class Main {

    private static GraphTracer tracer = new GraphTracer();

    private static Array1DTracer tracerS = new Array1DTracer();

    private static LogTracer logger = new LogTracer();

    private static Integer[][] G;

    private static final int MAX_VALUE = Integer.MAX_VALUE;

    private static int[] S;

    public static void main(String[] args) {
        G = (Integer[][]) new Randomize.Graph(5, 1, new Randomize.Integer(1, 9)).directed(false).weighted(true).create();
        S = new int[G.length];
        for (int i = 0; i < G.length; i++) {
            S[i] = MAX_VALUE;
        }

        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, tracerS, logger}));
        tracer.directed(false);
        tracer.weighted();
        tracer.log(logger);
        tracer.set(G);
        tracerS.set(S);
        Tracer.delay();

        int s = new Randomize.Integer(0, G.length - 1).create(); // s = start node
        int e; // e = end node
        do {
            e = new Randomize.Integer(0, G.length - 1).create();
        } while (s == e);
        logger.printf("finding the shortest path from %d to %d\n", s, e);
        Tracer.delay();
        dijkstra(s, e);
    }

    private static void dijkstra(int start, int end) {
        boolean[] D = new boolean[G.length]; // D[i] indicates whether the i-th node is discovered or not
        S[start] = 0; // Starting node is at distance 0 from itself
        tracerS.patch(start, S[start]);
        Tracer.delay();
        tracerS.depatch(start);
        tracerS.select(start);
        int k = G.length;
        while (k-- > 0) {
            // Finding a node with the shortest distance from S[minIndex]
            int minDistance = MAX_VALUE;
            int minIndex = -1;
            for (int i = 0; i < G.length; i++) {
                if (S[i] < minDistance && !D[i]) {
                    minDistance = S[i];
                    minIndex = i;
                }
            }
            if (minDistance == MAX_VALUE) break; // If there is no edge from current node, jump out of loop
            D[minIndex] = true;
            tracerS.select(minIndex);
            tracer.visit(minIndex);
            Tracer.delay();
            // For every unvisited neighbour of current node, we check
            // whether the path to it is shorter if going over the current node
            for (int i = 0; i < G.length; i++) {
                if (G[minIndex][i] != 0 && S[i] > S[minIndex] + G[minIndex][i]) {
                    S[i] = S[minIndex] + G[minIndex][i];
                    tracerS.patch(i, S[i]);
                    tracer.visit(i, minIndex, S[i]);
                    Tracer.delay();
                    tracerS.depatch(i);
                    tracer.leave(i, minIndex);
                    Tracer.delay();
                }
            }
            tracer.leave(minIndex);
            Tracer.delay();
        }
        if (S[end] == MAX_VALUE) {
            logger.printf("there is no path from %d to %d\n", start, end);
        } else {
            logger.printf("the shortest path from %d to %d is %d\n", start, end, S[end]);
        }
    }
}
