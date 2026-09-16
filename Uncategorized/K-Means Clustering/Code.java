// import visualization libraries {
import org.algorithm_visualizer.*;
import java.util.*;
// }

class Main {

    // define tracer variables {
    private static Array2DTracer array2dTracer = new Array2DTracer("Grid");

    private static LogTracer logTracer = new LogTracer("Console");
    // }

    private static List<double[]> unClusteredData = new ArrayList<>();

    public static void main(String[] args) {
        // define input variables
        Integer[][] raw = (Integer[][]) new Randomize.Array2D(15, 2, new Randomize.Integer(1, 9)).create();
        for (Integer[] row : raw) unClusteredData.add(new double[]{row[0], row[1]});
        int k = (int) new Randomize.Integer(2, Math.max(2, unClusteredData.size() / 5)).create();

        // visualize {
        Layout.setRoot(new VerticalLayout(new Commander[]{array2dTracer, logTracer}));

        logTracer.println("Un-clustered data = " + stringify(unClusteredData));
        List<List<double[]>> initial = new ArrayList<>();
        initial.add(unClusteredData);
        array2dTracer.set(arrayify(initial, true));

        Tracer.delay();
        // }

        // Start with random centers
        List<double[]> centers = chooseRandomCenters(unClusteredData, k);

        // trace {
        logTracer.println("Initial random selected centers = " + stringify(centers));

        Tracer.delay();
        // }

        // Cluster to the random centers
        List<List<double[]>> clusters = cluster(unClusteredData, centers);

        // trace {
        logTracer.println("Initial clusters = \n\t" + joinClusters(clusters));
        array2dTracer.set(arrayify(clusters, true));

        Tracer.delay();
        // }

        // start iterations here
        Ret ret = improve(0, clusters, centers);

        // trace {
        Tracer.delay();

        logTracer.println("Final clustered data = \n\t" + joinClusters(ret.clusters));
        logTracer.println("Best centers = " + stringify(ret.centers));
        array2dTracer.set(arrayify(ret.clusters, true));
        Tracer.delay();
        // }
    }

    // define helper functions {
    private static class Ret {
        List<List<double[]>> clusters;
        List<double[]> centers;

        Ret(List<List<double[]>> clusters, List<double[]> centers) {
            this.clusters = clusters;
            this.centers = centers;
        }
    }

    private static List<double[]> shuffle(List<double[]> a) {
        List<double[]> array = new ArrayList<>(a);
        List<double[]> copy = new ArrayList<>();
        int n = array.size();

        while (n > 0) {
            int i = (int) Math.floor(Math.random() * n--);
            copy.add(array.remove(i));
        }

        return copy;
    }

    private static List<double[]> chooseRandomCenters(List<double[]> data, int k) {
        return shuffle(data).subList(0, Math.min(k, data.size()));
    }

    private static String pointify(double[] p) {
        return "(" + p[0] + ", " + p[1] + ")";
    }

    private static List<String> arrayify(List<double[]> a) {
        List<String> result = new ArrayList<>();
        for (double[] p : a) result.add(pointify(p));
        return result;
    }

    private static List<List<String>> arrayify(List<List<double[]>> a, boolean nested) {
        List<List<String>> result = new ArrayList<>();
        for (List<double[]> c : a) result.add(arrayify(c));
        return result;
    }

    private static String stringify(List<double[]> a) {
        return String.join(", ", arrayify(a));
    }

    private static String joinClusters(List<List<double[]>> clusters) {
        List<String> parts = new ArrayList<>();
        for (List<double[]> c : clusters) parts.add(stringify(c));
        return String.join("\n\t", parts);
    }

    private static double distance(double[] p1, double[] p2) {
        return Math.pow(p1[0] - p2[0], 2) + Math.pow(p1[1] - p2[1], 2);
    }

    private static double mean(List<Double> a) {
        if (a.isEmpty()) return 0;
        double sum = 0;
        for (double v : a) sum += v;
        return sum / a.size();
    }

    private static double[] centerOfCluster(List<double[]> cluster) {
        List<Double> xs = new ArrayList<>();
        List<Double> ys = new ArrayList<>();
        for (double[] p : cluster) {
            xs.add(p[0]);
            ys.add(p[1]);
        }
        return new double[]{mean(xs), mean(ys)};
    }

    private static List<double[]> reCalculateCenters(List<List<double[]>> clusters) {
        List<double[]> centers = new ArrayList<>();
        for (List<double[]> c : clusters) centers.add(centerOfCluster(c));
        return centers;
    }

    private static boolean areCentersEqual(List<double[]> c1, List<double[]> c2) {
        if (c1 == null || c2 == null || c1.size() != c2.size()) return false;
        for (int i = 0; i < c1.size(); i++) {
            if (!Arrays.equals(c1.get(i), c2.get(i))) return false;
        }
        return true;
    }

    private static List<List<double[]>> cluster(List<double[]> data, List<double[]> centers) {
        List<List<double[]>> clusters = new ArrayList<>();
        for (int i = 0; i < centers.size(); i++) clusters.add(new ArrayList<>());

        for (double[] point : data) {
            double minDistance = Double.POSITIVE_INFINITY;
            int minDistanceIndex = -1;

            for (int j = 0; j < centers.size(); j++) {
                double d = distance(point, centers.get(j));

                if (d < minDistance) {
                    minDistance = d;
                    minDistanceIndex = j;
                }
            }

            clusters.get(minDistanceIndex).add(point);
        }

        return clusters;
    }
    // }

    private static Ret recenterAndCluster(List<List<double[]>> originalClusters) {
        List<double[]> centers = reCalculateCenters(originalClusters);
        List<List<double[]>> clusters = cluster(unClusteredData, centers);
        return new Ret(clusters, centers);
    }

    private static Ret improve(int loops, List<List<double[]>> clusters, List<double[]> centers) {
        while (true) {
            if (loops >= 1000) return new Ret(clusters, centers);

            loops++;

            Ret ret = recenterAndCluster(clusters);

            // trace {
            array2dTracer.set(arrayify(ret.clusters, true));

            logTracer.println("");
            logTracer.println("Iteration #" + loops + " Result: ");
            logTracer.println("\tClusters:");
            logTracer.println("\t\t" + joinClusters(ret.clusters));
            logTracer.println("\tCenters:");
            logTracer.println("\t\t" + stringify(ret.centers));
            logTracer.println("");

            Tracer.delay();
            // }

            if (loops >= 1000 || areCentersEqual(centers, ret.centers)) {
                return ret;
            }

            clusters = ret.clusters;
            centers = ret.centers;
        }
    }
}
