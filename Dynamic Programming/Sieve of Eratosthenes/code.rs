// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let n = 30usize;
    let mut a = Vec::new();
    let mut b = vec![0i64; n + 1];
    for i in 1..=n {
        a.push(i as i64);
    }

    // define tracer variables {
    let tracer = Array1DTracer::new("Sieve");
    tracer.set(&a);
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    Tracer::delay();
    // }

    // visualize {
    logger.println("1 is not prime");
    tracer.select(0, None);
    Tracer::delay();
    // }
    for i in 2..=n {
        if b[i] == 0 {
            // visualize {
            logger.println(format!("{} is not marked, so it is prime", i));
            // a[i-1] is prime mark by red indicators
            tracer.patch(i as i64 - 1, None::<i64>);
            Tracer::delay();
            // }
            let mut j = i + i;
            while j <= n {
                b[j] = 1; // a[j-1] is not prime, mark by blue indicators
                // visualize {
                logger.println(format!(
                    "{} is a multiple of {} so it is marked as composite",
                    j, i
                ));
                tracer.select(j as i64 - 1, None);
                Tracer::delay();
                // }
                j += i;
            }
            // visualize {
            tracer.depatch(i as i64 - 1);
            // }
        }
    }
    // logger {
    logger.println(format!(
        "The unmarked numbers are the prime numbers from 1 to {}",
        n
    ));
    // }
}
