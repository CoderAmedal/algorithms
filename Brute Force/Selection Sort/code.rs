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
    for i in 0..d.len() - 1 {
        let mut min_j = i;
        // visualize {
        tracer.select(i as i64, None);
        Tracer::delay();
        // }
        for j in i + 1..d.len() {
            // visualize {
            tracer.select(j as i64, None);
            Tracer::delay();
            // }
            if d[j] < d[min_j] {
                min_j = j;
                // visualize {
                tracer.patch(j as i64, None::<i64>);
                Tracer::delay();
                tracer.depatch(j as i64);
                // }
            }
            // visualize {
            tracer.deselect(j as i64, None);
            // }
        }
        if min_j != i {
            // logger {
            logger.println(format!("swap {} and {}", d[i], d[min_j]));
            // }
            d.swap(i, min_j);
            // visualize {
            tracer.patch(i as i64, Some(d[i]));
            tracer.patch(min_j as i64, Some(d[min_j]));
            Tracer::delay();
            tracer.depatch(i as i64);
            tracer.depatch(min_j as i64);
            // }
        }
        // visualize {
        tracer.deselect(i as i64, None);
        // }
    }
    // logger {
    logger.println(format!("sorted array = [{}]", join(&d)));
    // }
}

fn join(values: &[i64]) -> String {
    values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(", ")
}
