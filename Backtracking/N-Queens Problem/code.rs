// import visualization libraries {
use algorithm_visualizer::*;
// }

const N: usize = 4; // just change the value of N and the visuals will reflect the configuration!

fn main() {
    let board = vec![vec![0i64; N]; N];
    let mut queens = vec![[-1i64, -1i64]; N];

    // define tracer variables {
    let board_tracer = Array2DTracer::new("Board");
    let queen_tracer = Array2DTracer::new("Queen Positions");
    let logger = LogTracer::new("Progress");
    Layout::set_root(&VerticalLayout::new(layout![&board_tracer, &queen_tracer, &logger]));

    board_tracer.set(&board);
    queen_tracer.set(&queens);
    logger.println(format!("N Queens: {}X{} matrix, {} queens", N, N, N));
    Tracer::delay();
    // }

    // logger {
    logger.println("Starting execution");
    // }
    nq(&board_tracer, &queen_tracer, &logger, &mut queens, 0, 0);
    // logger {
    logger.println("DONE");
    // }
}

fn valid_state(queens: &[[i64; 2]], row: usize, col: usize, current_queen: usize) -> bool {
    for q in 0..current_queen {
        let current_q = queens[q];
        if row as i64 == current_q[0]
            || col as i64 == current_q[1]
            || (current_q[0] - row as i64).abs() == (current_q[1] - col as i64).abs()
        {
            return false;
        }
    }
    true
}

fn nq(
    board_tracer: &Array2DTracer,
    queen_tracer: &Array2DTracer,
    logger: &LogTracer,
    queens: &mut Vec<[i64; 2]>,
    current_queen: usize,
    current_col: usize,
) -> bool {
    // logger {
    logger.println(format!(
        "Starting new iteration of nQueens () with currentQueen = {} & currentCol = {}",
        current_queen, current_col
    ));
    logger.println("------------------------------------------------------------------");
    // }
    if current_queen >= N {
        // logger {
        logger.println("The recursion has BOTTOMED OUT. All queens have been placed successfully");
        // }
        return true;
    }

    let mut found = false;
    let mut row = 0;
    while row < N && !found {
        // visualize {
        board_tracer.select(row as i64, current_col as i64, None, None);
        Tracer::delay();
        logger.println(format!(
            "Trying queen {} at row {} & col {}",
            current_queen, row, current_col
        ));
        // }

        if valid_state(queens, row, current_col, current_queen) {
            queens[current_queen][0] = row as i64;
            queens[current_queen][1] = current_col as i64;

            // visualize {
            queen_tracer.patch(current_queen as i64, 0, Some(row as i64));
            Tracer::delay();
            queen_tracer.patch(current_queen as i64, 1, Some(current_col as i64));
            Tracer::delay();
            queen_tracer.depatch(current_queen as i64, 0);
            Tracer::delay();
            queen_tracer.depatch(current_queen as i64, 1);
            Tracer::delay();
            // }

            found = nq(
                board_tracer,
                queen_tracer,
                logger,
                queens,
                current_queen + 1,
                current_col + 1,
            );
        }

        if !found {
            // visualize {
            board_tracer.deselect(row as i64, current_col as i64, None, None);
            Tracer::delay();
            logger.println(format!(
                "row {} & col {} didn't work out. Going down",
                row, current_col
            ));
            // }
        }
        row += 1;
    }

    found
}
