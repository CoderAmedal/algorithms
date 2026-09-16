// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let tracer = Array1DTracer::new("Sequence");
    Layout::set_root(&VerticalLayout::new(layout![&tracer]));
    let index = 15i64;
    let mut d: Vec<i64> = vec![1];
    for _ in 1..index {
        d.push(0);
    }
    tracer.set(&d);
    Tracer::delay();
    // }

    fact(&tracer, &mut d, index);
}

fn fact(tracer: &Array1DTracer, d: &mut Vec<i64>, num: i64) -> i64 {
    if num < 0 {
        return 0;
    }

    if num == 0 {
        return 1;
    }

    let res = num * fact(tracer, d, num - 1);

    d[(num - 1) as usize] = res;

    // visualize {
    tracer.select(num - 1, None);
    Tracer::delay();
    tracer.patch(num - 1, Some(d[(num - 1) as usize]));
    Tracer::delay();
    tracer.depatch(num - 1);
    tracer.deselect(num - 1, None);
    // }

    res
}
