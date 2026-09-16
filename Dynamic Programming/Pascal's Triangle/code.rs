// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let n = 9usize;
    let mut a: Vec<Vec<Option<i64>>> = vec![vec![None; n]; n];

    // define tracer variables {
    let tracer = Array2DTracer::new("Pascal's Triangle");
    Layout::set_root(&VerticalLayout::new(layout![&tracer]));
    tracer.set(&a);
    Tracer::delay();
    // }

    for i in 0..n {
        for j in 0..=i {
            if j == i || j == 0 {
                // First and last values in every row are 1
                a[i][j] = Some(1);

                // visualize {
                tracer.patch(i as i64, j as i64, a[i][j]);
                Tracer::delay();
                tracer.depatch(i as i64, j as i64);
                // }
            } else {
                // Other values are sum of values just above and left of above
                // visualize {
                tracer.select(i as i64 - 1, j as i64 - 1, None, None);
                Tracer::delay();
                tracer.select(i as i64 - 1, j as i64, None, None);
                Tracer::delay();
                // }

                a[i][j] = Some(a[i - 1][j - 1].unwrap() + a[i - 1][j].unwrap());

                // visualize {
                tracer.patch(i as i64, j as i64, a[i][j]);
                Tracer::delay();
                tracer.depatch(i as i64, j as i64);
                tracer.deselect(i as i64 - 1, j as i64 - 1, None, None);
                tracer.deselect(i as i64 - 1, j as i64, None, None);
                // }
            }
        }
    }
}
