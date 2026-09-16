import org.algorithm_visualizer.*;

class Main {

    public static void main(String[] args) {
        String str1 = "stack";
        String str2 = "racket";
        int[][] table = new int[str1.length() + 1][str2.length() + 1];

        for (int i = 0; i < str1.length() + 1; i++) {
            for (int j = 0; j < str2.length() + 1; j++) table[i][j] = -1;
            table[i][0] = i;
        }
        for (int i = 1; i < str2.length() + 1; i++) {
            table[0][i] = i;
        }

        // define tracer variables {
        Array2DTracer tracer = new Array2DTracer("Distance Table");
        LogTracer logger = new LogTracer();
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.set(table);
        Tracer.delay();
        // }

        // logger {
        logger.println("Initialized DP Table");
        logger.println("Y-Axis (Top to Bottom): " + str1);
        logger.println("X-Axis (Left to Right): " + str2);
        // }

        int dist = editDistance(str1, str2, table, tracer, logger);

        // logger {
        logger.println("Minimum Edit Distance: " + dist);
        // }
    }

    private static int editDistance(String str1, String str2, int[][] table, Array2DTracer tracer, LogTracer logger) {
        // display grid with words
        // logger {
        logger.println("*** " + String.join(" ", str2.split("")));
        for (int index = 0; index < table.length; index++) {
            String character = (index == 0) ? "*" : String.valueOf(str1.charAt(index - 1));
            logger.println(character + "\t" + java.util.Arrays.toString(table[index]));
        }
        // }

        // begin ED execution
        for (int i = 1; i < str1.length() + 1; i++) {
            for (int j = 1; j < str2.length() + 1; j++) {
                if (str1.charAt(i - 1) == str2.charAt(j - 1)) {
                    // visualize {
                    tracer.select(i - 1, j - 1);
                    Tracer.delay();
                    // }
                    table[i][j] = table[i - 1][j - 1];
                    // visualize {
                    tracer.patch(i, j, table[i][j]);
                    Tracer.delay();
                    tracer.depatch(i, j);
                    tracer.deselect(i - 1, j - 1);
                    // }
                } else {
                    // visualize {
                    tracer.select(i - 1, j);
                    tracer.select(i, j - 1);
                    tracer.select(i - 1, j - 1);
                    Tracer.delay();
                    // }
                    table[i][j] = Math.min(table[i - 1][j], Math.min(table[i][j - 1], table[i - 1][j - 1])) + 1;
                    // visualize {
                    tracer.patch(i, j, table[i][j]);
                    Tracer.delay();
                    tracer.depatch(i, j);
                    tracer.deselect(i - 1, j);
                    tracer.deselect(i, j - 1);
                    tracer.deselect(i - 1, j - 1);
                    // }
                }
            }
        }

        // visualize {
        tracer.select(str1.length(), str2.length());
        // }
        return table[str1.length()][str2.length()];
    }
}
