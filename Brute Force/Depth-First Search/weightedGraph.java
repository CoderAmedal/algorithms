import org.algorithm_visualizer.*;

class Main {

    private static GraphTracer tracer = new GraphTracer();

    private static LogTracer logger = new LogTracer();

    private static Integer[][] G;

    private static boolean[] D;

    public static void main(String[] args) {
        tracer.directed(false);
        tracer.weighted();
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.log(logger);
        G = (Integer[][]) new Randomize.Graph(5, 1, new Randomize.Integer(1, 9)).directed(false).weighted(true).create();
        tracer.set(G);
        Tracer.delay();

        for (int i = 0; i < G.length; i++) { // start from every node
            logger.println("start from " + i);
            D = new boolean[G.length];
            DFS(i, null, 0);
        }
    }

    private static void DFS(int node, Integer parent, int weight) { // node = current node, parent = previous node
        tracer.visit(node, parent, weight);
        Tracer.delay();
        D[node] = true; // label current node as discovered
        for (int i = 0; i < G[node].length; i++) {
            if (G[node][i] != 0) { // if the edge from current node to the i-th node exists
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
