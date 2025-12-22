#![allow(dead_code)]

use crate::maze::MazeFactory;

mod building_blocks;
mod maze;
mod svg;

fn main() {
    let maze = MazeFactory::create(5);
    println!("Maze: {:?}", maze);
}
