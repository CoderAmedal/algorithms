// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let chart = ChartTracer::new("Chart");
    let tracer = Array1DTracer::new("Array");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&chart, &tracer, &logger]));
    let mut d = Array1D::new(10, Box::new(Integer::default())).create_ints();
    tracer.set(&d);
    tracer.chart(&chart);
    Tracer::delay();
    // }

    // logger {
    logger.println(format!("original array = [{}]", join(&d)));
    // }
    let n = d.len();

    for i in 0..n - 1 {
        // logger {
        logger.println(format!("round {}", i + 1));
        // }
        let mut curr_max_idx = 0usize;
        let mut curr_max_val = d[i];
        for idx in 0..(n - i) {
            if d[i + idx] > curr_max_val {
                curr_max_idx = idx;
                curr_max_val = d[i + idx];
            }
        }
        if curr_max_idx != 0 {
            // if curr_max_idx == 0 the max element is already at the bottom, no flip required
            // logger {
            logger.println(format!("flip at {} (step 1)", curr_max_idx + i));
            // }
            flip(&tracer, &mut d, curr_max_idx + i, n);
            // logger {
            logger.println(format!("flip at {} (step 2)", i));
            // }
            flip(&tracer, &mut d, i, n);
        }
    }

    // logger {
    logger.println(format!("sorted array = [{}]", join(&d)));
    // }
}

fn flip(tracer: &Array1DTracer, d: &mut Vec<i64>, start: usize, n: usize) {
    // visualize {
    tracer.select(start as i64, Some(n as i64 - 1));
    Tracer::delay();
    // }
    let mut idx = 0;
    let mut i = start;
    while i < (start + n) / 2 {
        // visualize {
        tracer.select(i as i64, None);
        Tracer::delay();
        // }
        d.swap(i, n - idx - 1);
        // visualize {
        tracer.patch(i as i64, Some(d[i]));
        tracer.patch((n - idx - 1) as i64, Some(d[n - idx - 1]));
        Tracer::delay();
        tracer.depatch(i as i64);
        tracer.depatch((n - idx - 1) as i64);
        tracer.deselect(i as i64, None);
        // }
        idx += 1;
        i += 1;
    }
    // visualize {
    tracer.deselect(start as i64, Some(n as i64 - 1));
    // }
}

fn join(values: &[i64]) -> String {
    values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(", ")
}
