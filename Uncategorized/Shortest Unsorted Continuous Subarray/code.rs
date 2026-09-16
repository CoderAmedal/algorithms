// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let tracer = Array1DTracer::new("Sequence");
    let d = vec![2i64, 6, 4, 8, 10, 9, 15];
    tracer.set(&d);
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    Tracer::delay();
    // }

    find_unsorted_subarray(&tracer, &logger, &d);
}

fn find_unsorted_subarray(tracer: &Array1DTracer, logger: &LogTracer, nums: &[i64]) -> i64 {
    let mut min_value = i64::MAX;
    let mut max_value = i64::MIN;
    let mut flag = false;
    // visualize {
    let mut min_index: i64 = -1;
    let mut max_index: i64 = -1;
    // }

    for i in 1..nums.len() {
        // visualize {
        tracer.deselect(i as i64 - 2, Some(i as i64 - 1));
        tracer.select(i as i64 - 1, Some(i as i64));
        Tracer::delay();
        // }

        if nums[i] < nums[i - 1] {
            flag = true;
        }
        if flag {
            min_value = min_value.min(nums[i]);
            // visualize {
            if min_value == nums[i] {
                tracer.depatch(min_index);
                min_index = i as i64;
                tracer.patch(i as i64, None::<i64>);
            }
            Tracer::delay();
            // }
        }
    }

    // visualize {
    tracer.depatch(min_index);
    tracer.deselect(nums.len() as i64 - 2, None);
    tracer.deselect(nums.len() as i64 - 1, None);
    // }

    // logger {
    logger.println(format!("min = {}", min_value));
    Tracer::delay();
    // }

    flag = false;
    let mut i = nums.len() as i64 - 2;
    while i >= 0 {
        // visualize {
        tracer.deselect(i + 1, Some(i + 2));
        tracer.select(i, Some(i + 1));
        Tracer::delay();
        // }

        if nums[i as usize] > nums[i as usize + 1] {
            flag = true;
        }
        if flag {
            max_value = max_value.max(nums[i as usize]);
            // visualize {
            if max_value == nums[i as usize] {
                tracer.depatch(max_index);
                max_index = i;
                tracer.patch(i, None::<i64>);
            }
            Tracer::delay();
            // }
        }
        i -= 1;
    }

    // visualize {
    tracer.depatch(max_index);
    tracer.deselect(0, None);
    tracer.deselect(1, None);
    Tracer::delay();
    // }

    // logger {
    logger.println(format!("max = {}", max_value));
    // }

    let mut l: i64 = 0;
    while l < nums.len() as i64 {
        // visualize {
        tracer.deselect(l - 1, None);
        tracer.select(l, None);
        Tracer::delay();
        // }

        if min_value < nums[l as usize] {
            // visualize {
            tracer.patch(l, None::<i64>);
            Tracer::delay();
            // }
            break;
        }
        l += 1;
    }

    let mut r: i64 = nums.len() as i64 - 1;
    while r >= 0 {
        // visualize {
        tracer.deselect(r + 1, None);
        tracer.select(r, None);
        Tracer::delay();
        // }

        if max_value > nums[r as usize] {
            // visualize {
            tracer.patch(r, None::<i64>);
            Tracer::delay();
            // }
            break;
        }
        r -= 1;
    }

    // visualize {
    tracer.depatch(l);
    tracer.depatch(r);
    tracer.select(l, Some(r));
    Tracer::delay();
    // }

    let result = if r - l < 0 { 0 } else { r - l + 1 };

    // logger {
    logger.println(format!("result = {}", result));
    Tracer::delay();
    // }

    result
}
