// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let mut word = String::from("virgo");
    let mut suffix_array: Vec<(i64, String)> = (1..=word.len() as i64 + 1)
        .map(|i| (i, "-".to_string()))
        .collect();

    // define tracer variables {
    let sa_tracer = Array2DTracer::new("Suffix Array");
    let word_tracer = Array1DTracer::new("Given Word");
    let logger = LogTracer::new("Progress");
    Layout::set_root(&VerticalLayout::new(layout![&sa_tracer, &word_tracer, &logger]));

    sa_tracer.set(&suffix_array);
    let word_chars: Vec<String> = word.chars().map(|c| c.to_string()).collect();
    word_tracer.set(&word_chars);
    Tracer::delay();
    // }

    word.push('$'); // special character
    // logger {
    logger.println("Appended '$' at the end of word as terminating (special) character. Beginning filling of suffixes");
    // }

    let word_len = word.len();
    create_sa(&sa_tracer, &word_tracer, &mut suffix_array, &word);

    // logger {
    logger.println("Re-organizing Suffix Array in sorted order of suffixes using efficient sorting algorithm (O(N.log(N)))");
    // }

    suffix_array.sort_by(|a, b| {
        // logger {
        logger.println(format!(
            "The condition a [1] ({}) > b [1] ({}) is {}",
            a.1,
            b.1,
            a.1 > b.1
        ));
        // }
        a.1.cmp(&b.1)
    });

    // visualize {
    for i in 0..word_len {
        sa_tracer.patch(i as i64, 0, Some(suffix_array[i].0));
        sa_tracer.patch(i as i64, 1, Some(suffix_array[i].1.clone()));
        Tracer::delay();

        sa_tracer.depatch(i as i64, 0);
        sa_tracer.depatch(i as i64, 1);
    }
    // }
}

fn select_suffix(word_tracer: &Array1DTracer, word: &str, start: usize) {
    let mut i = start;
    let mut c = start;

    while i < word.len() - 1 {
        // visualize {
        word_tracer.select(i as i64, None);
        // }
        i += 1;
    }
    // visualize {
    Tracer::delay();
    // }

    while c < word.len() - 1 {
        // visualize {
        word_tracer.deselect(c as i64, None);
        // }
        c += 1;
    }
    // visualize {
    Tracer::delay();
    // }
}

fn create_sa(
    sa_tracer: &Array2DTracer,
    word_tracer: &Array1DTracer,
    suffix_array: &mut Vec<(i64, String)>,
    word: &str,
) {
    for i in 0..word.len() {
        suffix_array[i].1 = word[i..].to_string();

        select_suffix(word_tracer, word, i);
        // visualize {
        sa_tracer.patch(i as i64, 1, Some(suffix_array[i].1.clone()));
        Tracer::delay();
        sa_tracer.depatch(i as i64, 1);
        Tracer::delay();
        // }
    }
}
