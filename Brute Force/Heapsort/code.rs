// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let chart = ChartTracer::new("Chart");
    let tracer = Array1DTracer::new("Array");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&chart, &tracer, &logger]));
    let mut d = Array1D::new(10, Box::new(Integer::default())).create_ints();
    tracer.set(&d);
    tracer.chart(&chart);
    Tracer::delay();
    // }

    // logger {
    logger.println(format!("Original array = [{}]", join(&d)));
    // }

    heap_sort(&tracer, &logger, &mut d);

    // logger {
    logger.println(format!("Final array = [{}]", join(&d)));
    // }
}

fn heapify(tracer: &Array1DTracer, logger: &LogTracer, array: &mut Vec<i64>, size: usize, root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;

    if left < size && array[left] > array[largest] {
        largest = left;
    }

    if right < size && array[right] > array[largest] {
        largest = right;
    }

    if largest != root {
        array.swap(root, largest);

        // visualize {
        tracer.patch(root as i64, Some(array[root]));
        tracer.patch(largest as i64, Some(array[largest]));
        logger.println(format!("Swapping elements : {} & {}", array[root], array[largest]));
        Tracer::delay();
        tracer.depatch(root as i64);
        tracer.depatch(largest as i64);
        // }

        heapify(tracer, logger, array, size, largest);
    }
}

fn heap_sort(tracer: &Array1DTracer, logger: &LogTracer, array: &mut Vec<i64>) {
    let size = array.len();

    let mut i = size / 2;
    while i > 0 {
        i -= 1;
        heapify(tracer, logger, array, size, i);
    }

    let mut j = size;
    while j > 0 {
        j -= 1;
        array.swap(0, j);

        // visualize {
        tracer.patch(0, Some(array[0]));
        tracer.patch(j as i64, Some(array[j]));
        logger.println(format!("Swapping elements : {} & {}", array[0], array[j]));
        Tracer::delay();
        tracer.depatch(0);
        tracer.depatch(j as i64);
        tracer.select(j as i64, None);
        Tracer::delay();
        // }

        heapify(tracer, logger, array, j, 0);

        // visualize {
        tracer.deselect(j as i64, None);
        // }
    }
}

fn join(values: &[i64]) -> String {
    values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(", ")
}
