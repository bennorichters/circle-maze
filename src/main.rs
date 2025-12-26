use crate::{
    circle_coord::CircleCoord,
    json::parse_json_file,
    maze::{MazeDeserializer, MazeSerializer, factory},
    svg::{render, render_with_path},
};
use clap::Parser;
use std::fs::File;
use std::io::Write;

mod circle_coord;
mod json;
mod maze;
mod merge;
mod svg;

#[derive(Parser)]
#[command(name = "circle-maze")]
#[command(about = "Generate circle maze SVG from JSON file")]
struct Cli {
    #[arg(long)]
    parse: Option<String>,

    #[arg(long)]
    create: Option<usize>,
}

fn main() {
    let cli = Cli::parse();

    let maze = if let Some(circles) = cli.create {
        factory(circles, &mut rand::rng())
    } else if let Some(path) = cli.parse {
        let json_value = parse_json_file(&path).expect("Failed to parse JSON file");
        MazeDeserializer::deserialize(json_value).expect("Failed to deserialize maze")
    } else {
        eprintln!("Error: Either --parse or --create must be provided");
        std::process::exit(1);
    };

    let serialized = MazeSerializer::serialize(&maze);
    let json_string = serde_json::to_string_pretty(&serialized)
        .expect("Failed to serialize maze to JSON string");
    let mut file = File::create("maze.json").expect("Failed to create maze.json");
    file.write_all(json_string.as_bytes()).expect("Failed to write to maze.json");

    let path = maze.tree_diameter();
    render_with_path(&maze, &path).expect("Failed to render SVG");
}
