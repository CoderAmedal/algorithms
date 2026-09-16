// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let n = 15usize;
    let mut a = vec![0i64; n];
    a[0] = 1; // By convention 1 is an ugly number

    let mut m = vec![2i64, 3, 5]; // multiples of 2, 3, 5 respectively
    let mut it = vec![0usize, 0, 0]; // iterators of 2, 3, 5 respectively

    // define tracer variables {
    let tracer = Array1DTracer::new("Ugly Numbers");
    let tracer2 = Array1DTracer::new("Multiples of 2, 3, 5");
    let tracer3 = Array1DTracer::new(" Iterators I0, I1, I2 ");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &tracer2, &tracer3, &logger]));
    tracer.set(&a);
    tracer2.set(&m);
    tracer3.set(&it);
    Tracer::delay();
    // }

    for i in 1..n {
        // next is minimum of m2, m3 and m5
        let next = if m[0] <= m[1] {
            if m[0] <= m[2] {
                m[0]
            } else {
                m[2]
            }
        } else if m[1] <= m[2] {
            m[1]
        } else {
            m[2]
        };
        // logger {
        logger.println(format!(" Minimum of {}, {}, {} : {}", m[0], m[1], m[2], next));
        // }
        a[i] = next;

        // visualize {
        tracer.patch(i as i64, Some(a[i]));
        Tracer::delay();
        tracer.depatch(i as i64);
        // }

        if next == m[0] {
            it[0] += 1;
            m[0] = a[it[0]] * 2;
            // visualize {
            tracer2.patch(0, Some(m[0]));
            Tracer::delay();
            tracer3.patch(0, Some(it[0] as i64));
            Tracer::delay();
            tracer2.depatch(0);
            tracer3.depatch(0);
            // }
        }
        if next == m[1] {
            it[1] += 1;
            m[1] = a[it[1]] * 3;
            // visualize {
            tracer2.patch(1, Some(m[1]));
            Tracer::delay();
            tracer3.patch(1, Some(it[1] as i64));
            Tracer::delay();
            tracer2.depatch(1);
            tracer3.depatch(1);
            // }
        }
        if next == m[2] {
            it[2] += 1;
            m[2] = a[it[2]] * 5;
            // visualize {
            tracer2.patch(2, Some(m[2]));
            Tracer::delay();
            tracer3.patch(2, Some(it[2] as i64));
            Tracer::delay();
            tracer2.depatch(2);
            tracer3.depatch(2);
            // }
        }
    }
}
