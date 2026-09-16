// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let chart_tracer = ChartTracer::new("Chart");
    let array_tracer = Array1DTracer::new("Array");
    let buckets_tracer = Array2DTracer::new("Buckets");
    Layout::set_root(&VerticalLayout::new(layout![&chart_tracer, &array_tracer, &buckets_tracer]));
    // }

    // define input variables
    let n = 25usize; // the size of an array
    let k = 5usize; // the number of buckets
    let mut array = Array1D::new(n, Box::new(Integer::new(0, 999))).create_ints();

    // create K buckets
    let mut buckets: Vec<Vec<i64>> = vec![Vec::new(); k];
    // visualize {
    array_tracer.chart(&chart_tracer);
    array_tracer.set(&array);
    buckets_tracer.set(&buckets);
    Tracer::delay();
    // }

    // find the maximum value that will be used for distribution
    let maximum = *array.iter().max().unwrap();

    // distribute the elements into the buckets
    for i in 0..n {
        let number = array[i];
        let bucket_index = (number / (maximum + 1) * k as i64) as usize;
        buckets[bucket_index].push(number);
        // visualize {
        array_tracer.select(i as i64, None);
        buckets_tracer.patch(bucket_index as i64, (buckets[bucket_index].len() - 1) as i64, Some(number));
        Tracer::delay();
        buckets_tracer.depatch(bucket_index as i64, (buckets[bucket_index].len() - 1) as i64);
        // }

        // insertion sort within the bucket
        let mut j = buckets[bucket_index].len() - 1;
        while j > 0 && buckets[bucket_index][j - 1] > buckets[bucket_index][j] {
            buckets[bucket_index].swap(j - 1, j);
            // visualize {
            buckets_tracer.patch(bucket_index as i64, j as i64 - 1, Some(buckets[bucket_index][j - 1]));
            buckets_tracer.patch(bucket_index as i64, j as i64, Some(buckets[bucket_index][j]));
            Tracer::delay();
            buckets_tracer.depatch(bucket_index as i64, j as i64 - 1);
            buckets_tracer.depatch(bucket_index as i64, j as i64);
            // }
            j -= 1;
        }
        // visualize {
        array_tracer.deselect(i as i64, None);
        // }
    }

    // concatenate the buckets back into the array
    let mut i = 0usize;
    for bucket_index in 0..k {
        for j in 0..buckets[bucket_index].len() {
            array[i] = buckets[bucket_index][j];
            // visualize {
            array_tracer.patch(i as i64, Some(array[i]));
            buckets_tracer.select(bucket_index as i64, j as i64, None, None);
            Tracer::delay();
            buckets_tracer.deselect(bucket_index as i64, j as i64, None, None);
            array_tracer.depatch(i as i64);
            // }
            i += 1;
        }
    }
}
