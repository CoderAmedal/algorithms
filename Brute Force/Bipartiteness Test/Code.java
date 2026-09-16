import org.algorithm_visualizer.*;
import java.util.ArrayList;

class Main {

    private static GraphTracer tracer = new GraphTracer();

    private static LogTracer logger = new LogTracer();

    private static Array1DTracer colorsTracer = new Array1DTracer("Colors");

    private static Integer[][] G = {
        {0, 1, 0, 1, 1},
        {1, 0, 1, 0, 0},
        {0, 1, 0, 1, 0},
        {1, 0, 1, 0, 0}, // <-- replace latest 0 with 1 to make G not biparted
        {1, 0, 0, 0, 0},
    };

    public static void main(String[] args) {
        tracer.directed(false);
        tracer.log(logger);
        tracer.set(G);
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger, colorsTracer}));
        Tracer.delay();

        bfsCheckBipartiteness(0);
    }

    private static boolean bfsCheckBipartiteness(int s) {
        ArrayList<Integer> Q = new ArrayList<>();

        // Create a new matrix to set colors (0,1)
        Integer[] Colors = new Integer[G.length];
        for (int i = 0; i < G.length; i++) Colors[i] = -1;
        colorsTracer.set(Colors);

        Colors[s] = 1;
        colorsTracer.patch(s, 1);

        Q.add(s); // add start node to queue

        while (!Q.isEmpty()) {
            int node = Q.remove(0); // dequeue
            tracer.visit(node);
            Tracer.delay();

            for (int i = 0; i < G[node].length; i++) {
                if (G[node][i] != 0) {
                    if (Colors[i] == -1) {
                        Colors[i] = 1 - Colors[node];
                        colorsTracer.patch(i, 1 - Colors[node]);

                        Q.add(i);
                        tracer.visit(i, node);
                        Tracer.delay();
                    } else if (Colors[i].equals(Colors[node])) {
                        logger.println("Graph is not biparted");
                        return false;
                    }
                }
            }
        }

        logger.println("Graph is biparted");
        return true;
    }
}
