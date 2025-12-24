use serde_json::Value;
use std::fs;

pub fn parse_json_file(filename: &str) -> Option<Value> {
    let content = fs::read_to_string(filename).ok()?;
    serde_json::from_str(&content).ok()
}
