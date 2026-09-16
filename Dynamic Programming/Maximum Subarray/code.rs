// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let d = vec![-2i64, -3, 4, -1, -2, 1, 5, -3];

    // define tracer variables {
    let tracer = Array1DTracer::new("Array");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    tracer.set(&d);
    Tracer::delay();
    // }

    let max_subarray_sum = max_subarray(&tracer, &logger, &d);

    // logger {
    logger.println(format!("Maximum Subarray's Sum is: {}", max_subarray_sum));
    // }
}

fn max_subarray(tracer: &Array1DTracer, logger: &LogTracer, array: &[i64]) -> i64 {
    let mut max_so_far = 0i64;
    let mut max_ending_here = 0i64;

    // logger {
    logger.println("Initializing maxSoFar = 0 & maxEndingHere = 0");
    // }

    for i in 0..array.len() {
        // visualize {
        tracer.select(i as i64, None);
        // }
        // logger {
        logger.println(format!("{} + {}", max_ending_here, array[i]));
        // }
        max_ending_here += array[i];
        // logger {
        logger.println(format!("=> {}", max_ending_here));
        // }

        if max_ending_here < 0 {
            // logger {
            logger.println("maxEndingHere is negative, set to 0");
            // }
            max_ending_here = 0;
        }

        if max_so_far < max_ending_here {
            // logger {
            logger.println(format!(
                "maxSoFar < maxEndingHere, setting maxSoFar to maxEndingHere ({})",
                max_ending_here
            ));
            // }
            max_so_far = max_ending_here;
        }

        // visualize {
        Tracer::delay();
        tracer.deselect(i as i64, None);
        // }
    }

    max_so_far
}
