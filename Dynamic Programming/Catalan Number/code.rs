// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let n = 10usize;
    let mut a = vec![0i64; n + 1];

    // define tracer variables {
    let tracer = Array1DTracer::new(" Catalan Numbers ");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    tracer.set(&a);
    Tracer::delay();
    // }

    a[0] = 1;
    // visualize {
    tracer.patch(0, Some(a[0]));
    Tracer::delay();
    tracer.depatch(0);
    // }
    a[1] = 1;
    // visualize {
    tracer.patch(1, Some(a[1]));
    Tracer::delay();
    tracer.depatch(1);
    // }

    for i in 2..=n {
        for j in 0..i {
            a[i] += a[j] * a[i - j - 1];
            // visualize {
            tracer.select(j as i64, None);
            Tracer::delay();
            tracer.select(i as i64 - j as i64 - 1, None);
            Tracer::delay();
            tracer.patch(i as i64, Some(a[i]));
            Tracer::delay();
            tracer.deselect(j as i64, None);
            tracer.deselect(i as i64 - j as i64 - 1, None);
            tracer.depatch(i as i64);
            // }
        }
    }

    // visualize {
    logger.println(format!(" The {}th Catalan Number is {}", n, a[n]));
    tracer.select(n as i64, None);
    Tracer::delay();
    // }
}
