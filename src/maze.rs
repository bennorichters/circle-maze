use crate::building_blocks::{CircleCoordinate, calc_total_arcs};
use serde_json::Value;

#[derive(Debug)]
pub struct Maze {
    circles: usize,
    arcs: Vec<CircleCoordinate>,
    lines: Vec<CircleCoordinate>,
}

impl Maze {
    pub fn circles(&self) -> usize {
        self.circles
    }
}

pub struct MazeFactory {
    circles: usize,
    arcs: Vec<CircleCoordinate>,
    lines: Vec<CircleCoordinate>,
    free: Vec<CircleCoordinate>,
}

impl MazeFactory {
    pub fn new(circles: usize) -> Self {
        Self {
            circles,
            arcs: Vec::new(),
            lines: Vec::new(),
            free: all_coords(circles),
        }
    }

    pub fn create(circles: usize) -> Maze {
        Maze {
            circles,
            arcs: vec!(),
            lines: vec!(),
        }
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
    pub fn deserialize(data: Value) -> Result<Maze, String> {
        // Check that data is an object
        let obj = data.as_object()
            .ok_or("Input must be a JSON object")?;

        // Extract and validate 'circles' field
        let circles = obj.get("circles")
            .ok_or("Missing 'circles' field")?
            .as_u64()
            .ok_or("'circles' must be a number")? as usize;

        // Extract and validate 'arcs' field
        let arcs_array = obj.get("arcs")
            .ok_or("Missing 'arcs' field")?
            .as_array()
            .ok_or("'arcs' must be an array")?;

        // Parse arcs array
        let mut arcs = Vec::new();
        for (i, arc_obj) in arcs_array.iter().enumerate() {
            let arc_map = arc_obj.as_object()
                .ok_or(format!("arcs[{}] must be an object", i))?;

            let circle = arc_map.get("circle")
                .ok_or(format!("arcs[{}] missing 'circle' field", i))?
                .as_u64()
                .ok_or(format!("arcs[{}].circle must be a number", i))? as usize;

            let arc = arc_map.get("arc")
                .ok_or(format!("arcs[{}] missing 'arc' field", i))?
                .as_u64()
                .ok_or(format!("arcs[{}].arc must be a number", i))? as usize;

            let coord = CircleCoordinate::create_with_arc_index(circle, arc)
                .map_err(|e| format!("Invalid arc coordinate at arcs[{}]: {}", i, e))?;

            arcs.push(coord);
        }

        // Extract and validate 'lines' field
        let lines_array = obj.get("lines")
            .ok_or("Missing 'lines' field")?
            .as_array()
            .ok_or("'lines' must be an array")?;

        // Parse lines array
        let mut lines = Vec::new();
        for (i, line_obj) in lines_array.iter().enumerate() {
            let line_map = line_obj.as_object()
                .ok_or(format!("lines[{}] must be an object", i))?;

            let circle = line_map.get("circle")
                .ok_or(format!("lines[{}] missing 'circle' field", i))?
                .as_u64()
                .ok_or(format!("lines[{}].circle must be a number", i))? as usize;

            let arc = line_map.get("arc")
                .ok_or(format!("lines[{}] missing 'arc' field", i))?
                .as_u64()
                .ok_or(format!("lines[{}].arc must be a number", i))? as usize;

            let coord = CircleCoordinate::create_with_arc_index(circle, arc)
                .map_err(|e| format!("Invalid line coordinate at lines[{}]: {}", i, e))?;

            lines.push(coord);
        }

        Ok(Maze {
            circles,
            arcs,
            lines,
        })
    }
}
