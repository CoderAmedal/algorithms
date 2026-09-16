// import visualization libraries {
import org.algorithm_visualizer.*;
import java.util.*;
// }

class Main {

    // G[i][j] indicates whether the path from the i-th node to the j-th node exists or not. NOTE: The graph must be Directed-Acyclic
    private static Integer[][] G = {
        {0, 0, 0, 0, 0, 0},
        {0, 0, 1, 0, 0, 0},
        {0, 0, 0, 1, 0, 0},
        {0, 0, 0, 0, 0, 0},
        {1, 0, 0, 1, 0, 0},
        {1, 1, 0, 0, 0, 0},
    };

    private static GraphTracer tracer = new GraphTracer();

    private static LogTracer logger = new LogTracer();

    public static void main(String[] args) {
        // define tracer variables {
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.log(logger);
        tracer.set(G);
        Tracer.delay();
        // }

        int[] inDegrees = new int[G.length]; // create an Array of G.length number of 0s
        ArrayList<Integer> Q = new ArrayList<>();
        int iter = 0;

        // logger {
        logger.printf("Calculating in-degrees for each Node...\n");
        // }

        for (int currNode = 0; currNode < G.length; currNode++) {
            for (int currNodeNeighbor = 0; currNodeNeighbor < G.length; currNodeNeighbor++) {
                if (G[currNode][currNodeNeighbor] != 0) {
                    // visualize {
                    logger.printf("%d has an incoming edge from %d\n", currNodeNeighbor, currNode);
                    tracer.visit(currNodeNeighbor, currNode);
                    Tracer.delay();
                    // }
                    inDegrees[currNodeNeighbor]++;
                    // visualize {
                    tracer.leave(currNodeNeighbor, currNode);
                    Tracer.delay();
                    // }
                }
            }
        }
        // logger {
        logger.printf("Done. In-Degrees are: [ %s ]\n", Arrays.toString(inDegrees));
        logger.printf("\n");

        logger.printf("Initializing queue with all the sources (nodes with no incoming edges)\n");
        // }
        for (int node = 0; node < inDegrees.length; node++) {
            // visualize {
            tracer.visit(node);
            Tracer.delay();
            // }
            if (inDegrees[node] == 0) {
                // logger {
                logger.printf("%d is a source\n", node);
                // }
                Q.add(node);
            }
            // visualize {
            tracer.leave(node);
            Tracer.delay();
            // }
        }
        // logger {
        logger.printf("Done. Initial State of Queue: [ %s ]\n", Q);
        logger.printf("\n");
        // }

        // begin topological sort (kahn)
        while (!Q.isEmpty()) {
            // logger {
            logger.printf("Iteration #%d. Queue state: [ %s ]\n", iter, Q);
            // }
            int currNode = Q.remove(0);
            // visualize {
            tracer.visit(currNode);
            Tracer.delay();
            // }

            for (int i = 0; i < G.length; i++) {
                if (G[currNode][i] != 0) {
                    // visualize {
                    logger.printf("%d has an incoming edge from %d. Decrementing %d's in-degree by 1.\n", i, currNode, i);
                    tracer.visit(i, currNode);
                    Tracer.delay();
                    // }
                    inDegrees[i]--;
                    // visualize {
                    tracer.leave(i, currNode);
                    Tracer.delay();
                    // }

                    if (inDegrees[i] == 0) {
                        // logger {
                        logger.printf("%d's in-degree is now 0. Enqueuing %d\n", i, i);
                        // }
                        Q.add(i);
                    }
                }
            }
            // visualize {
            tracer.leave(currNode);
            Tracer.delay();
            // }
            // logger {
            logger.printf("In-degrees are: [%s ]\n", Arrays.toString(inDegrees));
            logger.printf("-------------------------------------------------------------------\n");
            // }

            iter++;
        }
    }
}
