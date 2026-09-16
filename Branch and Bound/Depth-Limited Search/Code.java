// import visualization libraries {
import org.algorithm_visualizer.*;
import java.util.*;
// }

class Main {

    // G[i][j] indicates whether the path from the i-th node to the j-th node exists or not
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

    private static GraphTracer tracer = new GraphTracer();

    private static LogTracer logger = new LogTracer();

    public static void main(String[] args) {
        // define tracer variables {
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.log(logger);
        tracer.set(G);
        tracer.layoutTree(0);
        Tracer.delay();
        // }

        // logger {
        logger.printf("Number of descendant is %d\n", DLSCount(2, 0, -1));
        // }
    }

    // This is a sample DLS applications where
    // we try to find number of descendant of root within some depth
    // node = current node, parent = previous node
    private static int DLSCount(int limit, int node, int parent) {
        // visualize {
        if (parent == -1) tracer.visit(node);
        else tracer.visit(node, parent);
        Tracer.delay();
        // }

        int child = 0;
        if (limit > 0) { // cut off the search
            for (int i = 0; i < G[node].length; i++) {
                if (G[node][i] != 0) { // if current node has the i-th node as a child
                    child += 1 + DLSCount(limit - 1, i, node); // recursively call DLS
                }
            }
            return child;
        }
        return child;
    }
}
