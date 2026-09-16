import org.algorithm_visualizer.*;
import java.util.ArrayList;

class Main {

    private static GraphTracer graphTracer = new GraphTracer();

    private static Array1DTracer visitedTracer = new Array1DTracer("visited");

    private static LogTracer logger = new LogTracer();

    private static Integer[][] G;

    public static void main(String[] args) {
        graphTracer.directed(false);
        Layout.setRoot(new VerticalLayout(new Commander[]{graphTracer, visitedTracer, logger}));
        graphTracer.log(logger);
        G = (Integer[][]) new Randomize.Graph(8, 0.3, new Randomize.Integer(1, 9)).directed(false).create();
        graphTracer.set(G);
        Tracer.delay();

        boolean[] visited = DFSExplore(G, 0);
        boolean check = true;
        for (int i = 0; i < visited.length; i++) check &= visited[i];
        if (check) {
            logger.println("The Graph is CONNECTED");
        } else {
            logger.println("The Graph is NOT CONNECTED");
        }
    }

    private static boolean[] DFSExplore(Integer[][] graph, int source) {
        ArrayList<int[]> stack = new ArrayList<>();
        stack.add(new int[]{source, -1});
        boolean[] visited = new boolean[graph.length];
        visitedTracer.set(visited);

        while (!stack.isEmpty()) {
            int[] temp = stack.remove(stack.size() - 1);
            int node = temp[0];
            int prev = temp[1];

            if (!visited[node]) {
                visited[node] = true;
                visitedTracer.patch(node, visited[node]);

                if (prev != -1 && graph[node][prev] != 0) {
                    graphTracer.visit(node, prev);
                    Tracer.delay();
                } else {
                    graphTracer.visit(node);
                    Tracer.delay();
                }

                for (int i = 0; i < graph.length; i++) {
                    if (graph[node][i] != 0) {
                        stack.add(new int[]{i, node});
                    }
                }
            }
        }

        return visited;
    }
}
