use wasm_bindgen::prelude::*;
use rand::{SeedableRng, rngs::SmallRng};

mod circle_coord;
mod json;
mod maze;
mod merge;
mod svg;

use maze::{MazeDeserializer, MazeSerializer, factory};
use svg::{render_with_path};

fn create_rng() -> SmallRng {
    let mut seed = [0u8; 32];
    getrandom::fill(&mut seed).expect("Failed to get random seed");
    SmallRng::from_seed(seed)
}

#[wasm_bindgen]
pub fn generate_maze_svg(circles: usize) -> String {
    let maze = factory(circles, &mut create_rng());
    let path = maze.tree_diameter();
    render_with_path(&maze, &path)
}

#[wasm_bindgen]
pub fn generate_maze_json(circles: usize) -> String {
    let maze = factory(circles, &mut create_rng());
    let serialized = MazeSerializer::serialize(&maze);
    serde_json::to_string_pretty(&serialized)
        .unwrap_or_else(|_| String::from("{}"))
}

#[wasm_bindgen]
pub fn load_maze_svg(json_string: &str) -> Result<String, String> {
    let json_value: serde_json::Value = serde_json::from_str(json_string)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let maze = MazeDeserializer::deserialize(json_value)
        .map_err(|e| format!("Failed to deserialize maze: {}", e))?;

    let path = maze.tree_diameter();
    Ok(render_with_path(&maze, &path))
}
