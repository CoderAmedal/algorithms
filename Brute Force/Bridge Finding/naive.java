import org.algorithm_visualizer.*;
import java.util.ArrayList;

class Main {

    private static GraphTracer tracer = new GraphTracer();

    private static LogTracer logger = new LogTracer();

    private static Integer[][] G = {
        {0, 1, 0, 0, 0, 0},
        {1, 0, 0, 1, 1, 0},
        {0, 0, 0, 1, 0, 0},
        {0, 1, 1, 0, 1, 1},
        {0, 1, 0, 1, 0, 0},
        {0, 0, 0, 1, 0, 0},
    };

    public static void main(String[] args) {
        tracer.directed(false);
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.set(G);
        Tracer.delay();

        ArrayList<int[]> bridges = findBridges(G);

        logger.println("The bridges are: ");
        for (int i = 0; i < bridges.size(); i++) {
            logger.println(bridges.get(i)[0] + " to " + bridges.get(i)[1]);
        }
        logger.println("NOTE: A bridge is both ways, i.e., from A to B and from B to A, because this is an Undirected Graph");
    }

    // Depth First Search Exploration Algorithm to test connectedness of the Graph (see Graph Algorithms/DFS/exploration), without the tracer & logger commands
    private static boolean[] DFSExplore(Integer[][] graph, int source) {
        ArrayList<int[]> stack = new ArrayList<>();
        stack.add(new int[]{source, -1});
        boolean[] visited = new boolean[graph.length];

        while (!stack.isEmpty()) {
            int[] temp = stack.remove(stack.size() - 1);
            int node = temp[0];

            if (!visited[node]) {
                visited[node] = true;

                for (int i = 0; i < graph.length; i++) {
                    if (graph[node][i] != 0) {
                        stack.add(new int[]{i, node});
                    }
                }
            }
        }

        return visited;
    }

    private static ArrayList<int[]> findBridges(Integer[][] graph) {
        ArrayList<int[]> bridges = new ArrayList<>();

        for (int i = 0; i < graph.length; i++) {
            for (int j = 0; j < graph.length; j++) {
                if (graph[i][j] != 0) { // check if an edge exists
                    logger.println("Deleting edge " + i + "->" + j + " and calling DFSExplore ()");
                    tracer.visit(j, i);
                    Tracer.delay();
                    tracer.leave(j, i);
                    Tracer.delay();

                    Integer[][] tempGraph = new Integer[graph.length][graph.length];
                    for (int r = 0; r < graph.length; r++) tempGraph[r] = graph[r].clone();
                    tempGraph[i][j] = 0;
                    tempGraph[j][i] = 0;
                    boolean[] visited = DFSExplore(tempGraph, 0);

                    int count = 0;
                    for (boolean v : visited) if (v) count++;
                    if (count == graph.length) {
                        logger.println("Graph is CONNECTED. Edge is NOT a bridge");
                    } else {
                        logger.println("Graph is DISCONNECTED. Edge IS a bridge");
                        bridges.add(new int[]{i, j});
                    }
                }
            }
        }

        return bridges;
    }
}
