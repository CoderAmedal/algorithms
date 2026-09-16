// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let mut a: Vec<i64> = vec![465, 255];

    // define tracer variables {
    let tracer = Array1DTracer::new("Euclidean Algorithm");
    tracer.set(&a);
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    Tracer::delay();
    // }

    // logger {
    logger.println(format!("Finding the greatest common divisor of {} and {}", a[0], a[1]));

    logger.println("Checking if first number is at most the second number");
    // }

    if a[0] > a[1] {
        a.swap(0, 1);
        // logger {
        logger.println("The first number is bigger than the second number. Switching the numbers.");
        // }
        // visualize {
        tracer.set(&a);
        Tracer::delay();
        // }
    }

    while a[0] > 0 {
        // logger {
        logger.println(format!("{} % {} = {}", a[1], a[0], a[1] % a[0]));
        logger.println("Switching a[1] with a[1]%a[0]");
        // }
        a[1] %= a[0];
        // visualize {
        tracer.patch(1, Some(a[1]));
        Tracer::delay();
        // }
        // logger {
        logger.println("Now switching the two values to keep a[0] < a[1]");
        // }
        a.swap(0, 1);
        // visualize {
        tracer.patch(0, Some(a[0]));
        tracer.patch(1, Some(a[1]));
        Tracer::delay();
        tracer.depatch(0);
        tracer.depatch(1);
        // }
    }

    // logger {
    logger.println(format!("The greatest common divisor is {}", a[1]));
    // }
}
