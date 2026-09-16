// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let chart = ChartTracer::new("Chart");
    let tracer = Array1DTracer::new("Array");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&chart, &tracer, &logger]));
    let mut d = Array1D::new(15, Box::new(Integer::default())).create_ints();
    tracer.set(&d);
    tracer.chart(&chart);
    Tracer::delay();
    // }

    // logger {
    logger.println(format!("original array = [{}]", join(&d)));
    // }

    let length = d.len();
    merge_sort(&tracer, &logger, &mut d, 0, length);

    // logger {
    logger.println(format!("sorted array = [{}]", join(&d)));
    // }
}

fn merge(tracer: &Array1DTracer, logger: &LogTracer, d: &mut Vec<i64>, start: usize, middle: usize, end: usize) {
    let left_size = middle - start;
    let right_size = end - middle;
    let max_size = left_size.max(right_size);
    let size = end - start;
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    for i in 0..max_size {
        if i < left_size {
            left.push(d[start + i]);
            // visualize {
            tracer.select((start + i) as i64, None);
            logger.println(format!("insert value into left array[{}] = {}", i, d[start + i]));
            Tracer::delay();
            // }
        }
        if i < right_size {
            right.push(d[middle + i]);
            // visualize {
            tracer.select((middle + i) as i64, None);
            logger.println(format!("insert value into right array[{}] = {}", i, d[middle + i]));
            Tracer::delay();
            // }
        }
    }
    // logger {
    logger.println(format!("left array = [{}], right array = [{}]", join(&left), join(&right)));
    // }

    let mut i = 0;
    while i < size {
        if !left.is_empty() && !right.is_empty() {
            if left[0] > right[0] {
                d[start + i] = right.remove(0);
                // logger {
                logger.println(format!("rewrite from right array[{}] = {}", i, d[start + i]));
                // }
            } else {
                d[start + i] = left.remove(0);
                // logger {
                logger.println(format!("rewrite from left array[{}] = {}", i, d[start + i]));
                // }
            }
        } else if !left.is_empty() {
            d[start + i] = left.remove(0);
            // logger {
            logger.println(format!("rewrite from left array[{}] = {}", i, d[start + i]));
            // }
        } else {
            d[start + i] = right.remove(0);
            // logger {
            logger.println(format!("rewrite from right array[{}] = {}", i, d[start + i]));
            // }
        }

        // visualize {
        tracer.deselect((start + i) as i64, None);
        tracer.patch((start + i) as i64, Some(d[start + i]));
        Tracer::delay();
        tracer.depatch((start + i) as i64);
        // }
        i += 1;
    }

    let temp_array: Vec<i64> = (start..end).map(|i| d[i]).collect();
    // logger {
    logger.println(format!("merged array = [{}]", join(&temp_array)));
    // }
}

fn merge_sort(tracer: &Array1DTracer, logger: &LogTracer, d: &mut Vec<i64>, start: usize, end: usize) {
    if end.saturating_sub(start) <= 1 {
        return;
    }
    let middle = (start + end + 1) / 2;

    merge_sort(tracer, logger, d, start, middle);
    merge_sort(tracer, logger, d, middle, end);

    // logger {
    logger.println(format!("divide left[{}, {}], right[{}, {}]", start, middle - 1, middle, end - 1));
    // }
    merge(tracer, logger, d, start, middle, end);
}

fn join(values: &[i64]) -> String {
    values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(", ")
}
