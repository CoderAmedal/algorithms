// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let tracer = Array2DTracer::new("Radix");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    let k = Array1D::new(10, Box::new(Integer::new(1, 999))).create_ints();
    let mut d: Vec<Vec<i64>> = vec![k, vec![0; 10], vec![0; 10]];
    tracer.set(&d);
    Tracer::delay();
    // }

    // logger {
    logger.println(format!("original array = [{}]", join(&d[0])));
    // }

    for exp in 0..3 {
        // logger {
        logger.println(format!("Digit: {}", exp));
        // }
        for i in 0..d[0].len() {
            let digit = digit(&d, i, exp);
            // visualize {
            tracer.select(0, i as i64, None, None);
            Tracer::delay();
            // }
            d[2][digit] += 1;
            // visualize {
            tracer.patch(2, digit as i64, Some(d[2][digit]));
            Tracer::delay();
            tracer.depatch(2, digit as i64);
            tracer.deselect(0, i as i64, None, None);
            // }
        }
        for i in 1..10 {
            // visualize {
            tracer.select(2, i as i64 - 1, None, None);
            Tracer::delay();
            // }
            d[2][i] += d[2][i - 1];
            // visualize {
            tracer.patch(2, i as i64, Some(d[2][i]));
            Tracer::delay();
            tracer.depatch(2, i as i64);
            tracer.deselect(2, i as i64 - 1, None, None);
            // }
        }
        for i in (0..d[0].len()).rev() {
            let digit = digit(&d, i, exp);
            // visualize {
            tracer.select(0, i as i64, None, None);
            Tracer::delay();
            // }
            d[2][digit] -= 1;
            // visualize {
            tracer.patch(2, digit as i64, Some(d[2][digit]));
            Tracer::delay();
            tracer.depatch(2, digit as i64);
            // }
            let target = d[2][digit] as usize;
            d[1][target] = d[0][i];
            // visualize {
            tracer.patch(1, target as i64, Some(d[1][target]));
            Tracer::delay();
            tracer.depatch(1, target as i64);
            tracer.deselect(0, i as i64, None, None);
            // }
        }
        for i in 0..d[0].len() {
            // visualize {
            tracer.select(1, i as i64, None, None);
            Tracer::delay();
            // }
            d[0][i] = d[1][i];
            // visualize {
            tracer.patch(0, i as i64, Some(d[0][i]));
            Tracer::delay();
            tracer.depatch(0, i as i64);
            tracer.deselect(1, i as i64, None, None);
            // }
        }
        for i in 0..10 {
            d[2][i] = 0;
            // visualize {
            tracer.patch(2, i as i64, Some(d[2][i]));
            Tracer::delay();
            tracer.depatch(2, i as i64);
            // }
        }
    }
    // logger {
    logger.println(format!("sorted array = [{}]", join(&d[0])));
    // }
}

fn digit(d: &[Vec<i64>], i: usize, exp: i32) -> usize {
    (d[0][i] / 10i64.pow(exp as u32) % 10) as usize
}

fn join(values: &[i64]) -> String {
    values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(", ")
}
