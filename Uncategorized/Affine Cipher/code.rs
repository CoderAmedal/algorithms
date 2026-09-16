// import visualization libraries {
use algorithm_visualizer::*;
// }

const A_KEY: i64 = 5;
const B_KEY: i64 = 7;
const N: i64 = 26;

fn main() {
    // define tracer variables {
    let pt_tracer = Array1DTracer::new("Encryption");
    let ct_tracer = Array1DTracer::new("Decryption");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&pt_tracer, &ct_tracer, &logger]));
    let plain_text = "secret";
    pt_tracer.set(&plain_text.chars().map(|c| c.to_string()).collect::<Vec<_>>());
    Tracer::delay();
    // }

    let cipher_text = encrypt(&pt_tracer, &logger, plain_text);
    ct_tracer.set(&cipher_text.chars().map(|c| c.to_string()).collect::<Vec<_>>());
    decrypt(&ct_tracer, &logger, &cipher_text);
}

fn crypt_alpha(logger: &LogTracer, alpha: char) -> char {
    let index = alpha as i64 - 'a' as i64;
    let result = ((A_KEY * index) + B_KEY).rem_euclid(N);

    // logger {
    logger.println(format!("Index of {} = {}", alpha, index));
    // }

    (result + 'a' as i64) as u8 as char
}

fn encrypt(pt_tracer: &Array1DTracer, logger: &LogTracer, text: &str) -> String {
    let mut cypher_text = String::new();

    // logger {
    logger.println("Beginning Affine Encryption");
    logger.println("Encryption formula: <b>((keys.a * indexOfAlphabet) + keys.b) % N</b>");
    logger.println(format!("keys.a={}, keys.b={}, N={}", A_KEY, B_KEY, N));
    // }

    for (i, alpha) in text.chars().enumerate() {
        // visualize {
        pt_tracer.select(i as i64, None);
        Tracer::delay();
        pt_tracer.deselect(i as i64, None);
        // }

        let ch = crypt_alpha(logger, alpha);
        cypher_text.push(ch);

        // visualize {
        pt_tracer.patch(i as i64, Some(ch.to_string()));
        Tracer::delay();
        pt_tracer.depatch(i as i64);
        // }
    }

    cypher_text
}

fn decrypt(ct_tracer: &Array1DTracer, logger: &LogTracer, cypher_text: &str) -> String {
    let mut plain_text = String::new();
    let mut a_inverse = 0i64;
    for i in 1..N {
        if (A_KEY * i).rem_euclid(N) == 1 {
            a_inverse = i;
            break;
        }
    }

    // logger {
    logger.println(format!("a<sup>-1</sup> = {}", a_inverse));
    // }

    // logger {
    logger.println("Beginning Affine Decryption");
    logger.println("Decryption formula: <b>(a<sup>-1</sup> * (index - keys.b)) % N</b>");
    logger.println(format!("keys.b={}, N={}", B_KEY, N));
    // }

    for (i, alpha) in cypher_text.chars().enumerate() {
        // visualize {
        ct_tracer.select(i as i64, None);
        Tracer::delay();
        ct_tracer.deselect(i as i64, None);
        Tracer::delay();
        // }

        let index = alpha as i64 - 'a' as i64;
        let result = (a_inverse * (index - B_KEY)).rem_euclid(N);

        // logger {
        logger.println(format!("Index of {} = {}", alpha, index));
        // }

        let ch = (result + 'a' as i64) as u8 as char;
        plain_text.push(ch);

        // visualize {
        ct_tracer.patch(i as i64, Some(ch.to_string()));
        Tracer::delay();
        ct_tracer.depatch(i as i64);
        Tracer::delay();
        // }
    }

    plain_text
}
