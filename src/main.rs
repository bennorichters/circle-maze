#![allow(dead_code)]

use crate::maze::MazeFactory;

mod building_blocks;
mod maze;

fn main() {
    let maze = MazeFactory::create(5);
    println!("Maze: {:?}", maze);
}
