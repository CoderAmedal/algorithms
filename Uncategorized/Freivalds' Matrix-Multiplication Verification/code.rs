// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let a = vec![vec![2i64, 3], vec![3, 4]];
    let b = vec![vec![1i64, 0], vec![1, 2]];
    let c = vec![vec![6i64, 5], vec![8, 7]];

    // define tracer variables {
    let matrix_a_tracer = Array2DTracer::new("Matrix A");
    let matrix_b_tracer = Array2DTracer::new("Matrix B");
    let matrix_c_tracer = Array2DTracer::new("Matrix C");
    let random_vector_tracer = Array1DTracer::new("Random Vector");
    let result_vector_tracer = Array1DTracer::new("Result Vector");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![
        &matrix_a_tracer,
        &matrix_b_tracer,
        &matrix_c_tracer,
        &random_vector_tracer,
        &result_vector_tracer,
        &logger
    ]));
    matrix_a_tracer.set(&a);
    matrix_b_tracer.set(&b);
    matrix_c_tracer.set(&c);
    Tracer::delay();
    // }

    freivalds_algorithm(&random_vector_tracer, &result_vector_tracer, &logger, &a, &b, &c);
}

fn freivalds_algorithm(
    random_vector_tracer: &Array1DTracer,
    result_vector_tracer: &Array1DTracer,
    logger: &LogTracer,
    a: &[Vec<i64>],
    b: &[Vec<i64>],
    c: &[Vec<i64>],
) -> bool {
    let mut k = 5;
    let n = a.len();

    while k > 0 {
        k -= 1;
        // logger {
        logger.println(format!("Iterations remained: #{}", k));
        // }

        // Generate random vector
        let mut r: Vec<i64> = Vec::new();
        let mut p: Vec<i64> = Vec::new();
        for _ in 0..n {
            p.push(-1);
            r.push(Integer::new(0, 1).create_int());
        }
        // visualize {
        random_vector_tracer.set(&r);
        Tracer::delay();
        // }

        // Compute Br, Cr
        let mut br: Vec<i64> = Vec::new();
        let mut cr: Vec<i64> = Vec::new();
        for i in 0..n {
            let mut tmp_b = 0i64;
            let mut tmp_c = 0i64;
            for j in 0..n {
                tmp_b += r[j] * b[j][i];
                tmp_c += r[j] * c[j][i];
            }
            br.push(tmp_b);
            cr.push(tmp_c);
        }

        // Compute A * Br - Cr
        p = Vec::new();
        for i in 0..n {
            let mut tmp = 0i64;
            for _j in 0..n {
                tmp += (a[i][_j] * br[i]) - cr[i];
            }
            p.push(tmp);
        }
        // visualize {
        result_vector_tracer.set(&p);
        Tracer::delay();
        // }

        for i in 0..n {
            if p[i] != 0 {
                // logger {
                logger.println(format!("P[{}] !== 0 ({}), exit", i, p[i]));
                // }
                return false;
            }
        }

        // logger {
        logger.println("Result vector is identity, continue...");
        // }
    }

    true
}
