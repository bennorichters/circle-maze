use crate::building_blocks::{CircleCoordinate, calc_total_arcs};
use serde_json::Value;

#[derive(Debug)]
pub struct Maze {
    circles: usize,
    ars: Vec<CircleCoordinate>,
    lines: Vec<CircleCoordinate>,
}

pub struct MazeFactory {
    circles: usize,
    ars: Vec<CircleCoordinate>,
    lines: Vec<CircleCoordinate>,
    free: Vec<CircleCoordinate>,
}

impl MazeFactory {
    pub fn new(circles: usize) -> Self {
        Self {
            circles,
            ars: Vec::new(),
            lines: Vec::new(),
            free: all_coords(circles),
        }
    }

    pub fn create(circles: usize) -> Maze {
        todo!()
    }
}

fn all_coords(circle: usize) -> Vec<CircleCoordinate> {
    (1..=circle)
        .flat_map(|c| coordinates_for_circle(c))
        .collect()
}

fn coordinates_for_circle(circle: usize) -> Vec<CircleCoordinate> {
    let total = calc_total_arcs(circle);
    (0..total)
        .map(|i| CircleCoordinate::create_with_arc_index(circle, i).unwrap())
        .collect()
}

pub struct MazeDeserializer;

impl MazeDeserializer {
    pub fn deserialize(data: Value) -> Maze {
        todo!()
    }
}
