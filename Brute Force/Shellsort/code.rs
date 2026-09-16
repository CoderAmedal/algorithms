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
    logger.println(format!("Original array = [{}]", join(&d)));
    // }
    let n = d.len();

    let mut gap = n / 2;
    while gap > 0 {
        // logger {
        logger.println("");
        logger.println(format!("Gap of {}", gap));
        // }
        for i in gap..n {
            // visualize {
            tracer.select(i as i64, None);
            tracer.select((i - gap) as i64, None);
            Tracer::delay();
            // }
            let k = d[i];
            // logger {
            logger.println(format!("Holding: {}", k));
            // }
            let mut j = i;
            while j >= gap && k < d[j - gap] {
                // logger {
                logger.println(format!("{} < {}", k, d[j - gap]));
                // }
                d[j] = d[j - gap];
                // visualize {
                tracer.patch(j as i64, Some(d[j]));
                Tracer::delay();
                tracer.depatch(j as i64);
                // }
                j -= gap;
            }
            let old = d[j];
            d[j] = k;
            // visualize {
            if old != k {
                tracer.patch(j as i64, Some(d[j]));
                Tracer::delay();
                tracer.depatch(j as i64);
                logger.println(format!("Swapped {} with {}", d[j], old));
            }

            tracer.deselect(i as i64, None);
            tracer.deselect((i - gap) as i64, None);
            // }
        }
        gap /= 2;
    }
    // logger {
    logger.println("");
    logger.println(format!("Sorted array = [{}]", join(&d)));
    // }
}

fn join(values: &[i64]) -> String {
    values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(", ")
}
