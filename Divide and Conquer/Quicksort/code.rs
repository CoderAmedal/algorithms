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

    let length = d.len();
    partition(&tracer, &mut d, 0, length as i64 - 1);
    // logger {
    logger.println(format!("sorted array = [{}]", join(&d)));
    // }
}

fn partition(tracer: &Array1DTracer, d: &mut Vec<i64>, mut low: i64, mut high: i64) {
    while high > low {
        let mut i = low;
        let mut j = high;
        let s = d[low as usize];
        while i < j {
            // visualize {
            tracer.select(high, None);
            tracer.select(low, None);
            Tracer::delay();
            // }
            while d[j as usize] > s {
                // visualize {
                tracer.select(j, None);
                Tracer::delay();
                tracer.deselect(j, None);
                // }
                j -= 1;
            }
            d[i as usize] = d[j as usize];
            // visualize {
            tracer.patch(i, Some(d[j as usize]));
            Tracer::delay();
            tracer.depatch(i);
            // }
            while s >= d[i as usize] && i < j {
                // visualize {
                tracer.select(i, None);
                Tracer::delay();
                tracer.deselect(i, None);
                // }
                i += 1;
            }
            d[j as usize] = d[i as usize];
            // visualize {
            tracer.patch(j, Some(d[i as usize]));
            Tracer::delay();
            tracer.depatch(j);
            tracer.deselect(high, None);
            tracer.deselect(low, None);
            // }
        }
        d[i as usize] = s;
        // visualize {
        tracer.patch(i, Some(s));
        Tracer::delay();
        tracer.depatch(i);
        // }
        partition(tracer, d, low, i - 1);
        low = i + 1;
    }
}

fn join(values: &[i64]) -> String {
    values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(", ")
}
