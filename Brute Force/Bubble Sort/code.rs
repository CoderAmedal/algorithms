// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let chart = ChartTracer::new("Chart");
    let tracer = Array1DTracer::new("Array");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&chart, &tracer, &logger]));
    let mut d = Array1D::new(15, Box::new(Integer::default())).create_ints();
    tracer.set(&d);
    tracer.chart(&chart);
    Tracer::delay();
    // }

    // logger {
    logger.println(format!("original array = [{}]", join(&d)));
    // }
    let mut n = d.len();
    loop {
        let mut swapped = false;
        // visualize {
        tracer.select(n as i64 - 1, None);
        Tracer::delay();
        // }
        for i in 1..n {
            // visualize {
            tracer.select(i as i64, None);
            Tracer::delay();
            // }
            if d[i - 1] > d[i] {
                // logger {
                logger.println(format!("swap {} and {}", d[i - 1], d[i]));
                // }
                d.swap(i - 1, i);
                swapped = true;
                // visualize {
                tracer.patch(i as i64 - 1, Some(d[i - 1]));
                tracer.patch(i as i64, Some(d[i]));
                Tracer::delay();
                tracer.depatch(i as i64 - 1);
                tracer.depatch(i as i64);
                // }
            }
            // visualize {
            tracer.deselect(i as i64, None);
            // }
        }
        // visualize {
        tracer.deselect(n as i64 - 1, None);
        // }
        if !swapped {
            break;
        }
        n -= 1;
    }
    // logger {
    logger.println(format!("sorted array = [{}]", join(&d)));
    // }
}

fn join(values: &[i64]) -> String {
    values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(", ")
}
