import org.algorithm_visualizer.*;

class Main {

    private static final int gridSize = 10;

    private static final int generations = 4;

    private static final double fillChance = 0.55;

    private static String[][] G = new String[gridSize][gridSize];

    private static Array2DTracer tracer = new Array2DTracer();

    public static void main(String[] args) {
        for (int i = 0; i < gridSize; i++) {
            for (int j = 0; j < gridSize; j++) {
                if (Math.random() < fillChance || i == 0 || j == 0 || i == gridSize - 1 || j == gridSize - 1) {
                    G[i][j] = "#";
                } else {
                    G[i][j] = ".";
                }
            }
        }

        Layout.setRoot(new VerticalLayout(new Commander[]{tracer}));
        tracer.set(G);
        Tracer.delay();

        for (int gi = 0; gi < G.length; gi++) {
            for (int gj = 0; gj < G[gi].length; gj++) {
                if (G[gi][gj].equals("#")) {
                    tracer.patch(gi, gj, G[gi][gj]);
                }
            }
        }

        for (int iter = 0; iter < generations; iter++) {
            cellularAutomata("#", ".");
        }
    }

    private static void cellularAutomata(String fillShape, String emptyShape) {
        String[][] nextGrid = new String[G.length][];

        for (int i = 0; i < G.length; i++) {
            nextGrid[i] = new String[G[i].length];
            for (int j = 0; j < G[i].length; j++) {
                int adjCount = 0;
                int twoAwayCount = 0;
                for (int x = -2; x <= 2; x++) {
                    for (int y = -2; y <= 2; y++) {
                        if ((i + x >= 0 && i + x < G.length) && (j + y >= 0 && j + y < G[i].length)) {
                            if (!(x != 0 && y != 0) && G[i + x][j + y].equals(emptyShape)) {
                                if (x == -2 || x == 2 || y == -2 || y == 2) {
                                    twoAwayCount++;
                                } else {
                                    adjCount++;
                                }
                            }
                        }
                    }
                }
                if (adjCount >= 5) {
                    nextGrid[i][j] = fillShape;
                } else if (adjCount <= 1) {
                    if (twoAwayCount < 3) {
                        nextGrid[i][j] = fillShape;
                    } else {
                        nextGrid[i][j] = emptyShape;
                    }
                } else {
                    nextGrid[i][j] = emptyShape;
                }
            }
        }

        for (int i = 0; i < nextGrid.length; i++) {
            for (int j = 0; j < nextGrid[i].length; j++) {
                tracer.depatch(i, j);
                tracer.select(i, j);
                Tracer.delay();
                G[i][j] = nextGrid[i][j];
                if (G[i][j].equals(fillShape)) {
                    tracer.patch(i, j, G[i][j]);
                } else {
                    tracer.patch(i, j, G[i][j]);
                    tracer.depatch(i, j);
                    tracer.deselect(i, j);
                }
            }
        }
    }
}
