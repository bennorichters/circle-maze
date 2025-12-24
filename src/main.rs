#![allow(dead_code)]

use crate::{json_maze::parse_json_file, maze::{MazeDeserializer, factory}, svg::render};
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
    parse: Option<String>,

    #[arg(long)]
    create: Option<usize>,
}

fn main() {
    let cli = Cli::parse();

    let maze = if let Some(circles) = cli.create {
        factory(circles, &mut rand::rng())
    } else if let Some(path) = cli.parse {
        let json_value = parse_json_file(&path)
            .expect("Failed to parse JSON file");
        MazeDeserializer::deserialize(json_value)
            .expect("Failed to deserialize maze")
    } else {
        eprintln!("Error: Either --parse or --create must be provided");
        std::process::exit(1);
    };

    render(&maze).expect("Failed to render SVG");
}
