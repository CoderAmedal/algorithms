// import visualization libraries {
use algorithm_visualizer::*;
// }

fn solve(
    array1d_tracer: &Array1DTracer,
    log_tracer: &LogTracer,
    s: &[i64],
    d: i64,
    n: usize,
) {
    let mut sel = vec![0i64; n + 1];
    let mut k: i64 = 0;
    let mut sum: i64 = 0;
    let mut found = 0;
    sel[0] = 1;
    array1d_tracer.select(k, None);
    Tracer::delay();
    loop {
        if k < n as i64 && sel[k as usize] == 1 {
            if sum + s[k as usize] == d {
                found = 1;
                log_tracer.print("{");
                for i in 0..n {
                    if sel[i] == 1 {
                        log_tracer.print(format!("{}  ", s[i]));
                    }
                }
                log_tracer.println("}");
                sel[k as usize] = 0;
                Tracer::delay();
                array1d_tracer.deselect(k, None);
                Tracer::delay();
            } else if sum + s[k as usize] < d {
                sum += s[k as usize];
            } else {
                sel[k as usize] = 0;
                array1d_tracer.deselect(k, None);
                Tracer::delay();
            }
        } else {
            k -= 1;
            while k >= 0 && sel[k as usize] == 0 {
                k -= 1;
            }
            if k < 0 {
                break;
            }
            sel[k as usize] = 0;
            array1d_tracer.deselect(k, None);
            Tracer::delay();
            sum -= s[k as usize];
        }
        k += 1;
        if k < n as i64 {
            sel[k as usize] = 1;
            array1d_tracer.select(k, None);
            Tracer::delay();
        }
    }
    if found == 0 {
        log_tracer.println("Not possible subsets");
    }
}

fn main() {
    // define tracer variables {
    let array1d_tracer = Array1DTracer::new("Set");
    let log_tracer = LogTracer::new("Console");
    // }

    // visualize {
    Layout::set_root(&VerticalLayout::new(layout![&array1d_tracer, &log_tracer]));
    Tracer::delay();
    // }
    let n = 10usize;
    // Randomizing the array{
    let s = Array1D::new(n, Box::new(Integer::new(0, 29))).create_ints();
    let d = Integer::new(0, 99).create_int();
    // }

    log_tracer.print("The Given set is: ");
    for x in &s {
        log_tracer.print(format!("{},", x));
    }
    log_tracer.println(format!("\nDesired sum is:{}", d));
    log_tracer.println(format!("The possible subsets of sum {} are: ", d));
    array1d_tracer.set(&s);
    Tracer::delay();
    solve(&array1d_tracer, &log_tracer, &s, d, n);
}
