import org.algorithm_visualizer.*;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedHashMap;

class Main {

    private static Array1DTracer tracerA = new Array1DTracer("A");

    private static Array1DTracer tracerB = new Array1DTracer("B");

    private static LogTracer logTracer = new LogTracer("Console");

    private static String[] aKeys = {"Flavio", "Stephen", "Albert", "Jack"};

    private static String[] bKeys = {"July", "Valentine", "Violet", "Summer"};

    private static class Person {
        String key;
        Person stable; // null means unstable
        ArrayList<String> rankKeys;

        Person(String key, String[] ranks) {
            this.key = key;
            this.stable = null;
            this.rankKeys = new ArrayList<>(Arrays.asList(ranks));
        }
    }

    public static void main(String[] args) {
        Layout.setRoot(new VerticalLayout(new Commander[]{tracerA, tracerB, logTracer}));
        tracerA.set(aKeys);
        tracerB.set(bKeys);
        Tracer.delay();

        LinkedHashMap<String, Person> A = new LinkedHashMap<>();
        A.put("Flavio", new Person("Flavio", new String[]{"Valentine", "July", "Summer", "Violet"}));
        A.put("Stephen", new Person("Stephen", new String[]{"Summer", "July", "Valentine", "Violet"}));
        A.put("Albert", new Person("Albert", new String[]{"July", "Violet", "Valentine", "Summer"}));
        A.put("Jack", new Person("Jack", new String[]{"July", "Violet", "Valentine", "Summer"}));

        LinkedHashMap<String, Person> B = new LinkedHashMap<>();
        B.put("July", new Person("July", new String[]{"Jack", "Stephen", "Albert", "Flavio"}));
        B.put("Valentine", new Person("Valentine", new String[]{"Flavio", "Jack", "Stephen", "Albert"}));
        B.put("Violet", new Person("Violet", new String[]{"Jack", "Stephen", "Flavio", "Albert"}));
        B.put("Summer", new Person("Summer", new String[]{"Stephen", "Flavio", "Albert", "Jack"}));

        Person a;
        while ((a = extractUnstable(A)) != null) {
            logTracer.printf("Selecting %s\n", a.key);
            Tracer.delay();

            String bKey = a.rankKeys.remove(0);
            Person b = B.get(bKey);

            logTracer.printf("--> Choicing %s\n", b.key);
            Tracer.delay();

            if (b.stable == null) {
                logTracer.printf("--> %s is not stable, stabilizing with %s\n", b.key, a.key);
                Tracer.delay();

                a.stable = b;
                b.stable = a;

                tracerA.select(indexOf(aKeys, a.key));
                Tracer.delay();
                tracerB.select(indexOf(bKeys, b.key));
                Tracer.delay();
            } else {
                int rankAinB = b.rankKeys.indexOf(a.key);
                int rankPrevAinB = b.rankKeys.indexOf(b.stable.key);
                if (rankAinB < rankPrevAinB) {
                    logTracer.printf("--> %s is more stable with %s rather than %s - stabilizing again\n", bKey, a.key, b.stable.key);
                    Tracer.delay();

                    A.get(b.stable.key).stable = null;
                    tracerA.deselect(indexOf(aKeys, b.stable.key));
                    Tracer.delay();

                    a.stable = b;
                    b.stable = a;

                    tracerA.select(indexOf(aKeys, a.key));
                    Tracer.delay();
                    tracerB.select(indexOf(bKeys, b.key));
                    Tracer.delay();
                }
            }
        }
    }

    private static Person extractUnstable(LinkedHashMap<String, Person> Q) {
        for (Person person : Q.values()) {
            if (person.stable == null) {
                return person;
            }
        }
        return null;
    }

    private static int indexOf(String[] keys, String key) {
        for (int i = 0; i < keys.length; i++) {
            if (keys[i].equals(key)) return i;
        }
        return -1;
    }
}
