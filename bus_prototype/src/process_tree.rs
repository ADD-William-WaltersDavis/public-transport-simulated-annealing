use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use serde::Deserialize;
use serde_json::Result;
// use std::io::BufReader;
// use fs_err::File;

#[derive(Deserialize, Debug)]
struct KDNodes {
    coords: [f64; 2],
    node_id: usize,
}

pub fn process() -> KdTree<f64, usize, [f64; 2]> {
    let kdpoints = read_json_file("data/nodes_for_kdtree.json").unwrap();

    let dimensions = 2;
    let mut kdtree = KdTree::new(dimensions);

    for point in &kdpoints {
        kdtree.add(point.coords, point.node_id).unwrap();
    }

    assert_eq!(kdtree.size(), kdpoints.len());
    assert_eq!(
        kdtree
            .nearest(&kdpoints[0].coords, 0, &squared_euclidean)
            .unwrap(),
        vec![]
    );
    assert_eq!(
        kdtree
            .nearest(&kdpoints[0].coords, 1, &squared_euclidean)
            .unwrap(),
        vec![(0f64, &kdpoints[0].node_id)]
    );
    let search_coord: [f64; 2] = [565811.0, 141704.0];
    let result = kdtree
        .nearest(&search_coord, 1, &squared_euclidean)
        .unwrap();
    println!("result: {:?}", result);

    println!("All tests passed!");
    kdtree
}

fn read_json_file(file_path: &str) -> Result<Vec<KDNodes>> {
    let contents = fs_err::read_to_string(&file_path).unwrap();
    let output: Vec<KDNodes> = serde_json::from_str(&contents).unwrap();
    Ok(output)
}
