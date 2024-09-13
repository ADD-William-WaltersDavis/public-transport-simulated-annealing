use crate::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Clone)]
pub struct PriorityQueueItem<C, N> {
    pub cost: C,
    pub node: N,
}

impl<C: Ord, N: Ord> PartialOrd for PriorityQueueItem<C, N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Telling rust to order the heap by cost
impl<C: Ord, N: Ord> Ord for PriorityQueueItem<C, N> {
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
    node_values: &Vec<bool>,
    start_node: usize,
    start_node_weight: usize,
    time_limit: usize,
) -> usize {
    let mut queue: BinaryHeap<PriorityQueueItem<usize, usize>> = BinaryHeap::new();
    queue.push(PriorityQueueItem {
        cost: 0,
        node: start_node,
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
                });
            }
        }
    }
    time_taken * start_node_weight
}
