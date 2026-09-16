import org.algorithm_visualizer.*;
import java.util.Arrays;
import java.util.Comparator;

class Main {

    private static Array2DTracer saTracer = new Array2DTracer("Suffix Array");

    private static Array1DTracer wordTracer = new Array1DTracer("Given Word");

    private static LogTracer logger = new LogTracer("Progress");

    private static String word = "virgo";

    private static Object[][] suffixArray = new Object[word.length() + 1][2];

    public static void main(String[] args) {
        for (int i = 0; i < suffixArray.length; i++) {
            suffixArray[i][0] = i + 1;
            suffixArray[i][1] = "-";
        }

        Layout.setRoot(new VerticalLayout(new Commander[]{saTracer, wordTracer, logger}));

        saTracer.set(suffixArray);
        String[] wordChars = new String[word.length()];
        for (int i = 0; i < word.length(); i++) {
            wordChars[i] = String.valueOf(word.charAt(i));
        }
        wordTracer.set(wordChars);
        Tracer.delay();

        word += "$"; // special character
        logger.printf("Appended '$' at the end of word as terminating (special) character. Beginning filling of suffixes\n");

        createSA(suffixArray, word);

        logger.printf("Re-organizing Suffix Array in sorted order of suffixes using efficient sorting algorithm (O(N.log(N)))\n");

        Arrays.sort(suffixArray, new Comparator<Object[]>() {
            public int compare(Object[] a, Object[] b) {
                String as = (String) a[1];
                String bs = (String) b[1];
                logger.printf("The condition a [1] (%s) > b [1] (%s) is %b\n", as, bs, as.compareTo(bs) > 0);
                return as.compareTo(bs);
            }
        });

        for (int i = 0; i < word.length(); i++) {
            saTracer.patch(i, 0, suffixArray[i][0]);
            saTracer.patch(i, 1, suffixArray[i][1]);
            Tracer.delay();

            saTracer.depatch(i, 0);
            saTracer.depatch(i, 1);
        }
    }

    private static void selectSuffix(String word, int start) {
        int c = start;
        int i = start;

        while (i < word.length() - 1) {
            wordTracer.select(i);
            i++;
        }
        Tracer.delay();

        while (c < word.length() - 1) {
            wordTracer.deselect(c);
            c++;
        }
        Tracer.delay();
    }

    private static void createSA(Object[][] sa, String word) {
        for (int i = 0; i < word.length(); i++) {
            sa[i][1] = word.substring(i);

            selectSuffix(word, i);
            saTracer.patch(i, 1, sa[i][1]);
            Tracer.delay();
            saTracer.depatch(i, 1);
            Tracer.delay();
        }
    }
}
