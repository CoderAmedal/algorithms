import org.algorithm_visualizer.*;

class Main {

    private static Array1DTracer textTracer = new Array1DTracer("text");

    private static Array1DTracer pattTracer = new Array1DTracer("pattern");

    private static Array1DTracer concatTracer = new Array1DTracer("concatenated string");

    private static Array1DTracer tracer = new Array1DTracer("zArray");

    private static LogTracer logger = new LogTracer();

    private static String pattern = "abc";

    private static String text = "xabcabzabc";

    private static int len = pattern.length() + text.length() + 1;

    private static int[] z = new int[len];

    public static void main(String[] args) {
        pattTracer.set(pattern);
        textTracer.set(text);
        tracer.set(z);
        Layout.setRoot(new VerticalLayout(new Commander[]{textTracer, pattTracer, concatTracer, tracer, logger}));
        Tracer.delay();

        String concat = pattern + "$" + text;
        concatTracer.set(concat);
        int patLen = pattern.length();
        createZarr(concat);
        tracer.set(z);

        logger.println("The Values in Z array equal to the length of the pattern indicates the index at which the pattern is present");
        logger.println("===================================");
        for (int i = 0; i < len; i++) {
            if (z[i] == patLen) {
                int pos = i - (patLen + 1);
                logger.printf("Pattern Found at index %d\n", pos);
            }
        }
        logger.println("===================================");
    }

    private static void createZarr(String concat) {
        int left = 0;
        int right = 0;
        int N = concat.length();
        for (int i = 1; i < N; i++) {
            tracer.select(i);
            Tracer.delay();
            if (i > right) {
                left = right = i;
                while (right < N && concat.charAt(right) == concat.charAt(right - left)) {
                    concatTracer.patch(right);
                    concatTracer.select(right - left);
                    logger.printf("%c (at index %d) is equal to %c (at index %d)\n", concat.charAt(right), right, concat.charAt(right - left), right - left);
                    Tracer.delay();
                    concatTracer.depatch(right);
                    concatTracer.deselect(right - left);
                    right++;
                }
                if (right < N) {
                    concatTracer.patch(right);
                    concatTracer.select(right - left);
                    logger.printf("%c (at index %d) is NOT equal to %c (at index %d)\n", concat.charAt(right), right, concat.charAt(right - left), right - left);
                    Tracer.delay();
                    concatTracer.depatch(right);
                    concatTracer.deselect(right - left);
                }
                z[i] = right - left;
                logger.println("--------------------------------");
                logger.printf("Value of z[%d] = the length of the substring starting from %d which is also the prefix of the concatinated string(=%d)\n", i, i, right - left);
                logger.println("--------------------------------");
                right--;
            } else if (z[i - left] < (right - i + 1)) {
                logger.printf("The substring from index %d will not cross the right end.\n", i - left);
                concatTracer.patch(right - i + 1);
                concatTracer.select(i - left);
                Tracer.delay();
                z[i] = z[i - left];
                concatTracer.depatch(right - i + 1);
                concatTracer.deselect(i - left);
            } else {
                logger.printf("The substring from index %d will cross the right end.\n", i - left);
                left = i;
                while (right < N && concat.charAt(right) == concat.charAt(right - left)) {
                    concatTracer.patch(right);
                    concatTracer.select(right - left);
                    logger.printf("%c (at index %d) is equal to %c (at index %d)\n", concat.charAt(right), right, concat.charAt(right - left), right - left);
                    Tracer.delay();
                    concatTracer.depatch(right);
                    concatTracer.deselect(right - left);
                    right++;
                }
                if (right < N) {
                    concatTracer.patch(right);
                    concatTracer.select(right - left);
                    logger.printf("%c (at index %d) is NOT equal to %c (at index %d)\n", concat.charAt(right), right, concat.charAt(right - left), right - left);
                    Tracer.delay();
                    concatTracer.depatch(right);
                    concatTracer.deselect(right - left);
                }
                z[i] = right - left;
                right--;
                logger.println("--------------------------------");
                logger.printf("Value of z[%d] = the length of the substring starting from %d which is also the prefix of the concatinated string(=%d)\n", i, i, right - left);
                logger.println("--------------------------------");
            }
            tracer.deselect(i);
            tracer.set(z);
        }
    }
}
