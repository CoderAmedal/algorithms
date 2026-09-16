import org.algorithm_visualizer.*;

class Main {

    public static void main(String[] args) {
        String[] jobId = {"a", "b", "c", "d", "e"};
        int[] deadline = {2, 1, 2, 1, 3};
        int[] profit = {100, 19, 27, 25, 15};
        int N = deadline.length;
        // sort according to decreasing order of profit
        // Bubble sort implemented ... Implement a better algorithm for better performance
        for (int i = 0; i < N - 1; i++) {
            for (int j = 0; j < N - i - 1; j++) {
                if (profit[j] < profit[j + 1]) {
                    int temp = profit[j];
                    profit[j] = profit[j + 1];
                    profit[j + 1] = temp;
                    temp = deadline[j];
                    deadline[j] = deadline[j + 1];
                    deadline[j + 1] = temp;
                    String t = jobId[j];
                    jobId[j] = jobId[j + 1];
                    jobId[j + 1] = t;
                }
            }
        }

        int[] slot = new int[N];
        String[] result = new String[N];
        for (int i = 0; i < N; i++) {
            result[i] = "-";
        }

        Array1DTracer tracer3 = new Array1DTracer("Schedule");
        Array1DTracer tracer = new Array1DTracer("Job Ids");
        Array1DTracer tracer1 = new Array1DTracer("Deadlines");
        Array1DTracer tracer2 = new Array1DTracer("Profit");
        Layout.setRoot(new VerticalLayout(new Commander[]{tracer3, tracer, tracer1, tracer2}));
        tracer.set(jobId);
        tracer1.set(deadline);
        tracer2.set(profit);
        tracer3.set(result);
        Tracer.delay();

        // Initialise all slots to free
        for (int i = 0; i < N; i++) {
            slot[i] = 0;
        }

        // Iterate through all the given jobs
        for (int i = 0; i < N; i++) {
            // Start from the last possible slot. Find a slot for the job
            tracer.select(i);
            Tracer.delay();
            tracer1.select(i);
            Tracer.delay();
            for (int j = Math.min(N, deadline[i]) - 1; j >= 0; j--) {
                if (slot[j] == 0) {
                    tracer3.patch(j, jobId[i]);
                    Tracer.delay();
                    result[j] = jobId[i];
                    slot[j] = 1;
                    tracer3.depatch(j);
                    break;
                }
            }
            tracer.deselect(i);
            tracer1.deselect(i);
        }
    }
}
