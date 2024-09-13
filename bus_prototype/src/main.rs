mod floodfill;
mod process_tree;

use bus_prototype::{read_start_nodes, read_travel_times, read_walk_graph, write_file, Points, NodeWalk, EdgeWalk, StartNodes};
use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use rand::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let n_iterations = 1000;
    let temp = 100.0;
    let step_size = 1000.0;
    let bounds: [f64; 4] = [565600.0, 568300.0, 141400.0, 142800.0];

    let kdtree = process_tree::process();
    let mut graph_walk = read_walk_graph().unwrap();
    let node_values: Vec<bool> = (0..graph_walk.len()).map(|i| i == 28).collect();
    let travel_times = read_travel_times().unwrap();

    let start_nodes_and_weights = read_start_nodes().unwrap();

    let mut points_visited: Vec<Points> = Vec::new();

    let start_time = Instant::now();

    let mut best_points = initialise_points(bounds);
    let mut nodes_changed: Vec<usize>;
    (graph_walk, nodes_changed) =
        add_to_graph_walk(&mut graph_walk, &best_points, &kdtree, &travel_times);

    let mut best_total_time =
        calculate_total_weighted_time(&graph_walk, &node_values, &start_nodes_and_weights);
    // revert to origianl state
    graph_walk = reset_graph_walk(&mut graph_walk, nodes_changed);

    let mut current_points = best_points;
    let mut current_total_time = best_total_time;
    points_visited.push(current_points);

    for i in 0..n_iterations {
        // decrease temperature
        let mut t = temp / (i as f64 + 1.0);
        if t < 0.01 {
            t = 0.01;
        }

        let candidate_points = get_neighbour(current_points, step_size, &bounds);
        (graph_walk, nodes_changed) =
            add_to_graph_walk(&mut graph_walk, &candidate_points, &kdtree, &travel_times);
        let candidate_total_time =
            calculate_total_weighted_time(&graph_walk, &node_values, &start_nodes_and_weights);
        // revert to original state
        graph_walk = reset_graph_walk(&mut graph_walk, nodes_changed);

        let mut rng = rand::thread_rng();
        if candidate_total_time < best_total_time
            || rng.gen_range(0.0..1.0)
                < ((current_total_time as f64 - candidate_total_time as f64) / t).exp()
        {
            current_points = candidate_points;
            current_total_time = candidate_total_time;
            points_visited.push(current_points);
            if candidate_total_time < best_total_time {
                best_points = candidate_points;
                best_total_time = candidate_total_time;
            }
        }
        if i % 100 == 0 {
            println!("Iteration: {:?}", i);
            println!("Temperature: {:?}", t);
            println!("Best total time: {:?}", best_total_time);
        }
    }
    println!("Best point: {:?}", best_points);
    println!("Best total time: {:?}", best_total_time);

    println!("Time taken: {:?}", start_time.elapsed());

    let file_path = "results/points_visited.json";
    write_file(file_path, points_visited).unwrap();
}

fn add_to_graph_walk(
    graph_walk: &mut Vec<NodeWalk>,
    points: &Points,
    kdtree: &KdTree<f64, usize, [f64; 2]>,
    travel_times: &HashMap<usize, HashMap<usize, usize>>,
) -> (Vec<NodeWalk>, Vec<usize>) {
    let mut node_ids = Vec::new();
    for point in points.points.iter() {
        let result = kdtree.nearest(point, 1, &squared_euclidean).unwrap();
        node_ids.push(*result[0].1);
    }
    let mut nodes_changes = Vec::new();
    for i in 1..node_ids.len() {
        if node_ids[i] != node_ids[i - 1] {
            let new_edge = EdgeWalk {
                to: node_ids[i],
                cost: travel_times[&node_ids[i - 1]][&node_ids[i]] + 10,
                _pt: true,
            };
            graph_walk[node_ids[i - 1]].edges.push(new_edge);
            nodes_changes.push(node_ids[i - 1]);
        }
    }
    (graph_walk.to_vec(), nodes_changes)
}

fn reset_graph_walk(graph_walk: &mut Vec<NodeWalk>, nodes_changes: Vec<usize>) -> Vec<NodeWalk> {
    for node in nodes_changes {
        graph_walk[node].edges.pop();
    }
    graph_walk.to_vec()
}

fn calculate_total_weighted_time(
    graph_walk: &Vec<NodeWalk>,
    node_values: &Vec<bool>,
    start_nodes_and_weights: &Vec<StartNodes>,
) -> usize {
    let weighted_time_taken: Vec<usize> = start_nodes_and_weights
        .into_par_iter()
        .map(|start_nodes_and_weight| {
            let start_node = start_nodes_and_weight.node;
            let start_node_weight = start_nodes_and_weight.weight;
            floodfill::run(
                &graph_walk,
                &node_values,
                start_node,
                start_node_weight,
                3600,
            )
        })
        .collect();
    let total_weighted_time_taken: usize = weighted_time_taken.iter().sum();
    total_weighted_time_taken / 578
}

fn initialise_points(bounds: [f64; 4]) -> Points {
    let [minx, maxx, miny, maxy] = bounds;
    let mut rng = rand::thread_rng();
    let mut points = [[0.0; 2]; 2];
    for i in 0..2 {
        points[i][0] = rng.gen_range(minx..maxx);
        points[i][1] = rng.gen_range(miny..maxy);
    }
    Points { points: points }
}
fn get_neighbour(current_points: Points, step_size: f64, bounds: &[f64; 4]) -> Points {
    let mut new_points = current_points;
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-step_size..step_size);
    let y = rng.gen_range(-step_size..step_size);
    // select a random point to change
    let point_to_change = rng.gen_range(0..new_points.points.len());
    if new_points.points[point_to_change][0] + x < bounds[0] {
        new_points.points[point_to_change][0] = bounds[0];
    } else if new_points.points[point_to_change][0] + x > bounds[1] {
        new_points.points[point_to_change][0] = bounds[1];
    } else {
        new_points.points[point_to_change][0] += x;
    }
    if new_points.points[point_to_change][1] + y < bounds[2] {
        new_points.points[point_to_change][1] = bounds[2];
    } else if new_points.points[point_to_change][1] + y > bounds[3] {
        new_points.points[point_to_change][1] = bounds[3];
    } else {
        new_points.points[point_to_change][1] += y;
    }
    new_points
}
// fn get_neighbour(
//     current_points: Points,
//     step_size: f64,
//     bounds: &[f64; 4],
// ) -> Points {
//     let mut new_points = current_points;
//     let mut rng = rand::thread_rng();
//     for i in 0..10 {
//         let x = rng.gen_range(-step_size..step_size);
//         let y = rng.gen_range(-step_size..step_size);
//         if new_points.points[i][0] + x < bounds[0] {
//             new_points.points[i][0] = bounds[0];
//         } else if new_points.points[i][0] + x > bounds[1] {
//             new_points.points[i][0] = bounds[1];
//         } else {
//             new_points.points[i][0] += x;
//         }
//         if new_points.points[i][1] + y < bounds[2] {
//             new_points.points[i][1] = bounds[2];
//         } else if new_points.points[i][1] + y > bounds[3] {
//             new_points.points[i][1] = bounds[3];
//         } else {
//             new_points.points[i][1] += y;
//         }
//     }
//     new_points
// }
