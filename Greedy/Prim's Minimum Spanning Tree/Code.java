import org.algorithm_visualizer.*;

class Main {

    private static GraphTracer tracer = new GraphTracer();

    private static LogTracer logger = new LogTracer();

    private static Integer[][] G;

    public static void main(String[] args) {
        G = (Integer[][]) new Randomize.Graph(10, .4, new Randomize.Integer(1, 9)).directed(false).weighted(true).create();
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.directed(false);
        tracer.weighted();
        tracer.log(logger);
        tracer.set(G);
        Tracer.delay();

        logger.printf("nodes that belong to minimum spanning tree are: \n");
        prim();
    }

    private static void prim() {
        // Finds a tree so that there exists a path between
        // every two nodes while keeping the cost minimal
        int sum = 0;
        int[] D = new int[G.length];
        D[0] = 1; // First node is visited
        for (int k = 0; k < G.length - 1; k++) { // Searching for k edges
            int minD = Integer.MAX_VALUE;
            int minI = 0;
            int minJ = 0;
            for (int i = 0; i < G.length; i++) {
                if (D[i] != 0) { // First node in an edge must be visited
                    for (int j = 0; j < G.length; j++) {
                        if (D[j] == 0 && G[i][j] != 0) {
                            tracer.visit(i, j);
                            Tracer.delay();
                            // Second node must not be visited and must be connected to first node
                            if (G[i][j] < minD) {
                                // Searching for cheapest edge which satisfies requirements
                                minD = G[i][j];
                                minI = i;
                                minJ = j;
                            }
                            tracer.leave(i, j);
                            Tracer.delay();
                        }
                    }
                }
            }
            tracer.visit(minI, minJ);
            Tracer.delay();
            D[minJ] = 1; // Visit second node and insert it into or tree
            sum += G[minI][minJ];
        }
        logger.printf("The sum of all edges is: %d\n", sum);
    }
}
