import org.algorithm_visualizer.*;
import java.util.Arrays;

class Main {

    private static ChartTracer chartTracer = new ChartTracer();

    private static LogTracer logTracer = new LogTracer("Console");

    private static Integer[] array = (Integer[]) new Randomize.Array1D(15, new Randomize.Integer(1, 20)).create();

    public static void main(String[] args) {
        int N = array.length;
        int writes = 0; // number of writing performed
        Layout.setRoot(new VerticalLayout(new Commander[]{chartTracer, logTracer}));
        logTracer.printf("original array = %s\n", Arrays.toString(array));
        chartTracer.set(array);
        Tracer.delay();

        for (int cycleStart = 0; cycleStart <= N - 2; cycleStart++) {
            int item = array[cycleStart];

            // find where to put the item
            int pos = cycleStart;
            chartTracer.select(cycleStart);

            for (int i = cycleStart + 1; i <= N - 1; i++) {
                chartTracer.select(i);
                Tracer.delay();
                chartTracer.deselect(i);
                if (array[i] < item) {
                    pos++;
                }
            }

            // if the item is already there, this is not a circle
            if (pos == cycleStart) {
                chartTracer.deselect(cycleStart);
                continue;
            }

            // otherwise put the item there or right after any duplicates
            while (item == array[pos]) {
                pos++;
            }

            // write item to new index and increment writes
            int temp = array[pos];
            array[pos] = item;
            item = temp;

            writes++;

            if (pos != cycleStart) {
                logTracer.printf("Rewrite %d to index %d; the next value to rewrite is %d\n", array[pos], pos, item);
            } else {
                logTracer.printf("Rewrite %d to index %d\n", array[pos], pos);
            }
            chartTracer.select(pos);
            Tracer.delay();
            chartTracer.deselect(pos);
            chartTracer.patch(pos, array[pos]);
            chartTracer.patch(cycleStart, array[cycleStart]);
            Tracer.delay();
            chartTracer.depatch(pos);
            chartTracer.depatch(cycleStart);

            // rotate the rest of the cycle
            while (pos != cycleStart) {
                pos = cycleStart;

                for (int i = cycleStart + 1; i <= N - 1; i++) {
                    chartTracer.select(i);
                    Tracer.delay();
                    chartTracer.deselect(i);
                    if (array[i] < item) {
                        pos++;
                    }
                }

                while (item == array[pos]) {
                    pos++;
                }

                temp = array[pos];
                array[pos] = item;
                item = temp;

                if (pos != cycleStart) {
                    logTracer.printf("Rewrite %d to index %d; the next value to rewrite is %d\n", array[pos], pos, item);
                } else {
                    logTracer.printf("Rewrite %d to index %d\n", array[pos], pos);
                }
                chartTracer.select(pos);
                Tracer.delay();
                chartTracer.deselect(pos);
                chartTracer.patch(pos, array[pos]);
                chartTracer.patch(cycleStart, array[cycleStart]);
                Tracer.delay();
                chartTracer.depatch(pos);
                chartTracer.depatch(cycleStart);

                writes++;
            }
        }

        logTracer.printf("Number of writes performed is %d\n", writes);
    }
}
