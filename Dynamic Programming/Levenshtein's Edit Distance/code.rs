// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let str1 = "stack";
    let str2 = "racket";
    let mut table = vec![vec![-1i64; str2.len() + 1]; str1.len() + 1];

    for i in 0..=str1.len() {
        table[i][0] = i as i64;
    }
    for i in 1..=str2.len() {
        table[0][i] = i as i64;
    }

    // define tracer variables {
    let tracer = Array2DTracer::new("Distance Table");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    tracer.set(&table);
    Tracer::delay();
    // }

    // logger {
    logger.println("Initialized DP Table");
    logger.println(format!("Y-Axis (Top to Bottom): {}", str1));
    logger.println(format!("X-Axis (Left to Right): {}", str2));
    // }

    let dist = edit_distance(&tracer, &logger, str1, str2, &mut table);

    // logger {
    logger.println(format!("Minimum Edit Distance: {}", dist));
    // }
}

fn edit_distance(
    tracer: &Array2DTracer,
    logger: &LogTracer,
    str1: &str,
    str2: &str,
    table: &mut Vec<Vec<i64>>,
) -> i64 {
    let s1: Vec<u8> = str1.bytes().collect();
    let s2: Vec<u8> = str2.bytes().collect();

    // display grid with words
    // logger {
    logger.println(format!("*** {}", s2.iter().map(|c| (*c as char).to_string()).collect::<Vec<_>>().join(" ")));
    for (index, item) in table.iter().enumerate() {
        let character = if index == 0 {
            "*".to_string()
        } else {
            (s1[index - 1] as char).to_string()
        };
        logger.println(format!("{}\t[{}]", character, join(item)));
    }
    // }

    // begin ED execution
    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            if s1[i - 1] == s2[j - 1] {
                // visualize {
                tracer.select(i as i64 - 1, j as i64 - 1, None, None);
                Tracer::delay();
                // }
                table[i][j] = table[i - 1][j - 1];
                // visualize {
                tracer.patch(i as i64, j as i64, Some(table[i][j]));
                Tracer::delay();
                tracer.depatch(i as i64, j as i64);
                tracer.deselect(i as i64 - 1, j as i64 - 1, None, None);
                // }
            } else {
                // visualize {
                tracer.select(i as i64 - 1, j as i64, None, None);
                tracer.select(i as i64, j as i64 - 1, None, None);
                tracer.select(i as i64 - 1, j as i64 - 1, None, None);
                Tracer::delay();
                // }
                table[i][j] = table[i - 1][j].min(table[i][j - 1]).min(table[i - 1][j - 1]) + 1;
                // visualize {
                tracer.patch(i as i64, j as i64, Some(table[i][j]));
                Tracer::delay();
                tracer.depatch(i as i64, j as i64);
                tracer.deselect(i as i64 - 1, j as i64, None, None);
                tracer.deselect(i as i64, j as i64 - 1, None, None);
                tracer.deselect(i as i64 - 1, j as i64 - 1, None, None);
                // }
            }
        }
    }

    // visualize {
    tracer.select(str1.len() as i64, str2.len() as i64, None, None);
    // }
    table[str1.len()][str2.len()]
}

fn join(values: &[i64]) -> String {
    values.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(", ")
}
