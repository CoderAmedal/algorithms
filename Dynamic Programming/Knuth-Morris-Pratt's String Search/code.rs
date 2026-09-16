// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let string = "AAAABAABAAAABAAABAAAA";
    let pattern = "AAAABAAA";

    let mut next = vec![0i64; pattern.len()];
    // define tracer variables {
    let pattern_tracer = Array2DTracer::new("Pattern");
    let string_tracer = Array1DTracer::new("String");
    Layout::set_root(&VerticalLayout::new(layout![&pattern_tracer, &string_tracer]));
    let pattern_chars: Vec<String> = pattern.chars().map(|c| c.to_string()).collect();
    let next_display: Vec<String> = next.iter().map(|value| value.to_string()).collect();
    pattern_tracer.set(&vec![next_display, pattern_chars.clone(), pattern_chars.clone()]);
    string_tracer.set(&string);
    Tracer::delay();
    // }

    kmp(&string_tracer, &pattern_tracer, string, pattern, &mut next);
}

fn get_next(pattern_tracer: &Array2DTracer, pattern: &str, next: &mut Vec<i64>) {
    let chars: Vec<u8> = pattern.bytes().collect();
    let mut q = 1usize; // postfix pointer
    let mut k = 0usize; // prefix pointer
    // visualize {
    pattern_tracer.select(2, k as i64, None, None);
    // }
    while q < chars.len() {
        // visualize {
        pattern_tracer.select(1, q as i64, None, None);
        Tracer::delay();
        // }
        while k > 0 && chars[q] != chars[k] {
            // visualize {
            pattern_tracer.select(0, k as i64 - 1, None, None);
            Tracer::delay();
            pattern_tracer.deselect(2, k as i64, None, None);
            pattern_tracer.select(2, next[k - 1], None, None);
            Tracer::delay();
            pattern_tracer.deselect(0, k as i64 - 1, None, None);
            // }
            k = next[k - 1] as usize;
        }
        if chars[q] == chars[k] {
            // visualize {
            pattern_tracer.deselect(2, k as i64, None, None);
            pattern_tracer.select(2, k as i64 + 1, None, None);
            Tracer::delay();
            // }
            k += 1;
        }
        // visualize {
        pattern_tracer.patch(0, q as i64, Some(k as i64));
        Tracer::delay();
        pattern_tracer.depatch(0, q as i64);
        Tracer::delay();
        pattern_tracer.deselect(1, q as i64, None, None);
        // }
        next[q] = k as i64;
        q += 1;
    }
    // visualize {
    pattern_tracer.deselect(2, k as i64, None, None);
    let next_display: Vec<String> = next.iter().map(|value| value.to_string()).collect();
    let pattern_chars: Vec<String> = pattern.chars().map(|c| c.to_string()).collect();
    pattern_tracer.set(&vec![next_display, pattern_chars]);
    Tracer::delay();
    // }
}

fn kmp(
    string_tracer: &Array1DTracer,
    pattern_tracer: &Array2DTracer,
    string: &str,
    pattern: &str,
    next: &mut Vec<i64>,
) {
    let sbytes: Vec<u8> = string.bytes().collect();
    let pbytes: Vec<u8> = pattern.bytes().collect();
    let mut match_positions: Vec<usize> = Vec::new();

    let mut i = 0usize; // string pointer
    let mut k = 0usize; // pattern pointer
    get_next(pattern_tracer, pattern, next);
    while i < sbytes.len() {
        // visualize {
        string_tracer.select(i as i64, None);
        pattern_tracer.select(1, k as i64, None, None);
        Tracer::delay();
        // }
        while k > 0 && sbytes[i] != pbytes[k] {
            // visualize {
            pattern_tracer.select(0, k as i64 - 1, None, None);
            Tracer::delay();
            pattern_tracer.deselect(1, k as i64, None, None);
            pattern_tracer.select(1, next[k - 1], None, None);
            Tracer::delay();
            pattern_tracer.deselect(0, k as i64 - 1, None, None);
            // }
            k = next[k - 1] as usize;
        }
        if sbytes[i] == pbytes[k] {
            k += 1;
            if k == pbytes.len() {
                let match_start_position = i - pbytes.len() + 1;
                match_positions.push(match_start_position);
                // visualize {
                string_tracer.select(
                    match_start_position as i64,
                    Some((match_start_position + pbytes.len() - 1) as i64),
                );
                Tracer::delay();
                string_tracer.deselect(
                    match_start_position as i64,
                    Some((match_start_position + pbytes.len() - 1) as i64),
                );
                Tracer::delay();
                pattern_tracer.select(0, k as i64 - 1, None, None);
                Tracer::delay();
                pattern_tracer.deselect(1, k as i64 - 1, None, None);
                pattern_tracer.select(1, next[k - 1], None, None);
                Tracer::delay();
                pattern_tracer.deselect(0, k as i64 - 1, None, None);
                // }
                k = next[k - 1] as usize;
            } else {
                // visualize {
                pattern_tracer.deselect(1, k as i64 - 1, None, None);
                pattern_tracer.select(1, k as i64, None, None);
                Tracer::delay();
                // }
            }
        } else {
            // visualize {
            pattern_tracer.select(0, k as i64, None, None);
            Tracer::delay();
            // }
        }
        // visualize {
        pattern_tracer.deselect(0, k as i64, None, None);
        pattern_tracer.deselect(1, k as i64, None, None);
        string_tracer.deselect(i as i64, None);
        // }
        i += 1;
    }
    // visualize {
    for j in 0..match_positions.len() {
        string_tracer.select(
            match_positions[j] as i64,
            Some((match_positions[j] + pbytes.len() - 1) as i64),
        );
        Tracer::delay();
        string_tracer.deselect(
            match_positions[j] as i64,
            Some((match_positions[j] + pbytes.len() - 1) as i64),
        );
    }
    // }
}
