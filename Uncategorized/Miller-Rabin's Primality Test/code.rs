// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&logger]));
    Tracer::delay();
    // }

    for _ in 0..3 {
        let mut a = Integer::new(0, 299).create_int();
        if a % 2 == 0 {
            a += 1;
        }
        test_probably_prime(&logger, a, 5);
        // visualize {
        logger.println("----------");
        // }
    }

    test_probably_prime(&logger, 151, 5);
    // visualize {
    logger.println("----------");
    // }

    test_probably_prime(&logger, 199, 10);
}

// Utility function to do modular exponentiation.
// It returns (x^y) % p
fn power(x: i64, y: i64, p: i64) -> i64 {
    let mut res = 1i64;
    let mut x = x % p;
    let mut y = y;
    while y > 0 {
        // If y is odd, multiply x with result
        if y & 1 == 1 {
            res = (res * x) % p;
        }
        // y must be even now
        y >>= 1; // y = y/2
        x = (x * x) % p;
    }
    res
}

// Determine if N is prime using Miller-Rabin probabilistic algorithm
fn test_probably_prime(logger: &LogTracer, n: i64, k: i64) -> bool {
    let mut k = k;

    // visualize {
    logger.println(format!("==> Testing number {}", n));
    // }

    if n == 1 || n == 3 {
        // visualize {
        logger.println("==> Simple case, N is 1 or 3");
        // }
        return true;
    }
    if n % 2 == 0 {
        // visualize {
        logger.println(format!("==> Simple case, {} mod 2 = 0", n));
        // }
        return false;
    }

    // Write (n - 1) as 2^s * d
    let mut d = n - 1;
    while d % 2 == 0 {
        d /= 2;
    }
    // visualize {
    logger.println(format!("d = {}", d));
    // }

    let p = 100.0 * (1.0 - (1.0 / (4i64.pow(k as u32)) as f64));

    'witness: loop {
        // visualize {
        logger.println(format!("Remaining iterations: #{}", k));
        // }

        let a = 2 + Integer::new(0, n - 5).create_int();
        // visualize {
        logger.println(format!("--> first test with random = {}", a));
        // }

        // Compute a^d % n
        let mut x = power(a, d, n);

        if x == 1 || x == n - 1 {
            // visualize {
            logger.println("--> continue WitnessLoop, x = 1 or x = n-1");
            // }
            k -= 1;
            if k == 0 {
                break;
            }
            continue 'witness;
        }

        // visualize {
        logger.println("--> second test");
        // }

        // Keep squaring x while one of the following doesn't happen
        // (i)   d does not reach n-1
        // (ii)  (x^2) % n is not 1
        // (iii) (x^2) % n is not n-1
        let mut i = d;
        let mut continued = false;
        while i != n - 1 {
            x = (x * x) % n;
            i *= 2;

            if x == 1 {
                // visualize {
                logger.println(format!("--> exiting, {} is composite", n));
                // }
                return false;
            }

            if x == n - 1 {
                // visualize {
                logger.println("--> continue WitnessLoop");
                // }
                continued = true;
                break;
            }
        }

        if continued {
            k -= 1;
            if k == 0 {
                break;
            }
            continue 'witness;
        }

        // visualize {
        logger.println(format!("--> exiting, {} is composite 'cause (n-1) is reached", n));
        // }
        return false;
    }

    // visualize {
    logger.println(format!(
        "End of tests, {} is probably prime with probabilty of {}%",
        n, p
    ));
    // }
    true
}
