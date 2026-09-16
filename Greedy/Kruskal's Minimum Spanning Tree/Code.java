import org.algorithm_visualizer.*;
import java.util.ArrayList;
import java.util.Collections;
import java.util.LinkedHashMap;

class Main {

    private static GraphTracer tracer = new GraphTracer();

    private static LogTracer logger = new LogTracer();

    private static Integer[][] G;

    public static void main(String[] args) {
        G = (Integer[][]) new Randomize.Graph(5, 1, new Randomize.Integer(1, 9)).directed(false).weighted(true).create();
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.directed(false);
        tracer.weighted();
        tracer.set(G);
        Tracer.delay();
        kruskal();
    }

    private static void kruskal() {
        int vcount = G.length;

        // Preprocess: sort edges by weight.
        ArrayList<int[]> edges = new ArrayList<>();
        for (int vi = 0; vi < vcount - 1; vi++) {
            for (int vj = vi + 1; vj < vcount; vj++) {
                edges.add(new int[]{vi, vj, G[vi][vj]});
            }
        }
        Collections.sort(edges, (a, b) -> a[2] - b[2]);

        // Give each vertex a tree to decide if they are already in the same tree.
        @SuppressWarnings("unchecked")
        LinkedHashMap<Integer, Integer>[] t = new LinkedHashMap[vcount];
        for (int i = 0; i < vcount; i++) {
            t[i] = new LinkedHashMap<>();
            t[i].put(i, 1);
        }

        int wsum = 0;
        int n = 0;
        int index = 0;
        while (n < vcount - 1 && index < edges.size()) {
            int[] e = edges.get(index++); // Get the edge of min weight
            tracer.visit(e[0], e[1]);
            Tracer.delay();
            if (t[e[0]] == t[e[1]]) {
                // e[0] & e[1] already in the same tree, ignore
                tracer.leave(e[0], e[1]);
                Tracer.delay();
                continue;
            }

            // Choose the current edge.
            wsum += e[2];

            // Merge tree of e[0] & e[1]
            LinkedHashMap<Integer, Integer> merged = new LinkedHashMap<>(t[e[0]]);
            merged.putAll(t[e[1]]);
            for (Integer key : merged.keySet()) {
                t[key] = merged;
            }

            n += 1;
        }

        logger.printf("The sum of all edges is: %d\n", wsum);
    }
}
