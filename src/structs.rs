
pub mod config {
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub network: Network,
}

#[derive(Deserialize, Debug)]
pub struct Network {
    pub input: f32,
    pub layers: f32,
    pub nodes: f32,
    pub output: f32,
    pub cone_structure: bool
}

pub fn read_config() -> Config {
    let matrix_str: String = std::fs::read_to_string("src/assets/config.json").unwrap();
    let config: Config = serde_json::from_str(&matrix_str).unwrap();
    config
}

}