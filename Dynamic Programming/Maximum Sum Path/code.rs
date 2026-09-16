// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let d = Array2D::new(5, 5, Box::new(Integer::new(1, 5))).create_ints();
    let data_viewer = Array2DTracer::new("Data");
    let tracer = Array2DTracer::new("Results Table");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&data_viewer, &tracer, &logger]));
    data_viewer.set(&d);
    let mut dp = vec![vec![i64::MAX; d[0].len()]; d.len()];
    tracer.set(&dp);
    Tracer::delay();
    // }

    let n = dp.len();
    let m = dp[0].len();

    for i in 0..n {
        for j in 0..m {
            if i == 0 && j == 0 {
                update(&data_viewer, &tracer, &mut dp, i, j, d[i][j]);
            } else if i == 0 {
                // visualize {
                tracer.select(i as i64, j as i64 - 1, None, None);
                // }
                let value = dp[i][j - 1] + d[i][j];
                update(&data_viewer, &tracer, &mut dp, i, j, value);
                // visualize {
                tracer.deselect(i as i64, j as i64 - 1, None, None);
                // }
            } else if j == 0 {
                // visualize {
                tracer.select(i as i64 - 1, j as i64, None, None);
                // }
                let value = dp[i - 1][j] + d[i][j];
                update(&data_viewer, &tracer, &mut dp, i, j, value);
                // visualize {
                tracer.deselect(i as i64 - 1, j as i64, None, None);
                // }
            } else {
                // visualize {
                tracer.select(i as i64, j as i64 - 1, None, None);
                tracer.select(i as i64 - 1, j as i64, None, None);
                // }
                let value = dp[i][j - 1].max(dp[i - 1][j]) + d[i][j];
                update(&data_viewer, &tracer, &mut dp, i, j, value);
                // visualize {
                tracer.deselect(i as i64, j as i64 - 1, None, None);
                tracer.deselect(i as i64 - 1, j as i64, None, None);
                // }
            }
        }
    }
    // logger {
    logger.println(format!("max = {}", dp[n - 1][m - 1]));
    // }
}

fn update(
    data_viewer: &Array2DTracer,
    tracer: &Array2DTracer,
    dp: &mut Vec<Vec<i64>>,
    i: usize,
    j: usize,
    value: i64,
) {
    dp[i][j] = value;
    // visualize {
    data_viewer.select(i as i64, j as i64, None, None);
    Tracer::delay();
    tracer.patch(i as i64, j as i64, Some(dp[i][j]));
    Tracer::delay();
    tracer.depatch(i as i64, j as i64);
    data_viewer.deselect(i as i64, j as i64, None, None);
    // }
}
