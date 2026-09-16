import org.algorithm_visualizer.*;

class Main {

    private static GraphTracer treeTracer = new GraphTracer(" Traversal Pre-order ");

    private static LogTracer logger = new LogTracer(" Log ");

    private static Integer[][] G = {
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
        {0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
        {0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0},
        {1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1},
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0},
    };

    private static int[][] T = {
        {-1, -1},
        {-1, 7},
        {-1, -1},
        {6, 1},
        {-1, -1},
        {3, 8},
        {0, 2},
        {-1, -1},
        {10, 4},
        {-1, -1},
        {9, -1},
    };

    public static void main(String[] args) {
        Layout.setRoot(new VerticalLayout(new Commander[]{treeTracer, logger}));
        treeTracer.set(G);
        treeTracer.layoutTree(5);
        Tracer.delay();

        int a = 7;
        int b = 2;
        Integer result = lcaBT(null, 5, a, b);
        logger.printf("Lowest common ancestor of %d & %d is: %s\n", a, b, result == null ? "null" : result.toString());
    }

    private static Integer lcaBT(Integer parent, int root, int a, int b) {
        logger.printf("Beginning new Iteration of lcaBT () with parent: %s, current root: %d\n",
            parent == null ? "null" : parent.toString(), root);
        if (root == -1) {
            logger.println("Reached end of path & target node(s) not found");
            return null;
        }

        if (parent != null) treeTracer.visit(root, parent);
        else treeTracer.visit(root);
        Tracer.delay();

        if (root == a || root == b) return root;

        Integer left = lcaBT(root, T[root][0], a, b);
        Integer right = lcaBT(root, T[root][1], a, b);

        if (left != null && right != null) return root;
        if (left == null && right == null) {
            treeTracer.leave(root, parent);
            Tracer.delay();
        }

        return left != null ? left : right;
    }
}
