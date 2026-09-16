import org.algorithm_visualizer.*;
import java.util.ArrayList;

class Main {

    private static GraphTracer graphTracer = new GraphTracer("Web Page inter-connections");

    private static Array1DTracer rankTracer = new Array1DTracer("Web Page Ranks");

    private static Array1DTracer oecTracer = new Array1DTracer("Outgoing Edge Counts");

    private static Array2DTracer inTracer = new Array2DTracer("Incoming Nodes");

    private static LogTracer logger = new LogTracer();

    private static Integer[][] G;

    private static Double[] ranks;

    private static int[] outgoingEdgeCounts;

    private static ArrayList<ArrayList<Integer>> incomingNodes;

    public static void main(String[] args) {
        G = (Integer[][]) new Randomize.Graph(5, 0.4, new Randomize.Integer(1, 9)).directed(true).weighted(false).create();
        outgoingEdgeCounts = new int[G.length];
        incomingNodes = new ArrayList<>();
        for (int i = 0; i < G.length; i++) {
            ArrayList<Integer> row = new ArrayList<>();
            for (int j = 0; j < G.length; j++) row.add(-1);
            incomingNodes.add(row);
        }

        Layout.setRoot(new VerticalLayout(new Commander[]{graphTracer, rankTracer, oecTracer, inTracer, logger}));
        graphTracer.set(G);
        oecTracer.set(outgoingEdgeCounts);
        inTracer.set(incomingNodes);
        Tracer.delay();

        // PRECOMPUTATIONS

        logger.println("Calculate Outgoing Edge Count for each Node");
        for (int i = 0; i < G.length; i++) {
            outgoingEdgeCounts[i] = arraySum(G[i]);
            showOutgoingEdges(i);

            oecTracer.patch(i, outgoingEdgeCounts[i]);
            Tracer.delay();
            oecTracer.depatch(i);
            Tracer.delay();
        }

        logger.println("determine incoming nodes for each node");
        for (int i = 0; i < G.length; i++) {
            for (int j = 0; j < G.length; j++) {
                if (G[i][j] != 0) {
                    // there's an edge FROM i TO j
                    graphTracer.visit(j, i);
                    Tracer.delay();

                    int nextPos = incomingNodes.get(j).indexOf(-1);
                    incomingNodes.get(j).set(nextPos, i);
                    inTracer.patch(j, nextPos, i);
                    Tracer.delay();
                    inTracer.depatch(j, nextPos);
                    Tracer.delay();

                    graphTracer.leave(j, i);
                    Tracer.delay();
                }
            }
        }

        // All -1s will be removed from incoming node records, they are irrelevant
        for (ArrayList<Integer> arr : incomingNodes) {
            int idx = arr.indexOf(-1);
            if (idx == -1) idx = arr.size() - 1;
            while (arr.size() > idx) arr.remove(arr.size() - 1);
        }

        double damping = 0.85;
        int iterations = 7;
        double initialRank = 1.0;

        logger.println("Initialized all Page ranks to " + initialRank);
        ranks = new Double[G.length];
        for (int i = 0; i < G.length; i++) ranks[i] = initialRank;
        rankTracer.set(ranks);
        logger.println("Begin execution of PageRank Version #1");
        logger.println("Equation used: PR (X) = (1 - D) + D (In-Node-Summation i->X (PR (I) / Out (i)))");
        logger.println("D = Damping Factor, PR (X) = Page rank of Node X, i = the ith In-Node of X, Out (i) = outgoing Edge Count of i");
        logger.println("");

        while (iterations-- > 0) {
            for (int node = 0; node < ranks.length; node++) {
                ranks[node] = updateRank(node, damping);
                rankTracer.patch(node, ranks[node]);
                Tracer.delay();
                rankTracer.patch(node);
                Tracer.delay();
            }
        }

        logger.println("Page Ranks have been converged to.");
        for (int node = 0; node < ranks.length; node++) {
            logger.println("Rank of Node #" + node + " = " + ranks[node]);
        }
        logger.println("Done");
    }

    private static int arraySum(Integer[] array) {
        int sum = 0;
        for (Integer curr : array) {
            if (curr != null && curr != 0) sum += 1;
        }
        return sum;
    }

    private static void showOutgoingEdges(int i) {
        for (int j = 0; j < G[i].length; j++) {
            if (G[i][j] != 0) {
                graphTracer.visit(j, i);
                Tracer.delay();
                graphTracer.leave(j, i);
                Tracer.delay();
            }
        }
    }

    private static double updateRank(int nodeIndex, double damping) {
        double inNodeSummation = 0;

        logger.println("Updating rank of " + nodeIndex);
        logger.println("The incoming Nodes of " + nodeIndex + " are being highlighted");

        ArrayList<Integer> inNodes = incomingNodes.get(nodeIndex);
        for (int i = 0; i < inNodes.size(); i++) {
            int incoming = inNodes.get(i);
            inTracer.select(nodeIndex, i);
            Tracer.delay();
            logger.println("Outgoing edge count of " + incoming + " is " + outgoingEdgeCounts[incoming]);
            oecTracer.select(incoming);
            Tracer.delay();

            inNodeSummation += (ranks[incoming] / outgoingEdgeCounts[incoming]);

            oecTracer.deselect(incoming);
            Tracer.delay();
            inTracer.deselect(nodeIndex, i);
            Tracer.delay();
        }
        logger.println("In-Node summation of " + nodeIndex + " = " + inNodeSummation);

        double result = ((1 - damping) / G.length) + (damping * inNodeSummation);
        logger.println("Therefore, using Equation, new rank of " + nodeIndex + " = " + result);
        return result;
    }
}
