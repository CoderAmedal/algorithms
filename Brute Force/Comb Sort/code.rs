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
    let n = d.len();
    let shrink = 1.3f64; // set the gap shrink factor
    let mut gap_f = n as f64; // initialize gap size

    loop {
        // update the gap value for the next comb.
        gap_f = (gap_f / shrink).floor();
        let gap = if gap_f < 1.0 { 1 } else { gap_f as usize }; // minimum gap is 1

        let mut swapped = false; // initialize swapped
        // a single comb over the input list
        let mut i = 0;
        while i + gap < n {
            // visualize {
            tracer.select(i as i64, None);
            tracer.select((i + gap) as i64, None);
            Tracer::delay();
            // }

            if d[i] > d[i + gap] {
                // logger {
                logger.println(format!("swap {} and {}", d[i], d[i + gap])); // log swap event
                // }

                d.swap(i, i + gap);

                // visualize {
                tracer.patch(i as i64, Some(d[i]));
                tracer.patch((i + gap) as i64, Some(d[i + gap]));
                Tracer::delay();
                tracer.depatch(i as i64);
                tracer.depatch((i + gap) as i64);
                // }

                swapped = true; // Flag swapped has happened and list is not guaranteed sorted
            }
            // visualize {
            tracer.deselect(i as i64, None);
            tracer.deselect((i + gap) as i64, None);
            // }
            i += 1; // End of combing
        }

        if gap == 1 && !swapped {
            break;
        }
    }
}

fn join(values: &[i64]) -> String {
    values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(", ")
}
