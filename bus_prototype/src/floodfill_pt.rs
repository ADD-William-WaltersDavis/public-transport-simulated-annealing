use bus_prototype::{NodeRoute, NodeWalk};

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Clone)]
pub struct PriorityQueueItem<C, N, P> {
    pub cost: C,
    pub node: N,
    pub has_pt: P,
}

impl<C: Ord, N: Ord, P: Ord> PartialOrd for PriorityQueueItem<C, N, P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Telling rust to order the heap by cost
impl<C: Ord, N: Ord, P: Ord> Ord for PriorityQueueItem<C, N, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = other.cost.cmp(&self.cost);
        if ord != Ordering::Equal {
            return ord;
        }
        // The tie-breaker is arbitrary, based on the node
        self.node.cmp(&other.node)
    }
}

pub fn run(
    graph_walk: &Vec<NodeWalk>,
    graph_routes: &Vec<NodeRoute>,
    node_values: &Vec<bool>,
    start_node: usize,
    start_node_weight: usize,
    trip_start_seconds: usize,
    time_limit: usize,
) -> usize {
    let mut queue: BinaryHeap<PriorityQueueItem<usize, usize, bool>> = BinaryHeap::new();
    queue.push(PriorityQueueItem {
        cost: 0,
        node: start_node,
        has_pt: false,
    });
    let mut time_taken: usize = time_limit;
    let mut nodes_visited: Vec<bool> = vec![false; graph_walk.len()].into();

    while let Some(current) = queue.pop() {
        // break if we find the school
        if node_values[current.node] {
            time_taken = current.cost;
            break;
        }
        // skip if we've already visited this node
        if nodes_visited[current.node] {
            continue;
        }
        // mark the node as visited
        nodes_visited[current.node] = true;

        // add the edges to the queue
        for edge in &graph_walk[current.node].edges {
            let new_cost = current.cost + edge.cost;

            if new_cost < time_limit {
                queue.push(PriorityQueueItem {
                    cost: new_cost,
                    node: edge.to,
                    has_pt: edge.has_pt,
                });
            }
        }
        if current.has_pt {
            take_next_pt_route(
                &graph_routes,
                current.cost,
                &mut queue,
                time_limit,
                trip_start_seconds,
                current.node,
            );
        }
    }
    time_taken * start_node_weight
}

fn take_next_pt_route(
    graph_routes: &Vec<NodeRoute>,
    time_so_far: usize,
    queue: &mut BinaryHeap<PriorityQueueItem<usize, usize, bool>>,
    time_limit: usize,
    trip_start_seconds: usize,
    current_node: usize,
) {
    let time_of_arrival_current_node: usize = trip_start_seconds + time_so_far;

    let mut found_next_service = false;
    let mut journey_time_to_next_node = 0;
    let mut next_leaving_time = 0;

    for edge in &graph_routes[current_node].timetable {
        if time_of_arrival_current_node <= edge.leavetime {
            next_leaving_time = edge.leavetime;
            journey_time_to_next_node = edge.cost;
            found_next_service = true;
            break;
        }
    }

    if found_next_service {
        let wait_time_this_stop = next_leaving_time - time_of_arrival_current_node;
        let time_since_start_next_stop_arrival =
            time_so_far + journey_time_to_next_node + wait_time_this_stop;

        if time_since_start_next_stop_arrival < time_limit {
            let destination_node = graph_routes[current_node].to;

            queue.push(PriorityQueueItem {
                cost: time_since_start_next_stop_arrival,
                node: destination_node,
                has_pt: true,
            });
        };
    }
}
