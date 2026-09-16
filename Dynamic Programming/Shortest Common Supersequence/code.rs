// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let string1 = "AGGTAB";
    let string2 = "GXTXAYB";
    let m = string1.len();
    let n = string2.len();
    let mut a = vec![vec![0i64; n + 1]; m + 1];

    // define tracer variables {
    let tracer1 = Array1DTracer::new("String 1");
    let tracer2 = Array1DTracer::new("String 2");
    let tracer3 = Array2DTracer::new("Memo Table");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer1, &tracer2, &tracer3, &logger]));
    tracer1.set(&string1);
    tracer2.set(&string2);
    tracer3.set(&a);
    Tracer::delay();
    // }

    // Fill memo table in bottom up manner
    for i in 0..=m {
        for j in 0..=n {
            if i == 0 {
                a[i][j] = j as i64;
            } else if j == 0 {
                a[i][j] = i as i64;
            } else if string1.as_bytes()[i - 1] == string2.as_bytes()[j - 1] {
                // visualize {
                tracer1.select(i as i64 - 1, None);
                Tracer::delay();
                tracer2.select(j as i64 - 1, None);
                Tracer::delay();
                tracer3.select(i as i64 - 1, j as i64 - 1, None, None);
                Tracer::delay();
                // }

                a[i][j] = a[i - 1][j - 1] + 1;

                // visualize {
                tracer1.deselect(i as i64 - 1, None);
                tracer2.deselect(j as i64 - 1, None);
                tracer3.deselect(i as i64 - 1, j as i64 - 1, None, None);
                // }
            } else {
                // visualize {
                tracer3.select(i as i64 - 1, j as i64, None, None);
                Tracer::delay();
                tracer3.select(i as i64, j as i64 - 1, None, None);
                Tracer::delay();
                // }

                if a[i - 1][j] < a[i][j - 1] {
                    a[i][j] = 1 + a[i - 1][j];
                } else {
                    a[i][j] = 1 + a[i][j - 1];
                }

                // visualize {
                tracer3.deselect(i as i64 - 1, j as i64, None, None);
                tracer3.deselect(i as i64, j as i64 - 1, None, None);
                // }
            }
            // visualize {
            tracer3.patch(i as i64, j as i64, Some(a[i][j]));
            Tracer::delay();
            tracer3.depatch(i as i64, j as i64);
            // }
        }
    }

    // logger {
    logger.println(format!("Shortest Common Supersequence is {}", a[m][n]));
    // }
}
