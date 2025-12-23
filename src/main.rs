#![allow(dead_code)]

use crate::{json_maze::parse_json_file, maze::MazeDeserializer, svg::render};
use clap::Parser;

mod building_blocks;
mod json_maze;
mod maze;
mod merge;
mod svg;

#[derive(Parser)]
#[command(name = "circle-maze")]
#[command(about = "Generate circle maze SVG from JSON file")]
struct Cli {
    #[arg(long)]
    parse: String,
}

fn main() {
    let cli = Cli::parse();

    let json_value = parse_json_file(&cli.parse)
        .expect("Failed to parse JSON file");

    let maze = MazeDeserializer::deserialize(json_value)
        .expect("Failed to deserialize maze");

    render(&maze).expect("Failed to render SVG");
}
