#![allow(dead_code)]

use crate::{maze::MazeFactory, svg::render};

mod building_blocks;
mod maze;
mod svg;

fn main() {
    let maze = MazeFactory::create(5);

    let _ = render(&maze);
}
