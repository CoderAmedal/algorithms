import org.algorithm_visualizer.*;

class Main {

    private static Array2DTracer tracer = new Array2DTracer();

    private static String[][] G = {
        {"#", "#", "#", "#", "#", "#", "#", "#", "#"},
        {"#", "-", "-", "-", "#", "-", "-", "-", "#"},
        {"#", "-", "-", "-", "#", "-", "-", "-", "#"},
        {"#", "-", "-", "#", "-", "-", "-", "-", "#"},
        {"#", "#", "#", "-", "-", "-", "#", "#", "#"},
        {"#", "-", "-", "-", "-", "#", "-", "-", "#"},
        {"#", "-", "-", "-", "#", "-", "-", "-", "#"},
        {"#", "-", "-", "-", "#", "-", "-", "-", "#"},
        {"#", "#", "#", "#", "#", "#", "#", "#", "#"},
    };

    public static void main(String[] args) {
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer}));
        tracer.set(G);
        Tracer.delay();

        floodFill(4, 4, "-", "a");
    }

    private static void floodFill(int i, int j, String oldColor, String newColor) {
        if (i < 0 || i >= G.length || j < 0 || j >= G[i].length) return;
        if (!G[i][j].equals(oldColor)) return;

        // set the color of node to newColor
        G[i][j] = newColor;

        tracer.select(i, j);
        Tracer.delay();
        tracer.patch(i, j, G[i][j]);
        Tracer.delay();

        // next step four-way
        floodFill(i + 1, j, oldColor, newColor);
        floodFill(i - 1, j, oldColor, newColor);
        floodFill(i, j + 1, oldColor, newColor);
        floodFill(i, j - 1, oldColor, newColor);
    }
}
