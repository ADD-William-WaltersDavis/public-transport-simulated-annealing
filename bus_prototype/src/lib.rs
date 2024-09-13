use anyhow::Result;
use serde_json;

use fs_err::File;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::collections::HashMap;
use std::io::{BufWriter, Write};


#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct Points {
    pub points: [[f64; 2]; 2],
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct EdgeWalk {
    pub to: usize,
    pub cost: usize,
    pub _pt: bool,
}
#[derive(Deserialize, Debug, Clone)]
pub struct NodeWalk {
    pub edges: SmallVec<[EdgeWalk; 4]>,
}

pub fn read_walk_graph() -> Result<Vec<NodeWalk>> {
    let contents = fs_err::read_to_string("data/graph_walk.json").unwrap();
    let intermediate: Vec<Vec<EdgeWalk>> = serde_json::from_str(&contents).unwrap();
    let output = intermediate
        .into_iter()
        .map(|edges| NodeWalk {
            edges: edges.into(),
        })
        .collect();
    Ok(output)
}

#[derive(Deserialize, Debug)]
pub struct StartNodes {
    pub node: usize,
    pub weight: usize,
}

pub fn read_start_nodes() -> Result<Vec<StartNodes>> {
    let contents = fs_err::read_to_string("data/start_nodes.json").unwrap();
    let output: Vec<StartNodes> = serde_json::from_str(&contents).unwrap();
    Ok(output)
}

pub fn read_travel_times() -> Result<HashMap<usize, HashMap<usize, usize>>> {
    let contents = fs_err::read_to_string("data/travel_times.json").unwrap();
    let output = serde_json::from_str(&contents).unwrap();
    Ok(output)
}

pub fn write_file<T: Serialize>(path: &str, data: T) -> Result<()> {
    println!("Writing to {path}");
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &data)?;
    writer.flush()?;
    Ok(())
}