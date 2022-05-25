
pub mod v1 {

// output behavior in the form of a list of strings that say which
// outputs include ["move_left", "move_right", "jump", "attack_up", "attack_down", "attack"]

use serde_derive::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
struct InputNode {
    bias:f32,
    value:f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Node {
    bias:f32,
    value:f32,
    weights:Vec<f32>,
}


#[derive(Debug, Serialize, Deserialize)]
struct Matrix2442 {
    input_nodes: Vec<InputNode>,
    hidden_nodes_1: Vec<Node>
}

    pub fn run(px:i16, ex:i16) -> Vec<bool> {
        let matrix: Matrix2442 = serde_json::from_str(&std::fs::read_to_string("src/JSON/run.json").unwrap()).unwrap();
        let mut action: Vec<bool> = vec![false, false, false, false, false, false];
        action
    }

}

