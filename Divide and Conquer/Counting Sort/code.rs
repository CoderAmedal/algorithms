// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let array_tracer = Array1DTracer::new("Array");
    let counts_tracer = Array1DTracer::new("Counts");
    let sorted_array_tracer = Array1DTracer::new("Sorted Array");
    Layout::set_root(&VerticalLayout::new(layout![&array_tracer, &counts_tracer, &sorted_array_tracer]));
    // }

    // define input variables
    let n = 20usize; // the size of an array
    let array = Array1D::new(n, Box::new(Integer::new(0, 9))).create_ints();

    // find the maximum value that will decide the size of counts array
    let maximum = *array.iter().max().unwrap();
    let mut counts = vec![0i64; (maximum + 1) as usize];
    // visualize {
    array_tracer.set(&array);
    counts_tracer.set(&counts);
    Tracer::delay();
    // }

    // store counts of each number
    for i in 0..n {
        let number = array[i];
        counts[number as usize] += 1;
        // visualize {
        array_tracer.select(i as i64, None);
        counts_tracer.patch(number, Some(counts[number as usize]));
        Tracer::delay();
        counts_tracer.depatch(number);
        array_tracer.deselect(i as i64, None);
        // }
    }

    // calculate the prefix sums
    for i in 1..=maximum {
        counts[i as usize] += counts[(i - 1) as usize];
        // visualize {
        counts_tracer.select(i - 1, None);
        counts_tracer.patch(i, Some(counts[i as usize]));
        Tracer::delay();
        counts_tracer.depatch(i);
        counts_tracer.deselect(i - 1, None);
        // }
    }

    // create a sorted array based on the prefix sums
    let mut sorted_array = vec![0i64; n];
    // visualize {
    sorted_array_tracer.set(&sorted_array);
    // }
    for i in (0..n).rev() {
        let number = array[i];
        let count = counts[number as usize];
        sorted_array[(count - 1) as usize] = number;
        counts[number as usize] -= 1;
        // visualize {
        array_tracer.select(i as i64, None);
        counts_tracer.select(number, None);
        sorted_array_tracer.patch(count - 1, Some(sorted_array[(count - 1) as usize]));
        counts_tracer.patch(number, Some(counts[number as usize]));
        Tracer::delay();
        sorted_array_tracer.depatch(count - 1);
        counts_tracer.depatch(number);
        counts_tracer.deselect(number, None);
        array_tracer.deselect(i as i64, None);
        // }
    }
}
