// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let tracer = Array1DTracer::new("Array");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    let a = Array1D::new(10, Box::new(Integer::new(0, 10))).create_ints();
    let mut lis = vec![1i64; a.len()];
    tracer.set(&a);
    Tracer::delay();
    // }

    // Initialize LIS values for all indexes
    for i in 0..a.len() {
        lis[i] = 1;
    }

    // logger {
    logger.println("Calculating Longest Increasing Subsequence values in bottom up manner ");
    // }
    // Compute optimized LIS values in bottom up manner
    for i in 1..a.len() {
        // visualize {
        tracer.select(i as i64, None);
        logger.println(format!(" LIS[{}] = {}", i, lis[i]));
        // }
        for j in 0..i {
            // visualize {
            tracer.patch(j as i64, None::<i64>);
            Tracer::delay();
            tracer.depatch(j as i64);
            // }
            if a[i] > a[j] && lis[i] < lis[j] + 1 {
                lis[i] = lis[j] + 1;
                // logger {
                logger.println(format!(" LIS[{}] = {}", i, lis[i]));
                // }
            }
        }
        // visualize {
        tracer.deselect(i as i64, None);
        // }
    }

    // Pick maximum of all LIS values
    // logger {
    logger.println("Now calculate maximum of all LIS values ");
    // }
    let mut max_value = lis[0];
    for i in 1..a.len() {
        if max_value < lis[i] {
            max_value = lis[i];
        }
    }
    // logger {
    logger.println(format!("Longest Increasing Subsequence = max of all LIS = {}", max_value));
    // }
}
