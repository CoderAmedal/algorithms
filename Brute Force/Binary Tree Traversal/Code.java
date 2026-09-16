import org.algorithm_visualizer.*;
import java.util.Arrays;

class Main {

    private static GraphTracer treeTracer = new GraphTracer("Traversal Pre-order");

    private static Array1DTracer arrayTracer = new Array1DTracer("Print Pre-order");

    private static LogTracer logger = new LogTracer("Log");

    private static Integer[][] G = {
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
        {1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
        {0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
        {0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1},
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0},
    };

    private static int[][] T = {
        {-1, -1},
        {0, 2},
        {-1, -1},
        {1, 4},
        {-1, -1},
        {3, 8},
        {-1, 7},
        {-1, -1},
        {6, 10},
        {-1, -1},
        {9, -1},
    };

    private static int index = 0;

    public static void main(String[] args) {
        Layout.setRoot(new VerticalLayout(new Commander[]{treeTracer, arrayTracer, logger}));
        treeTracer.set(G);
        treeTracer.layoutTree(5);
        String[] dashes = new String[T.length];
        Arrays.fill(dashes, "-");
        arrayTracer.set(dashes);
        Tracer.delay();

        preOrder(5, null);
        logger.println("Finished");
    }

    private static void preOrder(int root, Integer parent) {
        if (root == -1) {
            logger.println("No more nodes. Backtracking.");
            Tracer.delay();
            return;
        }

        logger.println("Reached " + root);
        treeTracer.visit(root, parent);
        Tracer.delay();

        logger.println("Printing " + root);
        treeTracer.leave(root);
        arrayTracer.patch(index++, root);
        Tracer.delay();

        logger.println(" Going left from " + root);
        Tracer.delay();
        preOrder(T[root][0], root);

        logger.println(" Going right from " + root);
        Tracer.delay();
        preOrder(T[root][1], root);
    }
}
