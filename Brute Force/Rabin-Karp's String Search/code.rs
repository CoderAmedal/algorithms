// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let text: Vec<char> = "hello sir hello".chars().collect();
    let pattern: Vec<char> = "hello".chars().collect();

    let q = 101i64; // A prime number
    let d = 256i64; // number of characters in the input alphabet

    // define tracer variables {
    let logger = LogTracer::new("Log");
    let tracer1 = Array1DTracer::new("Text");
    let tracer2 = Array1DTracer::new("Pattern");
    Layout::set_root(&VerticalLayout::new(layout![&logger, &tracer1, &tracer2]));
    tracer1.set(&text);
    tracer2.set(&pattern);
    Tracer::delay();
    // }

    let n = text.len();
    let m = pattern.len();

    let mut hash_text = 0i64; // hash value for text
    let mut hash_pattern = 0i64; // hash value for pattern
    let mut h = 1i64;

    for _ in 0..(m - 1) {
        h = (h * d) % q;
    }

    for i in 0..m {
        hash_pattern = (d * hash_pattern + pattern[i] as i64) % q;
        hash_text = (d * hash_text + text[i] as i64) % q;
    }

    for i in 0..=(n - m) {
        // Check if hash values of current window of text matches
        // with hash values of pattern. If match is found then
        // check for characters one by one
        if hash_pattern == hash_text {
            let mut f = 0;
            // visualize {
            tracer1.select(i as i64, Some((i + m - 1) as i64));
            Tracer::delay();
            tracer2.select(0, Some((m - 1) as i64));
            Tracer::delay();
            // }
            for j in 0..m {
                // visualize {
                tracer1.patch((i + j) as i64, None::<i64>);
                Tracer::delay();
                tracer2.patch(j as i64, None::<i64>);
                Tracer::delay();
                // }
                if text[i + j] != pattern[j] {
                    f += 1;
                }
                // visualize {
                tracer1.depatch((i + j) as i64);
                tracer2.depatch(j as i64);
                // }
            }

            // visualize {
            if f == 0 {
                logger.println(format!(" Pattern found at index {}", i));
            }
            tracer1.deselect(i as i64, Some((i + m) as i64));
            tracer2.deselect(0, Some((m - 1) as i64));
            // }
        }

        // Calculate hash value for next window of text:
        if i < n - m {
            hash_text = (d * (hash_text - text[i] as i64 * h) + text[i + m] as i64) % q;

            // Convert negative value of hashText (if found) to positive
            if hash_text < 0 {
                hash_text += q;
            }
        }
    }
}
