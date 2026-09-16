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
    let mut writes = 0; // number of writing performed

    for cycle_start in 0..n.saturating_sub(1) {
        let mut item = d[cycle_start];

        // find where to put the item
        let mut pos = cycle_start;
        // visualize {
        tracer.select(cycle_start as i64, None);
        // }

        for i in cycle_start + 1..n {
            // visualize {
            tracer.select(i as i64, None);
            Tracer::delay();
            tracer.deselect(i as i64, None);
            // }
            if d[i] < item {
                pos += 1;
            }
        }

        // if the item is already there, this is not a circle
        if pos == cycle_start {
            // visualize {
            tracer.deselect(cycle_start as i64, None);
            // }
            continue;
        }

        // otherwise put the item there or right after any duplicates
        while item == d[pos] {
            pos += 1;
        }

        // write item to new index and increment writes
        std::mem::swap(&mut d[pos], &mut item);
        writes += 1;

        // logger {
        if pos != cycle_start {
            logger.println(format!("Rewrite {} to index {}; the next value to rewrite is {}", d[pos], pos, item));
        } else {
            logger.println(format!("Rewrite {} to index {}", d[pos], pos));
        }
        // }
        // visualize {
        tracer.select(pos as i64, None);
        Tracer::delay();
        tracer.deselect(pos as i64, None);
        tracer.patch(pos as i64, Some(d[pos]));
        tracer.patch(cycle_start as i64, Some(d[cycle_start]));
        Tracer::delay();
        tracer.depatch(pos as i64);
        tracer.depatch(cycle_start as i64);
        // }

        // rotate the rest of the cycle
        while pos != cycle_start {
            pos = cycle_start;

            for i in cycle_start + 1..n {
                // visualize {
                tracer.select(i as i64, None);
                Tracer::delay();
                tracer.deselect(i as i64, None);
                // }
                if d[i] < item {
                    pos += 1;
                }
            }

            while item == d[pos] {
                pos += 1;
            }

            std::mem::swap(&mut d[pos], &mut item);

            // logger {
            if pos != cycle_start {
                logger.println(format!("Rewrite {} to index {}; the next value to rewrite is {}", d[pos], pos, item));
            } else {
                logger.println(format!("Rewrite {} to index {}", d[pos], pos));
            }
            // }
            // visualize {
            tracer.select(pos as i64, None);
            Tracer::delay();
            tracer.deselect(pos as i64, None);
            tracer.patch(pos as i64, Some(d[pos]));
            tracer.patch(cycle_start as i64, Some(d[cycle_start]));
            Tracer::delay();
            tracer.depatch(pos as i64);
            tracer.depatch(cycle_start as i64);
            // }

            writes += 1;
        }
    }

    // logger {
    logger.println(format!("Number of writes performed is {}", writes));
    // }
}

fn join(values: &[i64]) -> String {
    values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(", ")
}
