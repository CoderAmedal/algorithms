// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let seq = "BBABCBCAB";
    let n = seq.len();

    let mut l = vec![vec![0i64; n]; n];

    for i in 0..n {
        l[i][i] = 1;
    }

    // define tracer variables {
    let tracer = Array1DTracer::new("Input Text");
    let matrix = Array2DTracer::new("Matrix");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &matrix, &logger]));
    tracer.set(&seq);
    matrix.set(&l);
    Tracer::delay();
    // }

    let chars: Vec<u8> = seq.bytes().collect();

    // logger {
    logger.println("LPS for any string with length = 1 is 1");
    // }
    for i in 2..=n {
        // logger {
        logger.println("--------------------------------------------------");
        logger.println(format!("Considering a sub-string of length {}", i));
        logger.println("--------------------------------------------------");
        // }
        for j in 0..n - i + 1 {
            let k = j + i - 1;
            // visualize {
            tracer.select(j as i64, None);
            Tracer::delay();
            tracer.patch(k as i64, None::<i64>);
            Tracer::delay();
            // }

            // logger {
            logger.println(format!("Comparing {} and {}", chars[j] as char, chars[k] as char));
            // }

            if chars[j] == chars[k] && i == 2 {
                // logger {
                logger.println(format!(
                    "They are equal and size of the string in the interval{} to {} is 2, so the Longest Palindromic Subsequence in the Given range is 2",
                    j, k
                ));
                // }

                // visualize {
                matrix.patch(j as i64, k as i64, None::<i64>);
                Tracer::delay();
                // }

                l[j][k] = 2;
                // visualize {
                matrix.set(&l);

                matrix.depatch(j as i64, k as i64);
                Tracer::delay();
                // }
            } else if chars[j] == chars[k] {
                // logger {
                logger.println(format!(
                    "They are equal, so the Longest Palindromic Subsequence in the Given range is 2 + the Longest Increasing Subsequence between the indices {} to {}",
                    j + 1,
                    k - 1
                ));
                // }

                // visualize {
                matrix.patch(j as i64, k as i64, None::<i64>);
                Tracer::delay();
                matrix.select(j as i64 + 1, k as i64 - 1, None, None);
                Tracer::delay();
                // }

                l[j][k] = l[j + 1][k - 1] + 2;
                // visualize {
                matrix.set(&l);

                matrix.depatch(j as i64, k as i64);
                Tracer::delay();
                matrix.deselect(j as i64 + 1, k as i64 - 1, None, None);
                Tracer::delay();
                // }
            } else {
                // logger {
                logger.println(format!(
                    "They are NOT equal, so the Longest Palindromic Subsequence in the Given range is the maximum Longest Increasing Subsequence between the indices {} to {} and {} to {}",
                    j + 1,
                    k,
                    j,
                    k - 1
                ));
                // }
                // visualize {
                matrix.patch(j as i64, k as i64, None::<i64>);
                Tracer::delay();
                matrix.select(j as i64 + 1, k as i64, None, None);
                Tracer::delay();
                matrix.select(j as i64, k as i64 - 1, None, None);
                Tracer::delay();
                // }

                l[j][k] = l[j + 1][k].max(l[j][k - 1]);
                // visualize {
                matrix.set(&l);

                matrix.depatch(j as i64, k as i64);
                Tracer::delay();
                matrix.deselect(j as i64 + 1, k as i64, None, None);
                Tracer::delay();
                matrix.deselect(j as i64, k as i64 - 1, None, None);
                Tracer::delay();
                // }
            }
            // logger {
            logger.println("--------------------------------------------------");
            // }
            // visualize {
            tracer.deselect(j as i64, None);
            Tracer::delay();
            tracer.depatch(k as i64);
            Tracer::delay();
            // }
        }
    }
    // logger {
    logger.println(format!(
        "Longest Increasing Subsequence of the given string = L[0][{}]={}",
        n - 1,
        l[0][n - 1]
    ));
    // }
}
