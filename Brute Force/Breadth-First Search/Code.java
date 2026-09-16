import org.algorithm_visualizer.*;
import java.util.ArrayList;

class Main {

    private static GraphTracer tracer = new GraphTracer();

    private static LogTracer logger = new LogTracer();

    private static Integer[][] G = {
        {0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0},
        {0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1},
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    };

    public static void main(String[] args) {
        tracer.log(logger);
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.set(G);
        tracer.layoutTree(0);
        Tracer.delay();

        BFS(0);
    }

    private static void BFS(int s) { // s = start node
        ArrayList<Integer> Q = new ArrayList<>();
        Q.add(s); // add start node to queue
        tracer.visit(s);
        Tracer.delay();
        while (!Q.isEmpty()) {
            int node = Q.remove(0); // dequeue
            for (int i = 0; i < G[node].length; i++) {
                if (G[node][i] != 0) { // if current node has the i-th node as a child
                    Q.add(i); // add child node to queue
                    tracer.visit(i, node);
                    Tracer.delay();
                }
            }
        }
    }
}
