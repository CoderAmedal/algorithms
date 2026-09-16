// import visualization libraries {
use algorithm_visualizer::*;
// }

fn main() {
    let mut g: Vec<Vec<char>> = vec![
        vec!['#', '#', '#', '#', '#', '#', '#', '#', '#'],
        vec!['#', '-', '-', '-', '#', '-', '-', '-', '#'],
        vec!['#', '-', '-', '-', '#', '-', '-', '-', '#'],
        vec!['#', '-', '-', '#', '-', '-', '-', '-', '#'],
        vec!['#', '#', '#', '-', '-', '-', '#', '#', '#'],
        vec!['#', '-', '-', '-', '-', '#', '-', '-', '#'],
        vec!['#', '-', '-', '-', '#', '-', '-', '-', '#'],
        vec!['#', '-', '-', '-', '#', '-', '-', '-', '#'],
        vec!['#', '#', '#', '#', '#', '#', '#', '#', '#'],
    ];

    // define tracer variables {
    let tracer = Array2DTracer::new("Array2D");
    Layout::set_root(&VerticalLayout::new(layout![&tracer]));
    tracer.set(&g);
    Tracer::delay();
    // }

    flood_fill(&tracer, &mut g, 4, 4, '-', 'a');
}

fn flood_fill(
    tracer: &Array2DTracer,
    g: &mut Vec<Vec<char>>,
    i: i64,
    j: i64,
    old_color: char,
    new_color: char,
) {
    if i < 0 || i >= g.len() as i64 || j < 0 || j >= g[i as usize].len() as i64 {
        return;
    }
    if g[i as usize][j as usize] != old_color {
        return;
    }

    // set the color of node to newColor
    g[i as usize][j as usize] = new_color;

    // visualize {
    tracer.select(i, j, None::<i64>, None::<i64>);
    Tracer::delay();
    tracer.patch(i, j, Some(g[i as usize][j as usize]));
    Tracer::delay();
    // }

    // next step four-way
    flood_fill(tracer, g, i + 1, j, old_color, new_color);
    flood_fill(tracer, g, i - 1, j, old_color, new_color);
    flood_fill(tracer, g, i, j + 1, old_color, new_color);
    flood_fill(tracer, g, i, j - 1, old_color, new_color);
}
