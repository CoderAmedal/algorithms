// import visualization libraries {
use algorithm_visualizer::*;
// }

// /*
// For N>3 the time taken by this algorithm is sufficiently high
// Also it is not possible to visualise for N>6 due to stack overflow
// caused by large number of recursive calls
// */
const N: usize = 3;
const X: [i64; 8] = [2, 1, -1, -2, -2, -1, 1, 2];
const Y: [i64; 8] = [1, 2, 2, 1, -1, -2, -2, -1];

fn knight_tour(
    board_tracer: &Array2DTracer,
    pos_tracer: &Array1DTracer,
    log_tracer: &LogTracer,
    board: &mut Vec<Vec<i64>>,
    x: i64,
    y: i64,
    move_num: i64,
) -> bool {
    if move_num == (N * N) as i64 {
        return true;
    }

    for i in 0..8 {
        let next_x = x + X[i];
        let next_y = y + Y[i];

        // visualize {
        pos_tracer.patch(0, Some(next_x));
        Tracer::delay();
        pos_tracer.patch(1, Some(next_y));
        Tracer::delay();
        pos_tracer.depatch(0);
        pos_tracer.depatch(1);
        // }
        // /*
        // Check if knight is still in the board
        // Check that knight does not visit an already visited square
        // */
        if next_x >= 0
            && next_x < N as i64
            && next_y >= 0
            && next_y < N as i64
            && board[next_x as usize][next_y as usize] == -1
        {
            board[next_x as usize][next_y as usize] = move_num;

            // visualize {
            log_tracer.println(format!("Move to {},{}", next_x, next_y));
            board_tracer.patch(next_x, next_y, Some(move_num));
            Tracer::delay();
            board_tracer.depatch(next_x, next_y);
            board_tracer.select(next_x, next_y, None, None);
            // }

            let next_move_num = move_num + 1;
            if knight_tour(
                board_tracer,
                pos_tracer,
                log_tracer,
                board,
                next_x,
                next_y,
                next_move_num,
            ) {
                return true;
            }

            // logger {
            log_tracer.println(format!("No place to move from {},{}: Backtrack", next_x, next_y));
            // }
            board[next_x as usize][next_y as usize] = -1; // backtrack
            // visualize {
            board_tracer.patch(next_x, next_y, Some(-1i64));
            Tracer::delay();
            board_tracer.depatch(next_x, next_y);
            board_tracer.deselect(next_x, next_y, None, None);
            // }
        } else {
            // logger {
            log_tracer.println(format!("{},{} is not a valid move", next_x, next_y));
            // }
        }
    }
    false
}

fn main() {
    let mut board = vec![vec![-1i64; N]; N];
    let mut pos = vec![-1i64, -1i64];

    // define tracer variables {
    let board_tracer = Array2DTracer::new("Board");
    let pos_tracer = Array1DTracer::new("Knight Position");
    let log_tracer = LogTracer::new("Console");
    board_tracer.set(&board);
    pos_tracer.set(&pos);
    Layout::set_root(&VerticalLayout::new(layout![&board_tracer, &pos_tracer, &log_tracer]));
    Tracer::delay();
    // }

    board[0][0] = 0; // start from this position
    pos[0] = 0;
    pos[0] = 0;

    // visualize {
    board_tracer.patch(0, 0, Some(0i64));
    Tracer::delay();
    pos_tracer.patch(0, Some(0i64));
    Tracer::delay();
    pos_tracer.patch(1, Some(0i64));
    Tracer::delay();
    board_tracer.depatch(0, 0);
    board_tracer.depatch(0, 0);
    pos_tracer.depatch(0);
    pos_tracer.depatch(1);
    // }

    // logger {
    if !knight_tour(&board_tracer, &pos_tracer, &log_tracer, &mut board, 0, 0, 1) {
        log_tracer.println("Solution does not exist");
    } else {
        log_tracer.println("Solution found");
    }
    // }
}
