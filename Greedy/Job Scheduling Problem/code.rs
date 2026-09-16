// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let mut job_id = vec!["a", "b", "c", "d", "e"];
    let mut deadline = vec![2i64, 1, 2, 1, 3];
    let mut profit = vec![100i64, 19, 27, 25, 15];
    let n = deadline.len();
    // sort according to decreasing order of profit
    // Bubble sort implemented ... Implement a better algorithm for better performance
    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if profit[j] < profit[j + 1] {
                profit.swap(j, j + 1);
                deadline.swap(j, j + 1);
                job_id.swap(j, j + 1);
            }
        }
    }

    let mut slot = vec![0i64; n];
    let mut result: Vec<String> = vec!["-".to_string(); n];

    // define tracer variables {
    let tracer3 = Array1DTracer::new("Schedule");
    let tracer = Array1DTracer::new("Job Ids");
    let tracer1 = Array1DTracer::new("Deadlines");
    let tracer2 = Array1DTracer::new("Profit");
    Layout::set_root(&VerticalLayout::new(layout![&tracer3, &tracer, &tracer1, &tracer2]));
    tracer.set(&job_id);
    tracer1.set(&deadline);
    tracer2.set(&profit);
    tracer3.set(&result);
    Tracer::delay();
    // }

    // Initialise all slots to free
    for i in 0..n {
        slot[i] = 0;
    }

    // Iterate through all the given jobs
    for i in 0..n {
        // Start from the last possible slot. Find a slot for the job
        // visualize {
        tracer.select(i as i64, None);
        Tracer::delay();
        tracer1.select(i as i64, None);
        Tracer::delay();
        // }
        let upper = n.min(deadline[i] as usize);
        let mut j = upper as i64 - 1;
        while j >= 0 {
            if slot[j as usize] == 0 {
                // visualize {
                tracer3.patch(j, Some(job_id[i]));
                Tracer::delay();
                // }
                result[j as usize] = job_id[i].to_string();
                slot[j as usize] = 1;
                // visualize {
                tracer3.depatch(j);
                // }
                break;
            }
            j -= 1;
        }
        // visualize {
        tracer.deselect(i as i64, None);
        tracer1.deselect(i as i64, None);
        // }
    }
}
