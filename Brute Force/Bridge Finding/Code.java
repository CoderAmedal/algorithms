import org.algorithm_visualizer.*;
import java.util.ArrayList;
import java.util.Arrays;

class Main {

    private static GraphTracer graphTracer = new GraphTracer();

    private static LogTracer logger = new LogTracer();

    private static Integer[][] G = {
        {0, 1, 0, 0, 1, 0},
        {1, 0, 0, 0, 1, 0},
        {0, 0, 0, 1, 0, 0},
        {0, 0, 1, 0, 1, 1},
        {1, 1, 0, 1, 0, 0},
        {0, 0, 0, 1, 0, 0},
    };

    /*
      NOTE: Code assumes NO parallel edges
    */

    private static int timer = 0; // adj keeps track of the neighbors of each node

    private static ArrayList<int[]> bridges = new ArrayList<>();
    private static ArrayList<ArrayList<Integer>> adj = new ArrayList<>();

    public static void main(String[] args) {
        graphTracer.directed(false);
        Layout.setRoot(new VerticalLayout(new Commander[]{graphTracer, logger}));
        graphTracer.set(G);
        Tracer.delay();

        findBridges(G);

        logger.println("There are " + bridges.size() + " bridges in the Graph");
        for (int i = 0; i < bridges.size(); i++) {
            logger.println(bridges.get(i)[0] + "-->" + bridges.get(i)[1]);
        }
        logger.println("NOTE: All bridges are both ways (just like in the Naive Algorithm) because the Graph is undirected. So, edge A->B and B->A, both are bridges");
    }

    private static void trace(int v, int u) {
        graphTracer.visit(v, u);
        Tracer.delay();
        graphTracer.leave(v, u);
        Tracer.delay();
    }

    private static void util(int u, int[] disc, int[] low, int parent) {
        // u is the node that is currently being processed in the DFS (depth-first search)
        // disc is the numbering of the vertices in the DFS, starting at 0
        // low[v] is the lowest numbered vertex that can be reached from vertex v along the DFS
        // parent is the node that u came from
        logger.println("");
        logger.println("Visiting node " + u);
        graphTracer.visit(u);
        Tracer.delay();
        graphTracer.leave(u);
        Tracer.delay();

        disc[u] = low[u] = timer++;

        logger.println("Nodes adjacent to " + u + " are: [ " + adj.get(u) + " ]");

        for (int v : adj.get(u)) {
            if (disc[v] > -1 && v == parent) {
                trace(v, u);
                logger.println(u + "'s neighbor " + v + " is u's parent. Not visiting it.");
            } else if (disc[v] > -1 && v != parent) {
                trace(v, u);
                logger.println(u + "'s neighbor " + v + " is not u's parent. Comparing low[u] with disc[v]");
                if (low[u] > disc[v]) {
                    logger.println("low[" + u + "] is greater than disc[" + v + "]. Setting low[" + u + "] to disc[" + v + "]");
                    low[u] = disc[v];
                }
            }

            if (disc[v] == -1) {
                trace(v, u);
                logger.println(u + "'s neighbor " + v + " has not been visited yet");
                logger.println("recursively calling util (" + v + ", [" + Arrays.toString(disc) + "], [" + Arrays.toString(low) + "]," + u + ")");
                util(v, disc, low, u);

                logger.println("--------------------------------------------------------------------");
                logger.println("Setting low [" + u + "] to " + Math.min(low[u], low[v]));
                low[u] = Math.min(low[u], low[v]);

                if (low[v] == disc[v]) {
                    logger.println("low [" + v + "] === disc [" + v + "], low[" + v + "]=" + low[v] + ", disc[" + v + "]=" + disc[v]);
                    logger.println(u + " -> " + v + " is a bridge. Adding " + u + "->" + v + "to the set of bridges found");
                    bridges.add(new int[]{u, v});
                }
            }
        }
    }

    private static void findBridges(Integer[][] graph) {
        int[] disc = new int[graph.length];
        int[] low = new int[graph.length];
        for (int i = 0; i < graph.length; i++) {
            disc[i] = -1;
            low[i] = -1;
        }

        // PRECOMPUTATION: store every node's neighbor info in auxiliary array for efficient retrieval later
        for (int i = 0; i < graph.length; i++) {
            ArrayList<Integer> temp = new ArrayList<>();
            for (int j = 0; j < graph[i].length; j++) {
                if (graph[i][j] != 0) temp.add(j);
            }
            adj.add(temp);
        }

        logger.println("Initializing: <b>disc</b>: " + Arrays.toString(disc) + " <b>low</b>: " + Arrays.toString(low));
        logger.println("");
        logger.println("Beginning efficient Bridge Finding");
        logger.println("NOTE: call to util () follows pattern: util (nodeToVisit, disc, low, parent). See code for clarity");
        logger.println("");

        logger.println("Starting the main for loop (for each node)");
        for (int v = 0; v < graph.length; v++) {
            if (disc[v] == -1) {
                logger.println(v + " has not been visited yet. Calling util (" + v + ",  [" + Arrays.toString(disc) + "], [" + Arrays.toString(low) + "]," + v + ") from the for loop");
                util(v, disc, low, v);
                logger.println("Returned in for loop after util (" + v + ", [" + Arrays.toString(disc) + "], [" + Arrays.toString(low) + "], [" + v + "])");
            }
        }
    }
}
