import org.algorithm_visualizer.*;

class Main {

    private static GraphTracer tracer = new GraphTracer();

    private static LogTracer logger = new LogTracer();

    private static Integer[][] G;

    private static boolean[] D;

    private static final long MAX_VALUE = Long.MAX_VALUE;

    private static long minWeight;

    private static int e;

    public static void main(String[] args) {
        tracer.directed(false);
        tracer.weighted();
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.log(logger);
        G = (Integer[][]) new Randomize.Graph(5, 1, new Randomize.Integer(1, 9)).directed(false).weighted(true).create();
        tracer.set(G);
        Tracer.delay();

        int s = new Randomize.Integer(0, G.length - 1).create(); // s = start node
        e = s; // e = end node
        while (e == s) {
            e = new Randomize.Integer(0, G.length - 1).create();
        }
        minWeight = MAX_VALUE;
        logger.println("finding the shortest path from " + s + " to " + e);
        D = new boolean[G.length]; // D[i] indicates whether the i-th node is discovered or not
        DFS(s, null, 0);
        if (minWeight == MAX_VALUE) {
            logger.println("there is no path from " + s + " to " + e);
        } else {
            logger.println("the shortest path from " + s + " to " + e + " is " + minWeight);
        }
    }

    private static void DFS(int node, Integer parent, long weight) { // node = current node, parent = previous node
        if (minWeight < weight) return;
        if (node == e) {
            tracer.visit(node, parent, weight);
            Tracer.delay();
            if (minWeight > weight) {
                minWeight = weight;
            }
            tracer.leave(node, parent, minWeight);
            Tracer.delay();
            return;
        }
        D[node] = true; // label current node as discovered
        tracer.visit(node, parent, weight);
        Tracer.delay();
        for (int i = 0; i < G[node].length; i++) {
            if (G[node][i] != 0) { // if the path from current node to the i-th node exists
                if (!D[i]) { // if the i-th node is not labeled as discovered
                    DFS(i, node, weight + G[node][i]); // recursively call DFS
                }
            }
        }
        D[node] = false; // label current node as undiscovered
        tracer.leave(node, parent, 0);
        Tracer.delay();
    }
}
