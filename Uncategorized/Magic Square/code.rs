// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let n: i64 = 7;
    let mut a = vec![vec![0i64; n as usize]; n as usize];

    // define tracer variables {
    let tracer = Array2DTracer::new("Magic Square");
    let log_tracer = LogTracer::new("Console");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &log_tracer]));
    tracer.set(&a);
    Tracer::delay();
    // }

    let mut i: i64 = n / 2;
    let mut j: i64 = n - 1;

    let mut num: i64 = 1;
    while num <= n * n {
        // logger {
        log_tracer.println(format!("i = {}", i));
        log_tracer.println(format!("j = {}", j));
        // }

        if i == -1 && j == n {
            j = n - 2;
            i = 0;

            // logger {
            log_tracer.println("Changing : ");
            log_tracer.println(format!("i = {}", i));
            log_tracer.println(format!("j = {}", j));
            // }
        } else {
            if j == n {
                j = 0;
                // logger {
                log_tracer.println(format!("Changing : j = {}", j));
                // }
            }
            if i < 0 {
                i = n - 1;
                // logger {
                log_tracer.println(format!("Changing : i = {}", i));
                // }
            }
        }

        if a[i as usize][j as usize] > 0 {
            // logger {
            log_tracer.println(format!("Cell already filled : Changing i = {} j = {}", i, j));
            // }
            j -= 2;
            i += 1;
        } else {
            a[i as usize][j as usize] = num;
            num += 1;
            // visualize {
            tracer.patch(i, j, Some(a[i as usize][j as usize]));
            Tracer::delay();
            tracer.depatch(i, j);
            tracer.select(i, j, None, None);
            Tracer::delay();
            // }
            j += 1;
            i -= 1;
        }
    }

    // logger {
    log_tracer.println(format!("Magic Constant is {}", n * (n * n + 1) / 2));
    // }
}
