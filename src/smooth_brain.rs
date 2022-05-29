
pub mod v1 {

// output behavior in the form of a list of strings that say which
// outputs include ["move_left", "move_right", "jump", "attack_up", "attack_down", "attack"]

use serde_derive::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputNode {
    bias:f32,
    value:f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    bias:f32,
    value:f32,
    weights:Vec<f32>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Matrix2442 {
    input_nodes: Vec<InputNode>,
    hidden_nodes: Vec<Vec<Node>>,
    output_nodes: Vec<Node>
}

    pub fn mutate(mx: &String) -> String {
        let mut mxo:Matrix2442 = serde_json::from_str(&mx).unwrap();
        let mut rng = rand::thread_rng();

        for i in 0..mxo.input_nodes.len() {
            let rng_num = rng.gen_range(-5.0..5.0);
            mxo.input_nodes[i].bias += rng_num;
        }
        // println!("{:?}", mxo);

        for k in 0..mxo.hidden_nodes.len() {
            for i in 0..mxo.hidden_nodes[k].len() {
                let rng_num = rng.gen_range(-1.0..1.0);
                mxo.hidden_nodes[k][i].bias += rng_num;
                for j in 0..mxo.hidden_nodes[k][i].weights.len() {
                    let rng_num = rng.gen_range(-0.1..0.1);
                    mxo.hidden_nodes[k][i].weights[j] += rng_num;
                }
            }
        }


        for i in 0..mxo.output_nodes.len() {
            let rng_num = rng.gen_range(-1.0..1.0);
            mxo.output_nodes[i].bias += rng_num;
            for j in 0..mxo.output_nodes[i].weights.len() {
                let rng_num = rng.gen_range(-0.1..0.1);
                mxo.output_nodes[i].weights[j] += rng_num;
            }
        }

        serde_json::to_string_pretty(&mxo).unwrap()


    }

    pub fn run(mut mx: Matrix2442, inputs: Vec<f64>) -> Vec<bool> {


        for i in 0..mx.input_nodes.len() {
            if mx.input_nodes.len() != inputs.len() {
                panic!("inputs and matrix input nodes are not the same length");
            }
            mx.input_nodes[i].value = inputs[i] as f32;
        }




        for k in 0..mx.hidden_nodes.len() {
            for i in 0..mx.hidden_nodes[k].len() {
            mx.hidden_nodes[k][i].value += mx.hidden_nodes[k][i].bias;
        for j in 0..mx.hidden_nodes[k][i].weights.len() {
                if j == 0 {
                mx.hidden_nodes[k][i].value += mx.input_nodes[j].value * mx.hidden_nodes[k][i].weights[j];
                } else {
                mx.hidden_nodes[k][i].value += mx.hidden_nodes[k][j-1].value * mx.hidden_nodes[k][i].weights[j];
                }
        }
        }
    }


        for i in 0..mx.output_nodes.len() {
            mx.output_nodes[i].value += mx.output_nodes[i].bias;
        for j in 0..mx.output_nodes[i].weights.len() {
                mx.output_nodes[i].value += mx.hidden_nodes[mx.hidden_nodes.len()-1][j].value * mx.output_nodes[i].weights[j];
        }
        }

    // outputs include ["move_left", "move_right", "jump", "attack_up", "attack_down", "attack forwards"]
    let mut action: Vec<bool> = vec![false, false, false, false, false, false];

    if mx.output_nodes.len() == 2 {
        action[0] = mx.output_nodes[0].value >= 0.0;
        action[1] = mx.output_nodes[1].value >= 0.0;
    }

    if mx.output_nodes.len() == 3 {
        action[0] = mx.output_nodes[0].value >= 0.0;
        action[1] = mx.output_nodes[1].value >= 0.0;
        action[2] = mx.output_nodes[2].value >= 0.0;
    }

    action
    

}

}