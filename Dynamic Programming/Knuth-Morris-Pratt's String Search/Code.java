import org.algorithm_visualizer.*;
import java.util.ArrayList;

class Main {

    private static Array2DTracer patternTracer = new Array2DTracer("Pattern");

    private static Array1DTracer stringTracer = new Array1DTracer("String");

    private static final String string = "AAAABAABAAAABAAABAAAA";

    private static final String pattern = "AAAABAAA";

    private static int[] _next = new int[pattern.length()];

    public static void main(String[] args) {
        // define tracer variables {
        Layout.setRoot(new VerticalLayout(new Commander[]{patternTracer, stringTracer}));
        patternTracer.set(new Object[]{_next, pattern, pattern});
        stringTracer.set(string);
        Tracer.delay();
        // }

        kmp(string, pattern);
    }

    private static void getNext(String pattern) {
        int q = 1; // postfix pointer
        int k = 0; // prefix pointer
        // visualize {
        patternTracer.select(2, k);
        // }
        for (; q < pattern.length(); ++q) {
            // visualize {
            patternTracer.select(1, q);
            Tracer.delay();
            // }
            while ((k > 0) && (pattern.charAt(q) != pattern.charAt(k))) {
                // visualize {
                patternTracer.select(0, k - 1);
                Tracer.delay();
                patternTracer.deselect(2, k);
                patternTracer.select(2, _next[k - 1]);
                Tracer.delay();
                patternTracer.deselect(0, k - 1);
                // }
                k = _next[k - 1];
            }
            if (pattern.charAt(q) == pattern.charAt(k)) {
                // visualize {
                patternTracer.deselect(2, k);
                patternTracer.select(2, k + 1);
                Tracer.delay();
                // }
                ++k;
            }
            // visualize {
            patternTracer.patch(0, q, k);
            Tracer.delay();
            patternTracer.depatch(0, q);
            Tracer.delay();
            patternTracer.deselect(1, q);
            // }
            _next[q] = k;
        }
        // visualize {
        patternTracer.deselect(2, k);
        patternTracer.set(new Object[]{_next, pattern});
        Tracer.delay();
        // }
    }

    private static void kmp(String string, String pattern) {
        ArrayList<Integer> matchPositions = new ArrayList<>();
        int matchStartPosition;

        int i = 0; // string pointer
        int k = 0; // pattern pointer
        getNext(pattern);
        for (; i < string.length(); i++) {
            // visualize {
            stringTracer.select(i);
            patternTracer.select(1, k);
            Tracer.delay();
            // }
            while ((k > 0) && (string.charAt(i) != pattern.charAt(k))) {
                // visualize {
                patternTracer.select(0, k - 1);
                Tracer.delay();
                patternTracer.deselect(1, k);
                patternTracer.select(1, _next[k - 1]);
                Tracer.delay();
                patternTracer.deselect(0, k - 1);
                // }
                k = _next[k - 1];
            }
            if (string.charAt(i) == pattern.charAt(k)) {
                ++k;
                if (k == pattern.length()) {
                    matchStartPosition = i - pattern.length() + 1;
                    matchPositions.add(matchStartPosition);
                    // visualize {
                    stringTracer.select(matchStartPosition, matchStartPosition + pattern.length() - 1);
                    Tracer.delay();
                    stringTracer.deselect(matchStartPosition, matchStartPosition + pattern.length() - 1);
                    Tracer.delay();
                    patternTracer.select(0, k - 1);
                    Tracer.delay();
                    patternTracer.deselect(1, k - 1);
                    patternTracer.select(1, _next[k - 1]);
                    Tracer.delay();
                    patternTracer.deselect(0, k - 1);
                    // }
                    k = _next[k - 1];
                } else {
                    // visualize {
                    patternTracer.deselect(1, k - 1);
                    patternTracer.select(1, k);
                    Tracer.delay();
                    // }
                }
            } else {
                // visualize {
                patternTracer.select(0, k);
                Tracer.delay();
                // }
            }
            // visualize {
            patternTracer.deselect(0, k);
            patternTracer.deselect(1, k);
            stringTracer.deselect(i);
            // }
        }
        // visualize {
        for (int j = 0; j < matchPositions.size(); j++) {
            stringTracer.select(matchPositions.get(j), matchPositions.get(j) + pattern.length() - 1);
            Tracer.delay();
            stringTracer.deselect(matchPositions.get(j), matchPositions.get(j) + pattern.length() - 1);
        }
        // }
    }
}
