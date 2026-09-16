// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let grid_size = 10usize;
    let generations = 4;
    let fill_chance = 0.55;

    let mut g: Vec<Vec<char>> = Vec::new();
    for i in 0..grid_size {
        let mut row: Vec<char> = Vec::new();
        for j in 0..grid_size {
            let random = Double::new(0.0, 1.0).create_float();
            if random < fill_chance || i == 0 || j == 0 || i == grid_size - 1 || j == grid_size - 1 {
                row.push('#');
            } else {
                row.push('.');
            }
        }
        g.push(row);
    }

    // define tracer variables {
    let tracer = Array2DTracer::new("Grid");
    Layout::set_root(&VerticalLayout::new(layout![&tracer]));
    tracer.set(&g);
    Tracer::delay();
    // }

    // visualize {
    for gi in 0..g.len() {
        for gj in 0..g[gi].len() {
            if g[gi][gj] == '#' {
                tracer.patch(gi as i64, gj as i64, Some(g[gi][gj]));
            }
        }
    }
    // }

    for _iter in 0..generations {
        cellular_automata(&tracer, &mut g, '#', '.');
    }
}

fn cellular_automata(
    tracer: &Array2DTracer,
    g: &mut Vec<Vec<char>>,
    fill_shape: char,
    empty_shape: char,
) {
    let mut next_grid: Vec<Vec<char>> = Vec::new();

    for i in 0..g.len() {
        let mut row: Vec<char> = Vec::new();
        for j in 0..g[i].len() {
            let mut adj_count = 0;
            let mut two_away_count = 0;
            // look at the states of the neighboring cells
            for x in -2i64..=2 {
                for y in -2i64..=2 {
                    let ni = i as i64 + x;
                    let nj = j as i64 + y;
                    if ni >= 0 && ni < g.len() as i64 && nj >= 0 && nj < g[i].len() as i64 {
                        if !(x != 0 && y != 0) && g[ni as usize][nj as usize] == empty_shape {
                            if x == -2 || x == 2 || y == -2 || y == 2 {
                                two_away_count += 1;
                            } else {
                                adj_count += 1;
                            }
                        }
                    }
                }
            }
            // change the current cell's state according to these rules
            if adj_count >= 5 {
                row.push(fill_shape);
            } else if adj_count <= 1 {
                if two_away_count < 3 {
                    row.push(fill_shape);
                } else {
                    row.push(empty_shape);
                }
            } else {
                row.push(empty_shape);
            }
        }
        next_grid.push(row);
    }

    for i in 0..next_grid.len() {
        for j in 0..next_grid[i].len() {
            // visualize {
            tracer.depatch(i as i64, j as i64);
            tracer.select(i as i64, j as i64, None, None);
            Tracer::delay();
            // }
            g[i][j] = next_grid[i][j];
            // visualize {
            if g[i][j] == fill_shape {
                tracer.patch(i as i64, j as i64, Some(g[i][j]));
            } else {
                tracer.patch(i as i64, j as i64, Some(g[i][j]));
                tracer.depatch(i as i64, j as i64);
                tracer.deselect(i as i64, j as i64, None, None);
            }
            // }
        }
    }
}
