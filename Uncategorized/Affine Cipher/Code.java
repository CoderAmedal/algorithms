// import visualization libraries {
import org.algorithm_visualizer.*;
import java.util.ArrayList;
import java.util.List;
// }

class Main {

    private static final int A_KEY = 5;
    private static final int B_KEY = 7;
    private static final int N = 26;

    private static Array1DTracer ptTracer = new Array1DTracer("Encryption");

    private static Array1DTracer ctTracer = new Array1DTracer("Decryption");

    private static LogTracer logger = new LogTracer();

    public static void main(String[] args) {
        // define tracer variables {
        Layout.setRoot(new VerticalLayout(new Commander[]{ptTracer, ctTracer, logger}));

        String plainText = "secret";
        ptTracer.set(toList(plainText));
        Tracer.delay();
        // }

        String cipherText = encrypt(plainText);
        ctTracer.set(toList(cipherText));
        decrypt(cipherText);
    }

    private static List<String> toList(String text) {
        List<String> list = new ArrayList<>();
        for (char c : text.toCharArray()) list.add(String.valueOf(c));
        return list;
    }

    private static char cryptAlpha(char alpha) {
        int index = alpha - 'a';
        int result = ((A_KEY * index) + B_KEY) % N;
        if (result < 0) result += N;

        // logger {
        logger.println(String.format("Index of %s = %d", alpha, index));
        // }

        return (char) (result + 'a');
    }

    private static String encrypt(String text) {
        StringBuilder cypherText = new StringBuilder();

        // logger {
        logger.println("Beginning Affine Encryption");
        logger.println("Encryption formula: <b>((keys.a * indexOfAlphabet) + keys.b) % N</b>");
        logger.println(String.format("keys.a=%d, keys.b=%d, N=%d", A_KEY, B_KEY, N));
        // }

        for (int i = 0; i < text.length(); i++) {
            // visualize {
            ptTracer.select(i);
            Tracer.delay();
            ptTracer.deselect(i);
            // }

            char ch = cryptAlpha(text.charAt(i));
            cypherText.append(ch);

            // visualize {
            ptTracer.patch(i, String.valueOf(ch));
            Tracer.delay();
            ptTracer.depatch(i);
            // }
        }

        return cypherText.toString();
    }

    private static String decrypt(String cypherText) {
        StringBuilder text = new StringBuilder();
        int aInverse = 0;
        for (int i = 1; i < N; i++) {
            if ((A_KEY * i) % N == 1) {
                aInverse = i;
                break;
            }
        }

        // logger {
        logger.println(String.format("a<sup>-1</sup> = %d", aInverse));
        // }

        // logger {
        logger.println("Beginning Affine Decryption");
        logger.println("Decryption formula: <b>(a<sup>-1</sup> * (index - keys.b)) % N</b>");
        logger.println(String.format("keys.b=%d, N=%d", B_KEY, N));
        // }

        for (int i = 0; i < cypherText.length(); i++) {
            // visualize {
            ctTracer.select(i);
            Tracer.delay();
            ctTracer.deselect(i);
            Tracer.delay();
            // }

            int index = cypherText.charAt(i) - 'a';
            int result = (aInverse * (index - B_KEY)) % N;
            if (result < 0) result += N;

            // logger {
            logger.println(String.format("Index of %s = %d", cypherText.charAt(i), index));
            // }

            char ch = (char) (result + 'a');
            text.append(ch);

            // visualize {
            ctTracer.patch(i, String.valueOf(ch));
            Tracer.delay();
            ctTracer.depatch(i);
            Tracer.delay();
            // }
        }

        return text.toString();
    }
}
