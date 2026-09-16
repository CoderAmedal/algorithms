import org.algorithm_visualizer.*;

class Main {

    private static int[] A = {1, 3, 3, 2, 1, 1, 1};

    private static int N = A.length;

    private static Array1DTracer tracer = new Array1DTracer("List of element");

    private static LogTracer logger = new LogTracer("Console");

    public static void main(String[] args) {
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer, logger}));
        tracer.set(A);
        Tracer.delay();
        findMajorityElement();
    }

    private static boolean isMajorityElement(int element) {
        int count = 0;
        logger.printf("Verify majority element %d\n", element);
        for (int i = N - 1; i >= 0; i--) {
            tracer.patch(i, A[i]);
            Tracer.delay();
            if (A[i] == element) {
                count++;
            } else {
                tracer.depatch(i);
            }
        }
        logger.printf("Count of our assumed majority element %d\n", count);
        if (count > (int) Math.floor(N / 2.0)) {
            logger.printf("Our assumption was correct!\n");
            return true;
        }
        logger.printf("Our assumption was incorrect!\n");
        return false;
    }

    private static int findProbableElement() {
        int index = 0;
        int count = 1;
        tracer.select(index);
        Tracer.delay();
        logger.printf("Beginning with assumed majority element : %d count : %d\n", A[index], count);
        logger.printf("--------------------------------------------------------\n");
        for (int i = 1; i < N; i++) {
            tracer.patch(i, A[i]);
            Tracer.delay();
            if (A[index] == A[i]) {
                count++;
                logger.printf("Same as assumed majority element! Count : %d\n", count);
            } else {
                count--;
                logger.printf("Not same as assumed majority element! Count : %d\n", count);
            }

            if (count == 0) {
                logger.printf("Wrong assumption in majority element\n");
                tracer.deselect(index);
                tracer.depatch(i);
                index = i;
                count = 1;
                tracer.select(i);
                Tracer.delay();
                logger.printf("New assumed majority element!%d Count : %d\n", A[i], count);
                logger.printf("--------------------------------------------------------\n");
            } else {
                tracer.depatch(i);
            }
        }
        logger.printf("Finally assumed majority element %d\n", A[index]);
        logger.printf("--------------------------------------------------------\n");
        return A[index];
    }

    private static void findMajorityElement() {
        int element = findProbableElement();
        if (isMajorityElement(element)) {
            logger.printf("Majority element is %d\n", element);
        } else {
            logger.printf("No majority element\n");
        }
    }
}
