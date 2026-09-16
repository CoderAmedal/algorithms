// import visualization libraries {
import org.algorithm_visualizer.*;
import java.util.ArrayList;
import java.util.List;
// }

class Main {

    private static final int n = 6; // rows (change these!)
    private static final int m = 6; // columns (change these!)

    private static final int hEnd = m * 4 - (m - 1);
    private static final int vEnd = n * 3 - (n - 1);

    private static String[][] G = new String[vEnd][hEnd];

    // define tracer variables {
    private static Array2DTracer tracer = new Array2DTracer();

    private static LogTracer logger = new LogTracer();
    // }

    public static void main(String[] args) {
        for (int i = 0; i < vEnd; i++) { // by row
            for (int j = 0; j < hEnd; j++) { // by column
                G[i][j] = " ";

                if (i == 0 && j == 0) { // top-left corner
                    G[i][j] = "\u250c";
                } else if (i == 0 && j == hEnd - 1) { // top-right corner
                    G[i][j] = "\u2510";
                } else if (i == vEnd - 1 && j == 0) { // bottom-left corner
                    G[i][j] = "\u2514";
                } else if (i == vEnd - 1 && j == hEnd - 1) { // bottom-right corner
                    G[i][j] = "\u2518";
                } else if ((j % 3 == 0) && (i % vEnd != 0 && i != vEnd - 1 && i % 2 == 1)) {
                    G[i][j] = "\u2502";
                } else if (i % 2 == 0) {
                    G[i][j] = "\u2500";
                }

                if (m > 1) { // More than one column
                    if (j % 3 == 0 && j != 0 && j != hEnd - 1 && i == 0) {
                        G[i][j] = "\u252c";
                    }
                    if (j % 3 == 0 && j != 0 && j != hEnd - 1 && i == vEnd - 1) {
                        G[i][j] = "\u2534";
                    }
                }

                if (n > 1) { // More than one row
                    if (i % 2 == 0 && i != 0 && i != vEnd - 1 && j == 0) {
                        G[i][j] = "\u251c";
                    }
                    if (i % 2 == 0 && i != 0 && i != vEnd - 1 && j == hEnd - 1) {
                        G[i][j] = "\u2524";
                    }
                }

                if (n > 1 && m > 1) { // More than one row and column
                    if (i % 2 == 0 && j % 3 == 0 && i != 0 && j != 0 && i != vEnd - 1 && j != hEnd - 1) {
                        G[i][j] = "\u253c";
                    }
                }
            }
        }

        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.set(G);
        Tracer.delay();

        buildMaze();
    }

    private static void buildMaze() {
        DisjointSet mySet = new DisjointSet();
        int width = m;
        int height = n;
        int setSize = 0;
        int[][] graph = new int[width][height];
        boolean[] visitedMap = new boolean[width * height];
        boolean[] wallDown = new boolean[width * height];
        boolean[] wallRight = new boolean[width * height];
        List<int[]> rightWalls = new ArrayList<>();
        List<int[]> downWalls = new ArrayList<>();
        int location = 0;

        mySet.addElements(width * height);

        // logger {
        logger.println("initializing grid (all walls are up)");
        // }
        // init 'graph'
        // each room has two walls, a down and right wall.
        for (int i = 0; i < width; i++) {
            for (int j = 0; j < height; j++) {
                graph[i][j] = location;

                wallDown[location] = true;
                wallRight[location] = true;
                visitedMap[location] = false;

                // If you can label the rooms with just 2 digits
                if (width * height < 100) {
                    String locationString = String.valueOf(location);

                    G[j * 2 + 1][i * 3 + 1] = String.valueOf(locationString.charAt(0));
                    G[j * 2 + 1][i * 3 + 2] = locationString.length() > 1 ? String.valueOf(locationString.charAt(1)) : " ";

                    // visualize {
                    tracer.set(G);
                    // }
                }

                rightWalls.add(new int[]{i, j});
                downWalls.add(new int[]{i, j});
                location++;
            }
        }

        // logger {
        logger.println("shuffled the walls for random selection");
        // }
        // Randomly shuffle the walls
        shuffle(rightWalls);
        shuffle(downWalls);

        // Picking random walls to remove
        while (setSize != mySet.elements - 1) {
            int randomWall = (int) Math.floor(Math.random() * 2) + 1;
            if (randomWall == 1 && downWalls.size() > 0) {
                // Down wall
                int[] currentRoom = downWalls.remove(downWalls.size() - 1);
                int iX = currentRoom[0];
                int iY = currentRoom[1];
                int iYdown = iY + 1;
                if (iYdown < height) {
                    int u = graph[iX][iY];
                    int v = graph[iX][iYdown];
                    // visualize {
                    tracer.patch(iY * 2 + 1, iX * 3 + 1);
                    tracer.patch(iY * 2 + 1, iX * 3 + 2);
                    tracer.patch(iYdown * 2 + 1, iX * 3 + 1);
                    tracer.patch(iYdown * 2 + 1, iX * 3 + 2);
                    // }
                    if (mySet.find(u) != mySet.find(v)) {
                        // logger {
                        logger.println("Rooms: " + u + " & " + v + " now belong to the same set, delete wall between them");

                        Tracer.delay();
                        // }
                        mySet.setUnion(u, v);
                        setSize++;
                        // delete wall
                        wallDown[u] = false;
                    } else {
                        // logger {
                        logger.println("Rooms: " + u + " & " + v + " would create a cycle! This is not good!");
                        Tracer.delay();
                        // }
                    }
                    // visualize {
                    tracer.depatch(iY * 2 + 1, iX * 3 + 1);
                    tracer.depatch(iY * 2 + 1, iX * 3 + 2);
                    tracer.depatch(iYdown * 2 + 1, iX * 3 + 1);
                    tracer.depatch(iYdown * 2 + 1, iX * 3 + 2);
                    // }
                }
            } else if (randomWall == 2 && rightWalls.size() > 0) {
                // Right Wall
                int[] currentRoom = rightWalls.remove(rightWalls.size() - 1);
                int iX = currentRoom[0];
                int iY = currentRoom[1];
                int iXright = iX + 1;
                if (iXright < width) {
                    int u = graph[iX][iY];
                    int v = graph[iXright][iY];
                    // visualize {
                    tracer.patch(iY * 2 + 1, iX * 3 + 1);
                    tracer.patch(iY * 2 + 1, iX * 3 + 2);
                    tracer.patch(iY * 2 + 1, iXright * 3 + 1);
                    tracer.patch(iY * 2 + 1, iXright * 3 + 2);
                    // }
                    if (mySet.find(u) != mySet.find(v)) {
                        // logger {
                        logger.println("Rooms: " + u + " & " + v + " now belong to the same set, delete wall between them");

                        Tracer.delay();
                        // }
                        mySet.setUnion(u, v);
                        setSize++;
                        // delete wall
                        wallRight[u] = false;
                    } else {
                        // logger {
                        logger.println("Rooms: " + u + " & " + v + " would create a cycle! This is not good!");
                        Tracer.delay();
                        // }
                    }
                    // visualize {
                    tracer.depatch(iY * 2 + 1, iX * 3 + 1);
                    tracer.depatch(iY * 2 + 1, iX * 3 + 2);
                    tracer.depatch(iY * 2 + 1, iXright * 3 + 1);
                    tracer.depatch(iY * 2 + 1, iXright * 3 + 2);
                    // }
                }
            }
        }

        // logger {
        logger.println("deleting the walls");
        // }
        // update deleted walls
        for (int i = 0; i < width; i++) {
            for (int j = 0; j < height; j++) {
                int loc = graph[i][j];

                if (!wallDown[loc]) {
                    G[j * 2 + 2][i * 3 + 1] = " ";
                    G[j * 2 + 2][i * 3 + 2] = " ";
                    // visualize {
                    tracer.select(j * 2 + 2, i * 3 + 1);
                    Tracer.delay();
                    tracer.select(j * 2 + 2, i * 3 + 2);
                    Tracer.delay();
                    // }
                }

                if (!wallRight[loc]) {
                    G[j * 2 + 1][i * 3 + 3] = " ";
                    // visualize {
                    tracer.select(j * 2 + 1, i * 3 + 3);
                    Tracer.delay();
                    // }
                }
                // visualize {
                tracer.set(G);
                // }
            }
        }
        // logger {
        logger.println("cleaning up the grid!");
        // }
        cleanUpGrid(width, height);

        // Clear out walls for the start and end locations.
        int randomStart = (int) Math.floor(Math.random() * width);
        int randomEnd = (int) Math.floor(Math.random() * width);

        // logger {
        logger.println("setting the Start (S) & End (E) locations");
        // }

        // Start Location
        G[0][randomStart * 3 + 1] = " ";
        G[0][randomStart * 3 + 2] = " ";
        G[1][randomStart * 3 + 1] = "S";

        // End Location
        G[vEnd - 1][randomEnd * 3 + 1] = " ";
        G[vEnd - 1][randomEnd * 3 + 2] = " ";
        G[vEnd - 2][randomEnd * 3 + 1] = "E";

        cleanUpStartLocation(randomStart);
        cleanUpEndLocation(randomEnd);

        // logger {
        logger.println("maze is completed!");
        // }

        // set the data
        // visualize {
        tracer.set(G);
        // }
    }

    private static void cleanUpStartLocation(int start) {
        if (G[0][start * 3].equals("\u252c") && G[1][start * 3].equals("\u2502")) {
            G[0][start * 3] = "\u2510";
        }
        if (G[0][start * 3 + 3].equals("\u252c") && G[1][start * 3 + 3].equals("\u2502")) {
            G[0][start * 3 + 3] = "\u250c";
        }
        if (G[0][start * 3].equals("\u250c")) {
            G[0][start * 3] = "\u2502";
        }
        if (G[0][start * 3 + 3].equals("\u2510")) {
            G[0][start * 3 + 3] = "\u2502";
        }
    }

    private static void cleanUpEndLocation(int end) {
        if (G[vEnd - 1][end * 3].equals("\u2534") && G[vEnd - 2][end * 3].equals("\u2502")) {
            G[vEnd - 1][end * 3] = "\u2518";
        }
        if (G[vEnd - 1][end * 3 + 3].equals("\u2534") && G[vEnd - 2][end * 3 + 3].equals("\u2502")) {
            G[vEnd - 1][end * 3 + 3] = "\u2514";
        }
        if (G[vEnd - 1][end * 3].equals("\u2514")) {
            G[vEnd - 1][end * 3] = "\u2502";
        }
        if (G[vEnd - 1][end * 3 + 3].equals("\u2518")) {
            G[vEnd - 1][end * 3 + 3] = "\u2502";
        }
    }

    private static void cleanUpGrid(int width, int height) {
        // Remove room numbers
        for (int i = 0; i < width; i++) {
            for (int j = 0; j < height; j++) {
                G[j * 2 + 1][i * 3 + 1] = " ";
                G[j * 2 + 1][i * 3 + 2] = " ";
            }
        }

        // clean up grid for looks
        for (int i = 0; i < vEnd; i++) {
            for (int j = 0; j < hEnd; j++) {
                if (G[i][j].equals("\u251c")) {
                    if (G[i][j + 1].equals(" ")) {
                        G[i][j] = "\u2502";
                    }
                }

                if (G[i][j].equals("\u2524")) {
                    if (G[i][j - 1].equals(" ")) {
                        G[i][j] = "\u2502";
                    }
                }

                if (G[i][j].equals("\u252c")) {
                    if (G[i + 1][j].equals(" ")) {
                        G[i][j] = "\u2500";
                    }
                }

                if (G[i][j].equals("\u2534")) {
                    if (G[i - 1][j].equals(" ")) {
                        G[i][j] = "\u2500";
                    }
                }

                if (G[i][j].equals("\u253c")) {
                    if (G[i][j + 1].equals(" ") && G[i - 1][j].equals(" ") && !G[i][j - 1].equals(" ") && !G[i + 1][j].equals(" ")) {
                        G[i][j] = "\u2510";
                    } else if (G[i][j - 1].equals(" ") && G[i - 1][j].equals(" ") && !G[i + 1][j].equals(" ") && !G[i][j + 1].equals(" ")) {
                        G[i][j] = "\u250c";
                    } else if (G[i][j - 1].equals(" ") && G[i + 1][j].equals(" ") && !G[i - 1][j].equals(" ") && !G[i][j + 1].equals(" ")) {
                        G[i][j] = "\u2514";
                    } else if (G[i][j + 1].equals(" ") && G[i + 1][j].equals(" ") && !G[i - 1][j].equals(" ") && !G[i][j - 1].equals(" ")) {
                        G[i][j] = "\u2518";
                    } else if (G[i][j + 1].equals(" ") && G[i][j - 1].equals(" ") && (G[i + 1][j].equals(" ") || G[i - 1][j].equals(" "))) {
                        G[i][j] = "\u2502";
                    } else if (G[i + 1][j].equals(" ") && G[i - 1][j].equals(" ") && (G[i][j - 1].equals(" ") || G[i][j + 1].equals(" "))) {
                        G[i][j] = "\u2500";
                    } else if (G[i][j + 1].equals(" ") && G[i][j - 1].equals(" ")) {
                        G[i][j] = "\u2502";
                    } else if (G[i + 1][j].equals(" ") && G[i - 1][j].equals(" ")) {
                        G[i][j] = "\u2500";
                    } else if (G[i + 1][j].equals(" ") && !G[i - 1][j].equals(" ") && !G[i][j - 1].equals(" ") && !G[i][j + 1].equals(" ")) {
                        G[i][j] = "\u2534";
                    } else if (G[i - 1][j].equals(" ") && !G[i + 1][j].equals(" ") && !G[i][j + 1].equals(" ") && !G[i][j - 1].equals(" ")) {
                        G[i][j] = "\u252c";
                    } else if (G[i][j + 1].equals(" ") && !G[i - 1][j].equals(" ") && !G[i + 1][j].equals(" ") && !G[i][j - 1].equals(" ")) {
                        G[i][j] = "\u2524";
                    } else if (G[i][j - 1].equals(" ") && !G[i - 1][j].equals(" ") && !G[i + 1][j].equals(" ") && !G[i][j + 1].equals(" ")) {
                        G[i][j] = "\u251c";
                    }
                }
            }
        }
    }

    // http://bost.ocks.org/mike/shuffle/
    private static void shuffle(List<int[]> array) {
        int m = array.size();
        // While there remain elements to shuffle...
        while (m > 0) {
            // Pick a remaining element...
            int i = (int) Math.floor(Math.random() * m--);
            // And swap it with the current element.
            int[] t = array.get(m);
            array.set(m, array.get(i));
            array.set(i, t);
        }
    }

    private static class DisjointSet {
        int[] set = new int[0];
        int elements = 0;

        void addElements(int numberOfElements) {
            set = new int[numberOfElements];
            for (int i = 0; i < numberOfElements; i++) {
                elements++;
                set[i] = -1;
            }
        }

        int find(int element) {
            if (set[element] < 0) {
                return element;
            }
            return set[element] = find(set[element]);
        }

        void setUnion(int _a, int _b) {
            int a = find(_a);
            int b = find(_b);

            if (a != b) {
                int newSize = set[a] + set[b];
                if (compareSize(a, b)) {
                    set[b] = a;
                    set[a] = newSize;
                } else {
                    set[a] = b;
                    set[b] = newSize;
                }
            }
        }

        boolean compareSize(int a, int b) {
            if (set[a] == set[b]) {
                return true;
            } else if (set[a] < set[b]) {
                return true;
            }
            return false;
        }
    }
}
