import org.algorithm_visualizer.*;
import java.util.ArrayList;

class Main {

    private static Integer[][] G = {
        {0, 0, 1, 1, 0, 0},
        {1, 0, 0, 0, 0, 0},
        {0, 1, 0, 0, 0, 0},
        {0, 0, 0, 1, 0, 0},
        {0, 0, 0, 0, 0, 1},
        {0, 0, 0, 0, 1, 0},
    };

    private static int[] disc = new int[G.length];
    private static int[] low = new int[G.length];
    private static boolean[] stackMember = new boolean[G.length];
    private static ArrayList<Integer> st = new ArrayList<>();
    private static int time = 0;

    private static GraphTracer graphTracer = new GraphTracer();
    private static Array1DTracer discTracer = new Array1DTracer("Disc");
    private static Array1DTracer lowTracer = new Array1DTracer("Low");
    private static Array1DTracer stackMemberTracer = new Array1DTracer("stackMember");
    private static Array1DTracer stTracer = new Array1DTracer("st");
    private static LogTracer logger = new LogTracer();

    public static void main(String[] args) {
        for (int i = 0; i < G.length; i++) {
            disc[i] = -1;
            low[i] = -1;
            stackMember[i] = false;
        }

        graphTracer.set(G);
        Layout.setRoot(new VerticalLayout(new Commander[]{graphTracer, discTracer, lowTracer, stackMemberTracer, stTracer, logger}));
        discTracer.set(disc);
        lowTracer.set(low);
        stackMemberTracer.set(stackMember);
        stTracer.set(st);
        Tracer.delay();

        for (int i = 0; i < G.length; i++) {
            if (disc[i] == -1) {
                SCCVertex(i);
            }
        }
    }

    private static void SCCVertex(int u) {
        graphTracer.visit(u);
        Tracer.delay();

        disc[u] = ++time;
        discTracer.patch(u, time);
        Tracer.delay();

        low[u] = time;
        lowTracer.patch(u, time);
        Tracer.delay();

        st.add(u);
        stTracer.set(st);
        Tracer.delay();

        stackMember[u] = true;
        stackMemberTracer.patch(u, true);
        Tracer.delay();

        // Go through all vertices adjacent to this
        for (int v = 0; v < G[u].length; v++) {
            if (G[u][v] != 0) {
                // If v is not visited yet, then recur for it
                if (disc[v] == -1) {
                    SCCVertex(v);

                    // Check if the subtree rooted with 'v' has a
                    // connection to one of the ancestors of 'u'
                    low[u] = Math.min(low[u], low[v]);
                    lowTracer.patch(u, low[u]);
                    Tracer.delay();
                }
                // Update low value of 'u' only of 'v' is still in stack
                // (i.e. it's a back edge, not cross edge).
                else if (stackMember[v]) {
                    low[u] = Math.min(low[u], disc[v]);
                    lowTracer.patch(u, low[u]);
                    Tracer.delay();
                }
            }
        }

        // head node found, pop the stack and print an SCC
        int w = 0; // To store stack extracted vertices
        if (low[u] == disc[u]) {
            while (st.get(st.size() - 1) != u) {
                w = st.remove(st.size() - 1);
                stTracer.set(st);
                Tracer.delay();

                logger.printf("%d\n", w);
                Tracer.delay();

                stackMember[w] = false;
                stackMemberTracer.patch(w, false);
                Tracer.delay();
            }

            w = st.remove(st.size() - 1);
            stTracer.set(st);
            Tracer.delay();

            logger.printf("%d\n", w);
            Tracer.delay();
            logger.println("------");

            stackMember[w] = false;
            stackMemberTracer.patch(w, false);
            Tracer.delay();
        }
    }
}
