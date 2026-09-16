// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let tracer = Array1DTracer::new("Array");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    let d = Array1D::new(20, Box::new(Integer::new(-5, 5))).create_ints();
    tracer.set(&d);
    Tracer::delay();
    // }

    let mut sum = d[0] + d[1] + d[2];
    let mut max = sum;
    // visualize {
    tracer.select(0, Some(2));
    logger.println(format!("sum = {}", sum));
    Tracer::delay();
    // }
    for i in 3..d.len() {
        sum += d[i] - d[i - 3];
        if max < sum {
            max = sum;
        }
        // visualize {
        tracer.deselect(i as i64 - 3, None);
        tracer.select(i as i64, None);
        logger.println(format!("sum = {}", sum));
        Tracer::delay();
        // }
    }
    // visualize {
    tracer.deselect(d.len() as i64 - 3, Some(d.len() as i64 - 1));
    logger.println(format!("max = {}", max));
    // }
}
