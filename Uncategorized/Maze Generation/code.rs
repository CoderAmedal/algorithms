// import visualization libraries {
use algorithm_visualizer::*;
// }

struct DisjointSet {
    set: Vec<i64>,
    elements: usize,
}

impl DisjointSet {
    fn new() -> Self {
        DisjointSet {
            set: Vec::new(),
            elements: 0,
        }
    }

    fn add_elements(&mut self, number_of_elements: usize) {
        for _ in 0..number_of_elements {
            self.elements += 1;
            self.set.push(-1);
        }
    }

    fn find(&mut self, element: usize) -> usize {
        if self.set[element] < 0 {
            return element;
        }
        let parent = self.set[element] as usize;
        let root = self.find(parent);
        self.set[element] = root as i64;
        root
    }

    fn set_union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a != root_b {
            let new_size = self.set[root_a] + self.set[root_b];
            if self.compare_size(root_a, root_b) {
                self.set[root_b] = root_a as i64;
                self.set[root_a] = new_size;
            } else {
                self.set[root_a] = root_b as i64;
                self.set[root_b] = new_size;
            }
        }
    }

    fn compare_size(&self, a: usize, b: usize) -> bool {
        if self.set[a] == self.set[b] {
            true
        } else {
            self.set[a] < self.set[b]
        }
    }
}

// http://bost.ocks.org/mike/shuffle/
fn shuffle(array: &mut Vec<(usize, usize)>) {
    let mut m = array.len();
    // While there remain elements to shuffle...
    while m > 0 {
        // Pick a remaining element...
        let i = Integer::new(0, (m - 1) as i64).create_int() as usize;
        m -= 1;
        // And swap it with the current element.
        array.swap(m, i);
    }
}

fn main() {
    let n = 6usize; // rows (change these!)
    let m = 6usize; // columns (change these!)

    let h_end = m * 4 - (m - 1);
    let v_end = n * 3 - (n - 1);

    let mut g: Vec<Vec<String>> = Vec::new();

    for i in 0..v_end {
        // by row
        g.push(vec![" ".to_string(); h_end]);
        for j in 0..h_end {
            // by column
            g[i][j] = " ".to_string();

            if i == 0 && j == 0 {
                // top-left corner
                g[i][j] = "\u{250c}".to_string();
            } else if i == 0 && j == h_end - 1 {
                // top-right corner
                g[i][j] = "\u{2510}".to_string();
            } else if i == v_end - 1 && j == 0 {
                // bottom-left corner
                g[i][j] = "\u{2514}".to_string();
            } else if i == v_end - 1 && j == h_end - 1 {
                // bottom-right corner
                g[i][j] = "\u{2518}".to_string();
            } else if (j % 3 == 0) && (i % v_end != 0 && i != v_end - 1 && i % 2 == 1) {
                g[i][j] = "\u{2502}".to_string();
            } else if i % 2 == 0 {
                g[i][j] = "\u{2500}".to_string();
            }

            if m > 1 {
                // More than one column
                if j % 3 == 0 && j != 0 && j != h_end - 1 && i == 0 {
                    g[i][j] = "\u{252c}".to_string();
                }
                if j % 3 == 0 && j != 0 && j != h_end - 1 && i == v_end - 1 {
                    g[i][j] = "\u{2534}".to_string();
                }
            }

            if n > 1 {
                // More than one row
                if i % 2 == 0 && i != 0 && i != v_end - 1 && j == 0 {
                    g[i][j] = "\u{251c}".to_string();
                }
                if i % 2 == 0 && i != 0 && i != v_end - 1 && j == h_end - 1 {
                    g[i][j] = "\u{2524}".to_string();
                }
            }

            if n > 1 && m > 1 {
                // More than one row and column
                if i % 2 == 0 && j % 3 == 0 && i != 0 && j != 0 && i != v_end - 1 && j != h_end - 1 {
                    g[i][j] = "\u{253c}".to_string();
                }
            }
        }
    }

    // define tracer variables {
    let tracer = Array2DTracer::new("Maze");
    let logger = LogTracer::new("Log");
    Layout::set_root(&VerticalLayout::new(layout![&tracer, &logger]));
    tracer.set(&g);
    Tracer::delay();
    // }

    build_maze(&tracer, &logger, &mut g, n, m, h_end, v_end);
}

fn build_maze(
    tracer: &Array2DTracer,
    logger: &LogTracer,
    g: &mut Vec<Vec<String>>,
    n: usize,
    m: usize,
    h_end: usize,
    v_end: usize,
) {
    let mut my_set = DisjointSet::new();
    let width = m;
    let height = n;
    let mut set_size = 0usize;
    let mut graph = vec![vec![0usize; height]; width];
    let mut walls: Vec<(bool, bool)> = vec![(true, true); width * height];
    let mut right_walls: Vec<(usize, usize)> = Vec::new();
    let mut down_walls: Vec<(usize, usize)> = Vec::new();
    let mut location = 0usize;

    my_set.add_elements(width * height);

    // logger {
    logger.println("initializing grid (all walls are up)");
    // }
    // init 'graph'
    // each room has two walls, a down and right wall.
    for i in 0..width {
        for j in 0..height {
            graph[i][j] = location;

            walls[location] = (true, true);

            // If you can label the rooms with just 2 digits
            if width * height < 100 {
                let location_string = location.to_string();

                g[j * 2 + 1][i * 3 + 1] = location_string.chars().next().unwrap().to_string();
                g[j * 2 + 1][i * 3 + 2] = location_string
                    .chars()
                    .nth(1)
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| " ".to_string());

                // visualize {
                tracer.set(g);
                // }
            }

            right_walls.push((i, j));
            down_walls.push((i, j));
            location += 1;
        }
    }

    // logger {
    logger.println("shuffled the walls for random selection");
    // }
    // Randomly shuffle the walls
    shuffle(&mut right_walls);
    shuffle(&mut down_walls);

    // Picking random walls to remove
    while set_size != my_set.elements - 1 {
        let random_wall = Integer::new(1, 2).create_int();
        if random_wall == 1 && !down_walls.is_empty() {
            // Down wall
            let (ix, iy) = down_walls.pop().unwrap();
            let iy_down = iy + 1;
            if iy_down < height {
                let u = graph[ix][iy];
                let v = graph[ix][iy_down];
                // visualize {
                tracer.patch((iy * 2 + 1) as i64, (ix * 3 + 1) as i64, None::<i64>);
                tracer.patch((iy * 2 + 1) as i64, (ix * 3 + 2) as i64, None::<i64>);
                tracer.patch((iy_down * 2 + 1) as i64, (ix * 3 + 1) as i64, None::<i64>);
                tracer.patch((iy_down * 2 + 1) as i64, (ix * 3 + 2) as i64, None::<i64>);
                // }
                let root_u = my_set.find(u);
                let root_v = my_set.find(v);
                if root_u != root_v {
                    // logger {
                    logger.println(format!(
                        "Rooms: {} & {} now belong to the same set, delete wall between them",
                        u, v
                    ));
                    Tracer::delay();
                    // }
                    my_set.set_union(u, v);
                    set_size += 1;
                    // delete wall
                    walls[u].0 = false;
                } else {
                    // logger {
                    logger.println(format!(
                        "Rooms: {} & {} would create a cycle! This is not good!",
                        u, v
                    ));
                    Tracer::delay();
                    // }
                }
                // visualize {
                tracer.depatch((iy * 2 + 1) as i64, (ix * 3 + 1) as i64);
                tracer.depatch((iy * 2 + 1) as i64, (ix * 3 + 2) as i64);
                tracer.depatch((iy_down * 2 + 1) as i64, (ix * 3 + 1) as i64);
                tracer.depatch((iy_down * 2 + 1) as i64, (ix * 3 + 2) as i64);
                // }
            }
        } else if random_wall == 2 && !right_walls.is_empty() {
            // Right Wall
            let (ix, iy) = right_walls.pop().unwrap();
            let ix_right = ix + 1;
            if ix_right < width {
                let u = graph[ix][iy];
                let v = graph[ix_right][iy];
                // visualize {
                tracer.patch((iy * 2 + 1) as i64, (ix * 3 + 1) as i64, None::<i64>);
                tracer.patch((iy * 2 + 1) as i64, (ix * 3 + 2) as i64, None::<i64>);
                tracer.patch((iy * 2 + 1) as i64, (ix_right * 3 + 1) as i64, None::<i64>);
                tracer.patch((iy * 2 + 1) as i64, (ix_right * 3 + 2) as i64, None::<i64>);
                // }
                let root_u = my_set.find(u);
                let root_v = my_set.find(v);
                if root_u != root_v {
                    // logger {
                    logger.println(format!(
                        "Rooms: {} & {} now belong to the same set, delete wall between them",
                        u, v
                    ));
                    Tracer::delay();
                    // }
                    my_set.set_union(u, v);
                    set_size += 1;
                    // delete wall
                    walls[u].1 = false;
                } else {
                    // logger {
                    logger.println(format!(
                        "Rooms: {} & {} would create a cycle! This is not good!",
                        u, v
                    ));
                    Tracer::delay();
                    // }
                }
                // visualize {
                tracer.depatch((iy * 2 + 1) as i64, (ix * 3 + 1) as i64);
                tracer.depatch((iy * 2 + 1) as i64, (ix * 3 + 2) as i64);
                tracer.depatch((iy * 2 + 1) as i64, (ix_right * 3 + 1) as i64);
                tracer.depatch((iy * 2 + 1) as i64, (ix_right * 3 + 2) as i64);
                // }
            }
        }
    }

    // logger {
    logger.println("deleting the walls");
    // }
    // update deleted walls
    for i in 0..width {
        for j in 0..height {
            let current_wall = walls[graph[i][j]];

            if !current_wall.0 {
                g[j * 2 + 2][i * 3 + 1] = " ".to_string();
                g[j * 2 + 2][i * 3 + 2] = " ".to_string();
                // visualize {
                tracer.select((j * 2 + 2) as i64, (i * 3 + 1) as i64, None, None);
                Tracer::delay();
                tracer.select((j * 2 + 2) as i64, (i * 3 + 2) as i64, None, None);
                Tracer::delay();
                // }
            }

            if !current_wall.1 {
                g[j * 2 + 1][i * 3 + 3] = " ".to_string();
                // visualize {
                tracer.select((j * 2 + 1) as i64, (i * 3 + 3) as i64, None, None);
                Tracer::delay();
                // }
            }
            // visualize {
            tracer.set(g);
            // }
        }
    }

    // logger {
    logger.println("cleaning up the grid!");
    // }
    clean_up_grid(g, width, height, h_end, v_end);

    // Clear out walls for the start and end locations.
    let random_start = Integer::new(0, (width - 1) as i64).create_int() as usize;
    let random_end = Integer::new(0, (width - 1) as i64).create_int() as usize;

    // logger {
    logger.println("setting the Start (S) & End (E) locations");
    // }

    // Start Location
    g[0][random_start * 3 + 1] = " ".to_string();
    g[0][random_start * 3 + 2] = " ".to_string();
    g[1][random_start * 3 + 1] = "S".to_string();

    // End Location
    g[v_end - 1][random_end * 3 + 1] = " ".to_string();
    g[v_end - 1][random_end * 3 + 2] = " ".to_string();
    g[v_end - 2][random_end * 3 + 1] = "E".to_string();

    clean_up_start_location(g, random_start, v_end);
    clean_up_end_location(g, random_end, v_end);

    // logger {
    logger.println("maze is completed!");
    // }

    // set the data
    // visualize {
    tracer.set(g);
    // }
}

fn clean_up_start_location(g: &mut Vec<Vec<String>>, start: usize, _v_end: usize) {
    if g[0][start * 3] == "\u{252c}" && g[1][start * 3] == "\u{2502}" {
        g[0][start * 3] = "\u{2510}".to_string();
    }
    if g[0][start * 3 + 3] == "\u{252c}" && g[1][start * 3 + 3] == "\u{2502}" {
        g[0][start * 3 + 3] = "\u{250c}".to_string();
    }
    if g[0][start * 3] == "\u{250c}" {
        g[0][start * 3] = "\u{2502}".to_string();
    }
    if g[0][start * 3 + 3] == "\u{2510}" {
        g[0][start * 3 + 3] = "\u{2502}".to_string();
    }
}

fn clean_up_end_location(g: &mut Vec<Vec<String>>, end: usize, v_end: usize) {
    if g[v_end - 1][end * 3] == "\u{2534}" && g[v_end - 2][end * 3] == "\u{2502}" {
        g[v_end - 1][end * 3] = "\u{2518}".to_string();
    }
    if g[v_end - 1][end * 3 + 3] == "\u{2534}" && g[v_end - 2][end * 3 + 3] == "\u{2502}" {
        g[v_end - 1][end * 3 + 3] = "\u{2514}".to_string();
    }
    if g[v_end - 1][end * 3] == "\u{2514}" {
        g[v_end - 1][end * 3] = "\u{2502}".to_string();
    }
    if g[v_end - 1][end * 3 + 3] == "\u{2518}" {
        g[v_end - 1][end * 3 + 3] = "\u{2502}".to_string();
    }
}

fn clean_up_grid(
    g: &mut Vec<Vec<String>>,
    width: usize,
    height: usize,
    h_end: usize,
    v_end: usize,
) {
    // Remove room numbers
    for i in 0..width {
        for j in 0..height {
            g[j * 2 + 1][i * 3 + 1] = " ".to_string();
            g[j * 2 + 1][i * 3 + 2] = " ".to_string();
        }
    }

    // clean up grid for looks
    for i in 0..v_end {
        for j in 0..h_end {
            if g[i][j] == "\u{251c}" {
                if g[i][j + 1] == " " {
                    g[i][j] = "\u{2502}".to_string();
                }
            }

            if g[i][j] == "\u{2524}" {
                if g[i][j - 1] == " " {
                    g[i][j] = "\u{2502}".to_string();
                }
            }

            if g[i][j] == "\u{252c}" {
                if g[i + 1][j] == " " {
                    g[i][j] = "\u{2500}".to_string();
                }
            }

            if g[i][j] == "\u{2534}" {
                if g[i - 1][j] == " " {
                    g[i][j] = "\u{2500}".to_string();
                }
            }

            if g[i][j] == "\u{253c}" {
                if g[i][j + 1] == " "
                    && g[i - 1][j] == " "
                    && g[i][j - 1] != " "
                    && g[i + 1][j] != " "
                {
                    g[i][j] = "\u{2510}".to_string();
                } else if g[i][j - 1] == " "
                    && g[i - 1][j] == " "
                    && g[i + 1][j] != " "
                    && g[i][j + 1] != " "
                {
                    g[i][j] = "\u{250c}".to_string();
                } else if g[i][j - 1] == " "
                    && g[i + 1][j] == " "
                    && g[i - 1][j] != " "
                    && g[i][j + 1] != " "
                {
                    g[i][j] = "\u{2514}".to_string();
                } else if g[i][j + 1] == " "
                    && g[i + 1][j] == " "
                    && g[i - 1][j] != " "
                    && g[i][j - 1] != " "
                {
                    g[i][j] = "\u{2518}".to_string();
                } else if g[i][j + 1] == " "
                    && g[i][j - 1] == " "
                    && (g[i + 1][j] == " " || g[i - 1][j] == " ")
                {
                    g[i][j] = "\u{2502}".to_string();
                } else if g[i + 1][j] == " "
                    && g[i - 1][j] == " "
                    && (g[i][j - 1] == " " || g[i][j + 1] == " ")
                {
                    g[i][j] = "\u{2500}".to_string();
                } else if g[i][j + 1] == " " && g[i][j - 1] == " " {
                    g[i][j] = "\u{2502}".to_string();
                } else if g[i + 1][j] == " " && g[i - 1][j] == " " {
                    g[i][j] = "\u{2500}".to_string();
                } else if g[i + 1][j] == " "
                    && g[i - 1][j] != " "
                    && g[i][j - 1] != " "
                    && g[i][j + 1] != " "
                {
                    g[i][j] = "\u{2534}".to_string();
                } else if g[i - 1][j] == " "
                    && g[i + 1][j] != " "
                    && g[i][j + 1] != " "
                    && g[i][j - 1] != " "
                {
                    g[i][j] = "\u{252c}".to_string();
                } else if g[i][j + 1] == " "
                    && g[i - 1][j] != " "
                    && g[i + 1][j] != " "
                    && g[i][j - 1] != " "
                {
                    g[i][j] = "\u{2524}".to_string();
                } else if g[i][j - 1] == " "
                    && g[i - 1][j] != " "
                    && g[i + 1][j] != " "
                    && g[i][j + 1] != " "
                {
                    g[i][j] = "\u{251c}".to_string();
                }
            }
        }
    }
}
