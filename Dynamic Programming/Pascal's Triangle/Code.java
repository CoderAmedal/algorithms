import org.algorithm_visualizer.*;

class Main {

    private static Array2DTracer tracer = new Array2DTracer("Pascal's Triangle");

    public static void main(String[] args) {
        int N = 9;
        Integer[][] A = new Integer[N][N];

        Layout.setRoot(new VerticalLayout(new Commander[]{tracer}));
        tracer.set(A);
        Tracer.delay();

        for (int i = 0; i < N; i++) {
            for (int j = 0; j <= i; j++) {
                if (j == i || j == 0) { // First and last values in every row are 1
                    A[i][j] = 1;

                    tracer.patch(i, j, A[i][j]);
                    Tracer.delay();
                    tracer.depatch(i, j);
                } else { // Other values are sum of values just above and left of above
                    tracer.select(i - 1, j - 1);
                    Tracer.delay();
                    tracer.select(i - 1, j);
                    Tracer.delay();

                    A[i][j] = A[i - 1][j - 1] + A[i - 1][j];

                    tracer.patch(i, j, A[i][j]);
                    Tracer.delay();
                    tracer.depatch(i, j);
                    tracer.deselect(i - 1, j - 1);
                    tracer.deselect(i - 1, j);
                }
            }
        }
    }
}
