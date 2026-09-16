// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let mut a = Array1D::new(7, Box::new(Integer::default())).create_ints();
    let n = a.len();

    // define tracer variables {
    let tracer1 = Array1DTracer::new("Array");
    let tracer2 = Array2DTracer::new("Holes");
    let log_tracer = LogTracer::new("Console");
    Layout::set_root(&VerticalLayout::new(layout![&tracer1, &tracer2, &log_tracer]));
    tracer1.set(&a);
    Tracer::delay();
    // }

    let mut minimum = a[0];
    let mut maximum = a[0];

    for i in 1..n {
        if a[i] < minimum {
            minimum = a[i];
        }
        if a[i] > maximum {
            maximum = a[i];
        }
    }
    let range = (maximum - minimum + 1) as usize;

    let mut holes: Vec<Vec<i64>> = vec![Vec::new(); range];
    // visualize {
    tracer2.set(&holes);
    // }

    // logger {
    log_tracer.println("Filling up holes");
    // }
    for i in 0..n {
        // visualize {
        tracer1.select(i as i64, None);
        Tracer::delay();
        // }

        holes[(a[i] - minimum) as usize].push(a[i]);

        // visualize {
        tracer2.set(&holes);
        tracer1.deselect(i as i64, None);
        // }
    }

    // logger {
    log_tracer.println("Building sorted array");
    // }
    let mut k = 0usize;
    for i in 0..range {
        for j in 0..holes[i].len() {
            // visualize {
            tracer2.select(i as i64, j as i64, None, None);
            Tracer::delay();
            // }
            a[k] = holes[i][j];
            k += 1;
            // visualize {
            tracer1.patch((k - 1) as i64, Some(a[k - 1]));
            Tracer::delay();
            tracer2.deselect(i as i64, j as i64, None, None);
            tracer1.depatch((k - 1) as i64);
            // }
        }
    }

    // logger {
    log_tracer.println(format!("Sorted array is {:?}", a));
    // }
}
