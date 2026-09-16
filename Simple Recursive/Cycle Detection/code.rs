// import visualization libraries {
use algorithm_visualizer::*;
// }


// define tracer variables {
fn main() {
    let next: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 2];
    let head = 0usize;

    let graph_tracer = GraphTracer::new("Linked List");
    graph_tracer.directed(Some(true));
    let log_tracer = LogTracer::new("Console");
    Layout::set_root(&VerticalLayout::new(layout![&graph_tracer, &log_tracer]));

    for node in 0..=6i64 {
        graph_tracer.add_node(node, None::<i64>, None, None);
    }
    graph_tracer.add_edge(0, 1, None::<i64>);
    graph_tracer.add_edge(1, 2, None::<i64>);
    graph_tracer.add_edge(2, 3, None::<i64>);
    graph_tracer.add_edge(3, 4, None::<i64>);
    graph_tracer.add_edge(4, 5, None::<i64>);
    graph_tracer.add_edge(5, 6, None::<i64>);
    graph_tracer.add_edge(6, 2, None::<i64>);
    Tracer::delay();
    // }

    let (cycle_length, cycle_start_position) = list_has_cycle(&graph_tracer, &next, head);

    // log {
    log_tracer.print(format!("cycle start position: {}", cycle_start_position));
    log_tracer.print("\n");
    log_tracer.print(format!("cycle length: {}", cycle_length));
    // }
}

fn list_has_cycle(graph_tracer: &GraphTracer, next: &[usize], head: usize) -> (i64, i64) {
    // visualize {
    graph_tracer.select(head as i64, None::<i64>);
    graph_tracer.visit(head as i64, None::<i64>, None::<i64>);
    Tracer::delay();
    graph_tracer.deselect(head as i64, None::<i64>);
    graph_tracer.leave(head as i64, None::<i64>, None::<i64>);
    // }

    // 1. is there a cycle?
    let mut slow = next[head];
    let mut fast = next[next[head]];
    // visualize {
    graph_tracer.select(slow as i64, None::<i64>);
    graph_tracer.visit(fast as i64, None::<i64>, None::<i64>);
    Tracer::delay();
    graph_tracer.deselect(slow as i64, None::<i64>);
    graph_tracer.leave(fast as i64, None::<i64>, None::<i64>);
    // }
    while slow != fast {
        slow = next[slow];
        fast = next[next[fast]];
        // visualize {
        graph_tracer.select(slow as i64, None::<i64>);
        graph_tracer.visit(fast as i64, None::<i64>, None::<i64>);
        Tracer::delay();
        graph_tracer.deselect(slow as i64, None::<i64>);
        graph_tracer.leave(fast as i64, None::<i64>, None::<i64>);
        // }
    }

    // 2. where does the cycle start?
    let mut cycle_start_position = 0i64;
    slow = head;
    // visualize {
    graph_tracer.select(slow as i64, None::<i64>);
    graph_tracer.visit(fast as i64, None::<i64>, None::<i64>);
    Tracer::delay();
    graph_tracer.deselect(slow as i64, None::<i64>);
    graph_tracer.leave(fast as i64, None::<i64>, None::<i64>);
    // }
    while slow != fast {
        slow = next[slow];
        fast = next[fast];
        cycle_start_position += 1;
        // visualize {
        graph_tracer.select(slow as i64, None::<i64>);
        graph_tracer.visit(fast as i64, None::<i64>, None::<i64>);
        Tracer::delay();
        graph_tracer.deselect(slow as i64, None::<i64>);
        graph_tracer.leave(fast as i64, None::<i64>, None::<i64>);
        // }
    }

    // 3. what is the length of the cycle?
    let mut cycle_length = 1i64;
    fast = next[slow];
    // visualize {
    graph_tracer.select(slow as i64, None::<i64>);
    graph_tracer.visit(fast as i64, None::<i64>, None::<i64>);
    Tracer::delay();
    graph_tracer.deselect(slow as i64, None::<i64>);
    graph_tracer.leave(fast as i64, None::<i64>, None::<i64>);
    // }
    while slow != fast {
        fast = next[fast];
        cycle_length += 1;
        // visualize {
        graph_tracer.select(slow as i64, None::<i64>);
        graph_tracer.visit(fast as i64, None::<i64>, None::<i64>);
        Tracer::delay();
        graph_tracer.deselect(slow as i64, None::<i64>);
        graph_tracer.leave(fast as i64, None::<i64>, None::<i64>);
        // }
    }

    (cycle_length, cycle_start_position)
}
