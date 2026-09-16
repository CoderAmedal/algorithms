// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let a = [1i64, 3, 3, 2, 1, 1, 1];
    let n = a.len();

    // define tracer variables {
    let tracer = Array1DTracer::new("List of element");
    let logger = LogTracer::new("Console");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    tracer.set(&a);
    Tracer::delay();
    // }

    find_majority_element(&tracer, &logger, &a, n);
}

fn is_majority_element(tracer: &Array1DTracer, logger: &LogTracer, a: &[i64], n: usize, element: i64) -> bool {
    let mut count = 0i64;
    // logger {
    logger.println(format!("Verify majority element {}", element));
    // }
    for i in (0..n).rev() {
        // visualize {
        tracer.patch(i as i64, Some(a[i]));
        Tracer::delay();
        // }
        if a[i] == element {
            count += 1;
        } else {
            // visualize {
            tracer.depatch(i as i64);
            // }
        }
    }
    // logger {
    logger.println(format!("Count of our assumed majority element {}", count));
    // }
    if count > (n / 2) as i64 {
        // logger {
        logger.println("Our assumption was correct!");
        // }
        return true;
    }
    // logger {
    logger.println("Our assumption was incorrect!");
    // }
    false
}

fn find_probable_element(tracer: &Array1DTracer, logger: &LogTracer, a: &[i64], n: usize) -> i64 {
    let mut index = 0usize;
    let mut count = 1i64;
    // visualize {
    tracer.select(index as i64, None);
    Tracer::delay();
    // }
    // logger {
    logger.println(format!("Beginning with assumed majority element : {} count : {}", a[index], count));
    logger.println("--------------------------------------------------------");
    // }
    for i in 1..n {
        // visualize {
        tracer.patch(i as i64, Some(a[i]));
        Tracer::delay();
        // }
        if a[index] == a[i] {
            count += 1;
            // logger {
            logger.println(format!("Same as assumed majority element! Count : {}", count));
            // }
        } else {
            count -= 1;
            // logger {
            logger.println(format!("Not same as assumed majority element! Count : {}", count));
            // }
        }

        if count == 0 {
            // logger {
            logger.println("Wrong assumption in majority element");
            // }
            // visualize {
            tracer.deselect(index as i64, None);
            tracer.depatch(i as i64);
            // }
            index = i;
            count = 1;
            // visualize {
            tracer.select(i as i64, None);
            Tracer::delay();
            // }
            // logger {
            logger.println(format!("New assumed majority element!{} Count : {}", a[i], count));
            logger.println("--------------------------------------------------------");
            // }
        } else {
            // visualize {
            tracer.depatch(i as i64);
            // }
        }
    }
    // logger {
    logger.println(format!("Finally assumed majority element {}", a[index]));
    logger.println("--------------------------------------------------------");
    // }
    a[index]
}

fn find_majority_element(tracer: &Array1DTracer, logger: &LogTracer, a: &[i64], n: usize) {
    let element = find_probable_element(tracer, logger, a, n);
    // logger {
    if is_majority_element(tracer, logger, a, n, element) {
        logger.println(format!("Majority element is {}", element));
    } else {
        logger.println("No majority element");
    }
    // }
}
