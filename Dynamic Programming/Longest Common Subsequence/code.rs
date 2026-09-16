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

    let s1: Vec<u8> = string1.bytes().collect();
    let s2: Vec<u8> = string2.bytes().collect();

    // Build the memo table in bottom up fashion
    for i in 0..=m {
        for j in 0..=n {
            if i == 0 || j == 0 {
                a[i][j] = 0;
            } else if s1[i - 1] == s2[j - 1] {
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

                if a[i - 1][j] > a[i][j - 1] {
                    a[i][j] = a[i - 1][j];
                } else {
                    a[i][j] = a[i][j - 1];
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

    let mut final_string = String::new();
    let mut i = m;
    let mut j = n;
    while i >= 1 && j >= 1 {
        // visualize {
        tracer3.select(i as i64, j as i64, None, None);
        Tracer::delay();
        // }
        if s1[i - 1] == s2[j - 1] {
            // visualize {
            tracer1.select(i as i64 - 1, None);
            Tracer::delay();
            tracer2.select(j as i64 - 1, None);
            Tracer::delay();
            // }

            final_string = format!("{}{}", s1[i - 1] as char, final_string);
            i -= 1;
            j -= 1;
        } else if a[i - 1][j] > a[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    // logger {
    logger.println(format!("Longest Common Subsequence Length is {}", a[m][n]));
    logger.println(format!("Longest Common Subsequence is {}", final_string));
    // }
}
