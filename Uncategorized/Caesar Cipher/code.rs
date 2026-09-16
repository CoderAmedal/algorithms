// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    // define tracer variables {
    let encrypt_tracer = Array1DTracer::new("Encryption");
    let decrypt_tracer = Array1DTracer::new("Decryption");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&encrypt_tracer, &decrypt_tracer, &logger]));
    let string = "hello! how are you doing?";
    let rotation = 5i64;
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    encrypt_tracer.set(&string.chars().map(|c| c.to_string()).collect::<Vec<_>>());
    Tracer::delay();
    // }

    let encrypted = encrypt(&encrypt_tracer, &logger, &alphabet, string, rotation);
    // logger {
    logger.println(format!("Encrypted result: {}", encrypted));
    // }

    decrypt_tracer.set(&encrypted.chars().map(|c| c.to_string()).collect::<Vec<_>>());
    let decrypted = decrypt(&decrypt_tracer, &logger, &alphabet, &encrypted, rotation);
    // logger {
    logger.println(format!("Decrypted result: {}", decrypted));
    // }
}

fn get_pos_up(pos: usize, alphabet_len: usize) -> usize {
    if pos == alphabet_len - 1 {
        0
    } else {
        pos + 1
    }
}

fn get_pos_down(pos: usize, alphabet_len: usize) -> usize {
    if pos == 0 {
        alphabet_len - 1
    } else {
        pos - 1
    }
}

fn get_next_char(logger: &LogTracer, alphabet: &[char], curr_char: char, direction: &str) -> char {
    let pos = alphabet.iter().position(|&c| c == curr_char).unwrap();
    let next_pos = if direction == "up" {
        get_pos_up(pos, alphabet.len())
    } else {
        get_pos_down(pos, alphabet.len())
    };
    let next_char = alphabet[next_pos];

    // logger {
    logger.println(format!("{} -> {}", curr_char, next_char));
    // }
    next_char
}

fn cipher(
    cipher_tracer: &Array1DTracer,
    logger: &LogTracer,
    alphabet: &[char],
    text: &str,
    rotation: i64,
    direction: &str,
) -> String {
    if text.is_empty() {
        return String::new();
    }

    let mut chars: Vec<char> = text.chars().collect();
    for i in 0..chars.len() {
        // visualize {
        Tracer::delay();
        // }

        let mut curr_char = chars[i];
        if alphabet.iter().any(|&c| c == curr_char) {
            let mut r = rotation;

            // logger {
            logger.println(format!("Rotating {} {} {} times", curr_char, direction, rotation));
            // }
            // visualize {
            cipher_tracer.select(i as i64, None);
            Tracer::delay();
            // }

            // perform given amount of rotations in the given direction
            while r > 0 {
                r -= 1;
                curr_char = get_next_char(logger, alphabet, curr_char, direction);
                // visualize {
                cipher_tracer.patch(i as i64, Some(curr_char.to_string()));
                Tracer::delay();
                // }
            }
        } else {
            // logger {
            logger.println("Ignore this character");
            // }
        }

        chars[i] = curr_char;
        // logger {
        logger.println(format!("Current result: {}", chars.iter().collect::<String>()));
        // }
    }

    chars.iter().collect()
}

fn encrypt(
    cipher_tracer: &Array1DTracer,
    logger: &LogTracer,
    alphabet: &[char],
    text: &str,
    rotation: i64,
) -> String {
    // logger {
    logger.println(format!("Encrypting: {}", text));
    // }
    cipher(cipher_tracer, logger, alphabet, text, rotation, "up")
}

fn decrypt(
    cipher_tracer: &Array1DTracer,
    logger: &LogTracer,
    alphabet: &[char],
    text: &str,
    rotation: i64,
) -> String {
    // logger {
    logger.println(format!("Decrypting: {}", text));
    // }
    cipher(cipher_tracer, logger, alphabet, text, rotation, "down")
}
