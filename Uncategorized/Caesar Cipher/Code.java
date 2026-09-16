// import visualization libraries {
import org.algorithm_visualizer.*;
import java.util.ArrayList;
import java.util.List;
// }

class Main {

    private static final String ALPHABET = "abcdefghijklmnopqrstuvwxyz";
    private static final int ROTATION = 5;

    private static Array1DTracer encryptTracer = new Array1DTracer("Encryption");

    private static Array1DTracer decryptTracer = new Array1DTracer("Decryption");

    private static LogTracer logger = new LogTracer();

    public static void main(String[] args) {
        String string = "hello! how are you doing?";

        // define tracer variables {
        Layout.setRoot(new VerticalLayout(new Commander[]{encryptTracer, decryptTracer, logger}));

        encryptTracer.set(toList(string));
        Tracer.delay();
        // }

        String encrypted = encrypt(string, ROTATION);
        // logger {
        logger.println("Encrypted result: " + encrypted);
        // }

        decryptTracer.set(toList(encrypted));
        String decrypted = decrypt(encrypted, ROTATION);
        // logger {
        logger.println("Decrypted result: " + decrypted);
        // }
    }

    private static List<String> toList(String text) {
        List<String> list = new ArrayList<>();
        for (char c : text.toCharArray()) list.add(String.valueOf(c));
        return list;
    }

    private static int getPosUp(int pos) {
        return (pos == ALPHABET.length() - 1) ? 0 : pos + 1;
    }

    private static int getPosDown(int pos) {
        return (pos == 0) ? ALPHABET.length() - 1 : pos - 1;
    }

    private static char getNextChar(char currChar, String direction) {
        int pos = ALPHABET.indexOf(currChar);
        int nextPos = direction.equals("up") ? getPosUp(pos) : getPosDown(pos);
        char nextChar = ALPHABET.charAt(nextPos);

        // logger {
        logger.println(currChar + " -> " + nextChar);
        // }
        return nextChar;
    }

    private static String cipher(String text, int rotation, String direction, Array1DTracer cipherTracer) {
        if (text == null || text.isEmpty()) return "";

        char[] chars = text.toCharArray();
        for (int i = 0; i < chars.length; i++) {
            // visualize {
            Tracer.delay();
            // }

            char currChar = chars[i];
            if (ALPHABET.indexOf(currChar) >= 0) { // don't encrypt/decrypt characters not in alphabet
                int r = rotation;

                // logger {
                logger.println("Rotating " + currChar + " " + direction + " " + rotation + " times");
                // }
                // visualize {
                cipherTracer.select(i);
                Tracer.delay();
                // }

                // perform given amount of rotations in the given direction
                while (r-- > 0) {
                    currChar = getNextChar(currChar, direction);
                    // visualize {
                    cipherTracer.patch(i, String.valueOf(currChar));
                    Tracer.delay();
                    // }
                }
            } else {
                // logger {
                logger.println("Ignore this character");
                // }
            }

            chars[i] = currChar;
            // logger {
            logger.println("Current result: " + new String(chars));
            // }
        }

        return new String(chars);
    }

    private static String encrypt(String text, int rotation) {
        // logger {
        logger.println("Encrypting: " + text);
        // }
        return cipher(text, rotation, "up", encryptTracer);
    }

    private static String decrypt(String text, int rotation) {
        // logger {
        logger.println("Decrypting: " + text);
        // }
        return cipher(text, rotation, "down", decryptTracer);
    }
}
