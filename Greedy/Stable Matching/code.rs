// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let a_keys = ["Flavio", "Stephen", "Albert", "Jack"];
    let b_keys = ["July", "Valentine", "Violet", "Summer"];
    // Rank lists as indices into `b_keys` / `a_keys` (matching the JS object order).
    let a_rank: Vec<Vec<usize>> = vec![
        vec![1, 0, 3, 2], // Flavio
        vec![3, 0, 1, 2], // Stephen
        vec![0, 2, 1, 3], // Albert
        vec![0, 2, 1, 3], // Jack
    ];
    let b_rank: Vec<Vec<usize>> = vec![
        vec![3, 1, 2, 0], // July
        vec![0, 3, 1, 2], // Valentine
        vec![3, 1, 0, 2], // Violet
        vec![1, 0, 2, 3], // Summer
    ];

    // define tracer variables {
    let tracer_a = Array1DTracer::new("A");
    let tracer_b = Array1DTracer::new("B");
    tracer_a.set(&a_keys);
    tracer_b.set(&b_keys);
    let log_tracer = LogTracer::new("Console");
    Layout::set_root(&VerticalLayout::new(layout![&tracer_a, &tracer_b, &log_tracer]));
    Tracer::delay();
    // }

    let n = a_keys.len();
    let mut a_stable: Vec<Option<usize>> = vec![None; n];
    let mut b_stable: Vec<Option<usize>> = vec![None; n];
    let mut a_ranks = a_rank.clone();

    loop {
        // Find an unstable A with remaining preferences.
        let mut a_index = None;
        for i in 0..n {
            if a_stable[i].is_none() && !a_ranks[i].is_empty() {
                a_index = Some(i);
                break;
            }
        }
        let ai = match a_index {
            Some(i) => i,
            None => break,
        };

        // logger {
        log_tracer.println(format!("Selecting {}", a_keys[ai]));
        Tracer::delay();
        // }

        let bj = a_ranks[ai].remove(0);
        let b_key = b_keys[bj];

        // logger {
        log_tracer.println(format!("--> Choicing {}", b_key));
        Tracer::delay();
        // }

        match b_stable[bj] {
            None => {
                // logger {
                log_tracer.println(format!("--> {} is not stable, stabilizing with {}", b_key, a_keys[ai]));
                Tracer::delay();
                // }

                a_stable[ai] = Some(bj);
                b_stable[bj] = Some(ai);

                // visualize {
                tracer_a.select(ai as i64, None);
                Tracer::delay();
                tracer_b.select(bj as i64, None);
                Tracer::delay();
                // }
            }
            Some(prev) => {
                let rank_a = b_rank[bj].iter().position(|&x| x == ai).unwrap();
                let rank_prev = b_rank[bj].iter().position(|&x| x == prev).unwrap();
                if rank_a < rank_prev {
                    // logger {
                    log_tracer.println(format!(
                        "--> {} is more stable with {} rather than {} - stabilizing again",
                        b_key, a_keys[ai], a_keys[prev]
                    ));
                    Tracer::delay();
                    // }

                    a_stable[prev] = None;
                    // visualize {
                    tracer_a.deselect(prev as i64, None);
                    Tracer::delay();
                    // }

                    a_stable[ai] = Some(bj);
                    b_stable[bj] = Some(ai);

                    // visualize {
                    tracer_a.select(ai as i64, None);
                    Tracer::delay();
                    tracer_b.select(bj as i64, None);
                    Tracer::delay();
                    // }
                }
            }
        }
    }
}
