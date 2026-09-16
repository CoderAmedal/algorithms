// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let chart = ChartTracer::new("Chart");
    let tracer = Array1DTracer::new("Array");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&chart, &tracer, &logger]));
    let d = Array1D::new(15, Box::new(Integer::new(0, 50)))
        .sorted(true)
        .create_ints();
    tracer.set(&d);
    tracer.chart(&chart);
    Tracer::delay();
    // }

    let element = d[Integer::new(0, d.len() as i64 - 1).create_int() as usize];

    // logger {
    logger.println(format!("Using iterative binary search to find {}", element));
    // }
    binary_search(&tracer, &logger, &d, element);
}

fn binary_search(
    tracer: &Array1DTracer,
    logger: &LogTracer,
    array: &[i64],
    element: i64,
) -> i64 {
    // array = sorted array, element = element to be found
    let mut min_index: i64 = 0;
    let mut max_index: i64 = array.len() as i64 - 1;

    while min_index <= max_index {
        let middle_index = (min_index + max_index) / 2;
        let test_element = array[middle_index as usize];

        // visualize {
        tracer.select(min_index, Some(max_index));
        Tracer::delay();
        tracer.patch(middle_index, None::<i64>);
        logger.println(format!("Searching at index: {}", middle_index));
        Tracer::delay();
        tracer.depatch(middle_index);
        tracer.deselect(min_index, Some(max_index));
        // }

        if test_element < element {
            // logger {
            logger.println("Going right.");
            // }
            min_index = middle_index + 1;
        } else if test_element > element {
            // logger {
            logger.println("Going left.");
            // }
            max_index = middle_index - 1;
        } else {
            // visualize {
            logger.println(format!("{} is found at position {}!", element, middle_index));
            tracer.select(middle_index, None::<i64>);
            // }

            return middle_index;
        }
    }

    // logger {
    logger.println(format!("{} is not found!", element));
    // }
    -1
}
