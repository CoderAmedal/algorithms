import org.algorithm_visualizer.*;

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

        DFS(0, null);
    }

    private static void DFS(int node, Integer parent) { // node = current node, parent = previous node
        tracer.visit(node, parent);
        Tracer.delay();
        for (int i = 0; i < G[node].length; i++) {
            if (G[node][i] != 0) { // if current node has the i-th node as a child
                DFS(i, node); // recursively call DFS
            }
        }
    }
}
