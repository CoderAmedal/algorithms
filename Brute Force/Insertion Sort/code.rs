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
    for i in 1..d.len() {
        let key = d[i];
        // visualize {
        logger.println(format!("insert {}", key));
        tracer.select(i as i64, None);
        Tracer::delay();
        // }
        let mut j = i as i64 - 1;
        while j >= 0 && d[j as usize] > key {
            d[(j + 1) as usize] = d[j as usize];
            // visualize {
            tracer.patch(j + 1, Some(d[(j + 1) as usize]));
            Tracer::delay();
            tracer.depatch(j + 1);
            // }
            j -= 1;
        }
        d[(j + 1) as usize] = key;
        // visualize {
        tracer.patch(j + 1, Some(d[(j + 1) as usize]));
        Tracer::delay();
        tracer.depatch(j + 1);
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
