// import visualization libraries {
import org.algorithm_visualizer.*;
import java.util.*;
// }

class Main {

    private static GraphTracer graphTracer = new GraphTracer(" BST - Elements marked red indicates the current status of tree ");

    private static Array1DTracer elemTracer = new Array1DTracer(" Elements ");

    private static LogTracer logger = new LogTracer(" Log ");

    private static Integer[] elements = {5, 8, 10, 3, 1, 6, 9, 7, 2, 0, 4}; // item to be inserted

    private static HashMap<Integer, int[]> T = new HashMap<>();

    public static void main(String[] args) {
        // define tracer variables {
        Layout.setRoot(new VerticalLayout(new Commander[]{graphTracer, elemTracer, logger}));
        elemTracer.set(elements);
        graphTracer.log(logger);
        Tracer.delay();
        // }

        int root = elements[0]; // take first element as root
        T.put(root, new int[]{-1, -1});
        // visualize {
        graphTracer.addNode(root);
        graphTracer.layoutTree(root, true);
        logger.printf("%d Inserted as root of tree \n", root);
        // }

        for (int i = 1; i < elements.length; i++) {
            // visualize {
            elemTracer.select(i);
            Tracer.delay();
            // }
            bstInsert(root, elements[i], -1); // insert ith element
            // visualize {
            elemTracer.deselect(i);
            Tracer.delay();
            // }
        }

        int key = elements[new Randomize.Integer(0, elements.length - 1).create()]; // item to be searched

        // logger {
        logger.printf("Finding number %d\n", key);
        // }
        bst(key, root, -1); // node with key root is the root
    }

    // root = current node , parent = previous node
    private static void bstInsert(int root, int element, int parent) {
        // visualize {
        if (parent == -1) graphTracer.visit(root);
        else graphTracer.visit(root, parent);
        Tracer.delay();
        // }

        String propName = "";
        if (element < root) propName = "left";
        else if (element > root) propName = "right";

        if (!propName.equals("")) {
            int[] treeNode = T.get(root);
            int child = propName.equals("left") ? treeNode[0] : treeNode[1];
            if (child == -1) { // insert as child of root
                if (propName.equals("left")) treeNode[0] = element;
                else treeNode[1] = element;
                T.put(element, new int[]{-1, -1});
                // visualize {
                graphTracer.addNode(element);
                graphTracer.addEdge(root, element);
                graphTracer.select(element, root);
                Tracer.delay();
                graphTracer.deselect(element, root);
                logger.printf("%d Inserted\n", element);
                // }
            } else {
                bstInsert(child, element, root);
            }
        }
        // visualize {
        if (parent == -1) graphTracer.leave(root);
        else graphTracer.leave(root, parent);
        Tracer.delay();
        // }
    }

    // node = current node , parent = previous node
    private static void bst(int item, int node, int parent) {
        // visualize {
        if (parent == -1) graphTracer.visit(node);
        else graphTracer.visit(node, parent);
        Tracer.delay();
        // }

        if (item == node) { // key found
            // logger {
            logger.printf(" Match Found \n");
            // }
        } else if (item < node) { // key less than value of current node
            int[] treeNode = T.get(node);
            if (treeNode[0] == -1) {
                // logger {
                logger.printf(" Not Found \n");
                // }
            } else {
                bst(item, treeNode[0], node);
            }
        } else { // key greater than value of current node
            int[] treeNode = T.get(node);
            if (treeNode[1] == -1) {
                // logger {
                logger.printf(" Not Found \n");
                // }
            } else {
                bst(item, treeNode[1], node);
            }
        }
    }
}
