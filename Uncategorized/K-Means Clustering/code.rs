// import visualization libraries {
use algorithm_visualizer::*;
// }

type Point = Vec<f64>;

// define helper functions {
fn shuffle(a: &[Point]) -> Vec<Point> {
    let mut array: Vec<Point> = a.to_vec();
    let mut copy: Vec<Point> = Vec::new();
    let mut n = array.len();

    while n > 0 {
        let i = Integer::new(0, (n - 1) as i64).create_int() as usize;
        n -= 1;
        copy.push(array.remove(i));
    }

    copy
}

fn choose_random_centers(data: &[Point], k: usize) -> Vec<Point> {
    shuffle(data).into_iter().take(k).collect()
}

fn pointify(p: &[f64]) -> String {
    format!("({}, {})", p[0], p[1])
}

fn arrayify(a: &[Point]) -> Vec<String> {
    a.iter().map(|p| pointify(p)).collect()
}

fn stringify(a: &[Point]) -> String {
    arrayify(a).join(", ")
}

fn distance(p1: &[f64], p2: &[f64]) -> f64 {
    (p1[0] - p2[0]).powi(2) + (p1[1] - p2[1]).powi(2)
}

fn mean(a: &[f64]) -> f64 {
    if a.is_empty() {
        0.0
    } else {
        a.iter().sum::<f64>() / a.len() as f64
    }
}

fn center_of_cluster(cluster: &[Point]) -> Point {
    let xs: Vec<f64> = cluster.iter().map(|p| p[0]).collect();
    let ys: Vec<f64> = cluster.iter().map(|p| p[1]).collect();
    vec![mean(&xs), mean(&ys)]
}

fn re_calculate_centers(clusters: &[Vec<Point>]) -> Vec<Point> {
    clusters.iter().map(|c| center_of_cluster(c)).collect()
}

fn are_centers_equal(c1: &[Point], c2: &[Point]) -> bool {
    c1 == c2
}

fn cluster(data: &[Point], centers: &[Point]) -> Vec<Vec<Point>> {
    let mut clusters: Vec<Vec<Point>> = vec![Vec::new(); centers.len()];

    for point in data {
        let mut min_distance = f64::INFINITY;
        let mut min_distance_index = 0usize;

        for (j, center) in centers.iter().enumerate() {
            let d = distance(point, center);

            if d < min_distance {
                min_distance = d;
                min_distance_index = j;
            }
        }

        clusters[min_distance_index].push(point.clone());
    }

    clusters
}
// }

fn recenter_and_cluster(data: &[Point], original_clusters: &[Vec<Point>]) -> (Vec<Point>, Vec<Vec<Point>>) {
    let centers = re_calculate_centers(original_clusters);
    let clusters = cluster(data, &centers);
    (centers, clusters)
}

fn improve(
    array2d_tracer: &Array2DTracer,
    log_tracer: &LogTracer,
    data: &[Point],
    mut clusters: Vec<Vec<Point>>,
    mut centers: Vec<Point>,
) -> (Vec<Vec<Point>>, Vec<Point>) {
    let mut loops = 0;

    loop {
        if loops >= 1000 {
            return (clusters, centers);
        }

        loops += 1;

        let (ret_centers, ret_clusters) = recenter_and_cluster(data, &clusters);

        // trace {
        let grid: Vec<Vec<String>> = ret_clusters.iter().map(|c| arrayify(c)).collect();
        array2d_tracer.set(&grid);

        log_tracer.println("");
        log_tracer.println(format!("Iteration #{} Result: ", loops));
        log_tracer.println("\tClusters:");
        log_tracer.println(format!(
            "\t\t{}",
            ret_clusters.iter().map(|c| stringify(c)).collect::<Vec<_>>().join("\n\t\t")
        ));
        log_tracer.println("\tCenters:");
        log_tracer.println(format!("\t\t{}", stringify(&ret_centers)));
        log_tracer.println("");

        Tracer::delay();
        // }

        if loops >= 1000 || are_centers_equal(&centers, &ret_centers) {
            return (ret_clusters, ret_centers);
        }

        clusters = ret_clusters;
        centers = ret_centers;
    }
}

fn main() {
    // define tracer variables {
    let array2d_tracer = Array2DTracer::new("Grid");
    let log_tracer = LogTracer::new("Console");
    // }

    // define input variables
    let raw = Array2D::new(15, 2, Box::new(Integer::new(1, 9))).create_ints();
    let unclustered_data: Vec<Point> = raw
        .iter()
        .map(|row| row.iter().map(|&v| v as f64).collect())
        .collect();
    let k = Integer::new(2, (unclustered_data.len() / 5).max(2) as i64).create_int() as usize;

    // visualize {
    Layout::set_root(&VerticalLayout::new(layout![&array2d_tracer, &log_tracer]));

    log_tracer.println(format!("Un-clustered data = {}", stringify(&unclustered_data)));
    array2d_tracer.set(&vec![arrayify(&unclustered_data)]);

    Tracer::delay();
    // }

    // Start with random centers
    let centers = choose_random_centers(&unclustered_data, k);

    // trace {
    log_tracer.println(format!("Initial random selected centers = {}", stringify(&centers)));

    Tracer::delay();
    // }

    // Cluster to the random centers
    let clusters = cluster(&unclustered_data, &centers);

    // trace {
    log_tracer.println(format!(
        "Initial clusters = \n\t{}",
        clusters.iter().map(|c| stringify(c)).collect::<Vec<_>>().join("\n\t")
    ));
    let grid: Vec<Vec<String>> = clusters.iter().map(|c| arrayify(c)).collect();
    array2d_tracer.set(&grid);

    Tracer::delay();
    // }

    // start iterations here
    let (ret_clusters, ret_centers) =
        improve(&array2d_tracer, &log_tracer, &unclustered_data, clusters, centers);

    // trace {
    Tracer::delay();

    log_tracer.println(format!(
        "Final clustered data = \n\t{}",
        ret_clusters.iter().map(|c| stringify(c)).collect::<Vec<_>>().join("\n\t")
    ));
    log_tracer.println(format!("Best centers = {}", stringify(&ret_centers)));
    let grid: Vec<Vec<String>> = ret_clusters.iter().map(|c| arrayify(c)).collect();
    array2d_tracer.set(&grid);
    Tracer::delay();
    // }
}
