// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // let pattern = "aab";
    // let text = "aabxaabxcaabxaabxay";
    let pattern = "abc";
    let text = "xabcabzabc";

    let length = pattern.len() + text.len() + 1;

    let mut z = vec![0i64; length];

    // define tracer variables {
    let text_tracer = Array1DTracer::new("text");
    let patt_tracer = Array1DTracer::new("pattern");
    let concat_tracer = Array1DTracer::new("concatenated string");
    let tracer = Array1DTracer::new("zArray");
    patt_tracer.set(&pattern);
    text_tracer.set(&text);
    tracer.set(&z);
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![
        &text_tracer,
        &patt_tracer,
        &concat_tracer,
        &tracer,
        &logger
    ]));
    Tracer::delay();
    // }

    let concat = format!("{}${}", pattern, text);
    // visualize {
    concat_tracer.set(&concat);
    // }
    let pat_len = pattern.len();
    create_zarr(&tracer, &concat_tracer, &logger, &mut z, &concat);
    // visualize {
    tracer.set(&z);
    // }
    // logger {
    logger.println("The Values in Z array equal to the length of the pattern indicates the index at which the pattern is present");
    logger.println("===================================");
    for i in 0..length {
        if z[i] == pat_len as i64 {
            let pos = i as i64 - (pat_len as i64 + 1);
            logger.println(format!("Pattern Found at index {}", pos));
        }
    }
    logger.println("===================================");
    // }
}

fn create_zarr(
    tracer: &Array1DTracer,
    concat_tracer: &Array1DTracer,
    logger: &LogTracer,
    z: &mut Vec<i64>,
    concat: &str,
) {
    let bytes = concat.as_bytes();
    let n = concat.len();
    let mut left = 0usize;
    let mut right = 0usize;
    for i in 1..n {
        // visualize {
        tracer.select(i as i64, None);
        Tracer::delay();
        // }
        if i > right {
            left = i;
            right = i;
            while right < n && bytes[right] == bytes[right - left] {
                // visualize {
                concat_tracer.patch(right as i64, None::<i64>);
                concat_tracer.select((right - left) as i64, None);
                logger.println(format!(
                    "{} (at index {}) is equal to {} (at index {})",
                    bytes[right] as char,
                    right,
                    bytes[right - left] as char,
                    right - left
                ));
                Tracer::delay();
                concat_tracer.depatch(right as i64);
                concat_tracer.deselect((right - left) as i64, None);
                // }
                right += 1;
            }
            // visualize {
            if right < n {
                concat_tracer.patch(right as i64, None::<i64>);
                concat_tracer.select((right - left) as i64, None);
                logger.println(format!(
                    "{} (at index {}) is NOT equal to {} (at index {})",
                    bytes[right] as char,
                    right,
                    bytes[right - left] as char,
                    right - left
                ));
                Tracer::delay();
                concat_tracer.depatch(right as i64);
                concat_tracer.deselect((right - left) as i64, None);
            }
            // }
            z[i] = (right - left) as i64;
            // logger {
            logger.println("--------------------------------");
            logger.println(format!(
                "Value of z[{}] = the length of the substring starting from {} which is also the prefix of the concatinated string(={})",
                i, i, right - left
            ));
            logger.println("--------------------------------");
            // }
            right -= 1;
        } else if z[i - left] < (right - i + 1) as i64 {
            // visualize {
            logger.println(format!(
                "The substring from index {} will not cross the right end.",
                i - left
            ));
            concat_tracer.patch((right - i + 1) as i64, None::<i64>);
            concat_tracer.select((i - left) as i64, None);
            Tracer::delay();
            // }
            z[i] = z[i - left];
            // visualize {
            concat_tracer.depatch((right - i + 1) as i64);
            concat_tracer.deselect((i - left) as i64, None);
            // }
        } else {
            // logger {
            logger.println(format!(
                "The substring from index {} will cross the right end.",
                i - left
            ));
            // }
            left = i;
            while right < n && bytes[right] == bytes[right - left] {
                // visualize {
                concat_tracer.patch(right as i64, None::<i64>);
                concat_tracer.select((right - left) as i64, None);
                logger.println(format!(
                    "{} (at index {}) is equal to {} (at index {})",
                    bytes[right] as char,
                    right,
                    bytes[right - left] as char,
                    right - left
                ));
                Tracer::delay();
                concat_tracer.depatch(right as i64);
                concat_tracer.deselect((right - left) as i64, None);
                // }
                right += 1;
            }
            // visualize {
            if right < n {
                concat_tracer.patch(right as i64, None::<i64>);
                concat_tracer.select((right - left) as i64, None);
                logger.println(format!(
                    "{} (at index {}) is NOT equal to {} (at index {})",
                    bytes[right] as char,
                    right,
                    bytes[right - left] as char,
                    right - left
                ));
                Tracer::delay();
                concat_tracer.depatch(right as i64);
                concat_tracer.deselect((right - left) as i64, None);
            }
            // }
            z[i] = (right - left) as i64;
            right -= 1;
            // logger {
            logger.println("--------------------------------");
            logger.println(format!(
                "Value of z[{}] = the length of the substring starting from {} which is also the prefix of the concatinated string(={})",
                i, i, right - left
            ));
            logger.println("--------------------------------");
            // }
        }
        // visualize {
        tracer.deselect(i as i64, None);
        tracer.set(&*z);
        // }
    }
}
