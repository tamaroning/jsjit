use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct JsonTopLevel {
    functions: Vec<JsonFunction>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonFunction {
    name: String,
    bytecode: Vec<u8>,
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let contents = fs::read_to_string(&args[1]).expect("Could not read file");
    let json: JsonTopLevel = serde_json::from_str(&contents).unwrap();

    dbg!(json);
}
