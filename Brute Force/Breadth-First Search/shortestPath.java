import org.algorithm_visualizer.*;
import java.util.ArrayList;

class Main {

    private static GraphTracer tracer = new GraphTracer();

    private static LogTracer logger = new LogTracer();

    private static Integer[][] G;

    private static final int MAX_VALUE = 0x7fffffff;

    private static int s;

    private static int e;

    public static void main(String[] args) {
        tracer.directed(false);
        tracer.weighted();
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.log(logger);
        G = (Integer[][]) new Randomize.Graph(5, 1, new Randomize.Integer(1, 9)).directed(false).weighted(true).create();
        tracer.set(G);
        Tracer.delay();

        s = new Randomize.Integer(0, G.length - 1).create(); // s = start node
        e = s; // e = start node
        while (e == s) {
            e = new Randomize.Integer(0, G.length - 1).create();
        }
        logger.println("finding the shortest path from " + s + " to " + e);
        int minWeight = BFS(s);
        if (minWeight == MAX_VALUE) {
            logger.println("there is no path from " + s + " to " + e);
        } else {
            logger.println("the shortest path from " + s + " to " + e + " is " + minWeight);
        }
    }

    private static int BFS(int s) {
        int[] W = new int[G.length]; // W[i] indicates the length of the shortest path from start node to the i-th node
        ArrayList<Integer> Q = new ArrayList<>();
        for (int i = 0; i < G.length; i++) {
            W[i] = MAX_VALUE;
            tracer.updateNode(i, MAX_VALUE);
        }
        W[s] = 0;
        Q.add(s); // add start node to queue
        tracer.visit(s, null, 0);
        Tracer.delay();
        while (!Q.isEmpty()) {
            int node = Q.remove(0); // dequeue
            for (int i = 0; i < G[node].length; i++) {
                if (G[node][i] != 0) { // if the edge from current node to the i-th node exists
                    if (W[i] > W[node] + G[node][i]) { // if current path is shorter than the previously shortest path
                        W[i] = W[node] + G[node][i]; // update the length of the shortest path
                        Q.add(i); // add child node to queue
                        tracer.visit(i, node, W[i]);
                        Tracer.delay();
                    }
                }
            }
        }
        return W[e];
    }
}
