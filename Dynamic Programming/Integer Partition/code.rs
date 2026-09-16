// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let tracer = Array2DTracer::new("Array2D");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    let integer = Integer::new(5, 14).create_int() as usize;
    let mut d = vec![vec![0i64; integer + 1]; integer + 1];
    for i in 0..=integer {
        d[i][0] = 1;
    }
    tracer.set(&d);
    Tracer::delay();
    // }

    // logger {
    logger.println(format!("Partitioning: {}", integer));
    // }
    partition(&logger, "", integer as i64, integer as i64);
    let part = integer_partition(&tracer, &mut d, integer);
    // logger {
    logger.println(part);
    // }
}

fn partition(logger: &LogTracer, a: &str, n: i64, p: i64) {
    // logger {
    if p == 0 {
        let joined = a.chars().map(|c| c.to_string()).collect::<Vec<_>>().join(", ");
        logger.println(format!("[{}]", joined));
    }
    // }
    else {
        if n > 1 {
            partition(logger, a, n - 1, p);
        }
        if n <= p {
            let new_a = format!("{}{}", n, a);
            partition(logger, &new_a, n, p - n);
        }
    }
}

fn integer_partition(tracer: &Array2DTracer, d: &mut Vec<Vec<i64>>, n: usize) -> i64 {
    // cycle through each cell of matrix
    for i in 1..=n {
        for j in 1..=n {
            if i > j {
                // visualize {
                tracer.select(i as i64, j as i64, None, None);
                Tracer::delay();
                // }
                // set cell to cell above it
                d[i][j] = d[i - 1][j];
                // visualize {
                tracer.patch(i as i64, j as i64, Some(d[i][j]));
                Tracer::delay();
                tracer.depatch(i as i64, j as i64);
                tracer.deselect(i as i64, j as i64, None, None);
                // }
            } else {
                // visualize {
                tracer.select(i as i64, j as i64, None, None);
                Tracer::delay();
                // }
                // grab above cell and add it to previous cell
                let above = d[i - 1][j];
                let left = d[i][j - i];
                d[i][j] = above + left;
                // visualize {
                tracer.patch(i as i64, j as i64, Some(d[i][j]));
                Tracer::delay();
                tracer.depatch(i as i64, j as i64);
                tracer.deselect(i as i64, j as i64, None, None);
                // }
            }
        }
    }
    d[n][n]
}
