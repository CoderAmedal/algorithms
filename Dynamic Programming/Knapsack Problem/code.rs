// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let val = vec![1i64, 4, 5, 7]; // The value of all available items
    let wt = vec![1i64, 3, 4, 5]; // The weights of available items
    let w = 7usize; // The maximum weight we can carry in our collection
    let n = val.len();
    let mut dp = vec![vec![0i64; w + 1]; n + 1];

    // define tracer variables {
    let tracer = Array2DTracer::new("Knapsack Table");
    let values_tracer = Array1DTracer::new("Values");
    let weights_tracer = Array1DTracer::new("Weights");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![
        &tracer,
        &values_tracer,
        &weights_tracer,
        &logger
    ]));
    tracer.set(&dp);
    values_tracer.set(&val);
    weights_tracer.set(&wt);
    Tracer::delay();
    // }

    for i in 0..=n {
        for j in 0..=w {
            if i == 0 || j == 0 {
                /*
                If we have no items or maximum weight we can take in collection is 0
                then the total weight in our collection is 0
                */
                dp[i][0] = 0;
                // visualize {
                tracer.patch(i as i64, j as i64, Some(dp[i][j]));
                Tracer::delay();
                tracer.depatch(i as i64, j as i64);
                // }
            } else if wt[i - 1] <= j as i64 {
                // take the current item in our collection
                // visualize {
                weights_tracer.select(i as i64 - 1, None);
                values_tracer.select(i as i64 - 1, None);
                Tracer::delay();
                tracer.select(i as i64 - 1, j as i64 - wt[i - 1], None, None);
                tracer.select(i as i64 - 1, j as i64, None, None);
                Tracer::delay();
                // }
                let a = val[i - 1] + dp[i - 1][j - wt[i - 1] as usize];
                let b = dp[i - 1][j];
                /*
                find the maximum of these two values
                and take which gives us a greater weight
                */
                if a > b {
                    dp[i][j] = a;
                    // visualize {
                    tracer.patch(i as i64, j as i64, Some(dp[i][j]));
                    Tracer::delay();
                    // }
                } else {
                    dp[i][j] = b;
                    // visualize {
                    tracer.patch(i as i64, j as i64, Some(dp[i][j]));
                    Tracer::delay();
                    // }
                }
                // visualize {
                // opt subproblem depatch
                tracer.depatch(i as i64, j as i64);
                tracer.deselect(i as i64 - 1, j as i64, None, None);
                tracer.deselect(i as i64 - 1, j as i64 - wt[i - 1], None, None);
                values_tracer.deselect(i as i64 - 1, None);
                weights_tracer.deselect(i as i64 - 1, None);
                // }
            } else {
                // leave the current item from our collection
                dp[i][j] = dp[i - 1][j];
                // visualize {
                tracer.patch(i as i64, j as i64, Some(dp[i][j]));
                Tracer::delay();
                tracer.depatch(i as i64, j as i64);
                // }
            }
        }
    }

    // logger {
    logger.println(format!(" Best value we can achieve is {}", dp[n][w]));
    // }
}
