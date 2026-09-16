// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let tracer = Array1DTracer::new("Sequence");
    Layout::set_root(&VerticalLayout::new(layout![&tracer]));
    let index = 15usize;
    let mut d = vec![1i64];
    for _ in 1..index {
        d.push(0);
    }
    tracer.set(&d);
    Tracer::delay();
    // }

    for i in 1..index {
        d[i] = d[i - 1] * i as i64;
        // visualize {
        tracer.select(i as i64 - 1, None);
        Tracer::delay();
        tracer.patch(i as i64, Some(d[i]));
        Tracer::delay();
        tracer.depatch(i as i64);
        tracer.deselect(i as i64 - 1, None);
        // }
    }
}
