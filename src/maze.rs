use crate::circle_coord::{CircleCoord, calc_total_arcs};
use rand::{Rng, seq::SliceRandom};
use serde_json::Value;

#[derive(Debug)]
pub struct Maze {
    circles: usize,
    arcs: Vec<CircleCoord>,
    lines: Vec<CircleCoord>,
}

impl Maze {
    pub fn circles(&self) -> usize {
        self.circles
    }

    pub fn arcs(&self) -> &Vec<CircleCoord> {
        &self.arcs
    }

    pub fn lines(&self) -> &Vec<CircleCoord> {
        &self.lines
    }

    pub fn accessible_neighbours(&self, coord: &CircleCoord) -> Vec<CircleCoord> {
        if coord.circle() < 2 {
            return vec![];
        }

        let mut neighbours = Vec::new();

        if !self.lines.contains(coord) {
            neighbours.push(coord.next_counter_clockwise());
        }

        let next_cw = coord.next_clockwise();
        if !self.lines.contains(&next_cw) {
            neighbours.push(next_cw);
        }

        if !self.arcs.contains(coord) {
            neighbours.push(coord.next_in());
        }

        let next_out = coord.next_out();
        if calc_total_arcs(coord.circle()) < calc_total_arcs(coord.circle() + 1) {
            let next_out_cw = next_out.next_clockwise();
            if !self.arcs.contains(&next_out_cw) {
                neighbours.push(next_out_cw);
            }
        }
        if !self.arcs.contains(&next_out) {
            neighbours.push(next_out);
        }

        neighbours
    }

    fn accessible_neighbours_circle_0(&self, _coord: &CircleCoord) -> Vec<CircleCoord> {
        let mut neighbours = Vec::new();

        for arc_index in 0..6 {
            let coord = CircleCoord::create_with_arc_index(1, arc_index);
            if !self.arcs.contains(&coord) {
                neighbours.push(coord);
            }
        }

        neighbours
    }

    fn accessible_neighbours_circle_1(&self, coord: &CircleCoord) -> Vec<CircleCoord> {
        let mut neighbours = Vec::new();

        if !self.lines.contains(coord) {
            neighbours.push(coord.next_counter_clockwise());
        }

        let next_cw = coord.next_clockwise();
        if !self.lines.contains(&next_cw) {
            neighbours.push(next_cw);
        }

        if !self.arcs.contains(coord) {
            neighbours.push(CircleCoord::create_with_arc_index(0, 0));
        }

        let next_out = coord.next_out();
        if calc_total_arcs(coord.circle()) < calc_total_arcs(coord.circle() + 1) {
            let next_out_cw = next_out.next_clockwise();
            if !self.arcs.contains(&next_out_cw) {
                neighbours.push(next_out_cw);
            }
        }
        if !self.arcs.contains(&next_out) {
            neighbours.push(next_out);
        }

        neighbours
    }
}

pub fn factory<R: Rng>(circles: usize, rng: &mut R) -> Maze {
    let outer = calc_total_arcs(circles);
    let total = outer * circles;
    let mut path: Vec<bool> = vec![false; total];
    let mut used: Vec<bool> = vec![false; total];
    used[(total - outer)..].fill(true);

    let mut free: Vec<(usize, usize)> = vec![];
    for c in 1..circles {
        let t = calc_total_arcs(c);
        for arc_index in 0..t {
            free.push((c, arc_index));
        }
    }
    free.shuffle(rng);

    let mut lines: Vec<CircleCoord> = vec![];
    let mut arcs: Vec<CircleCoord> = vec![];
    for f in free {
        let mut index = (f.0 - 1) * outer + f.1;
        if used[index] {
            continue;
        }

        path.fill(false);

        let mut coord = CircleCoord::create_with_arc_index(f.0, f.1);
        let mut options: Vec<(usize, usize, bool)> = vec![(f.0, f.1, false), (f.0, f.1, true)];
        loop {
            path[index] = true;
            used[index] = true;

            let mut opt: (usize, usize, bool);
            let mut next: CircleCoord;
            loop {
                let opt_index = rng.random_range(0..options.len());
                opt = options.swap_remove(opt_index);

                next = if opt.2 {
                    coord.next_out()
                } else {
                    coord.next_clockwise()
                };
                index = (next.circle() - 1) * outer + next.arc_index();

                if !path[index] {
                    break;
                }
            }

            if opt.2 {
                lines.push(coord);
            } else {
                arcs.push(coord);
            }

            if used[index] {
                break;
            }

            coord = next;
            options.push((coord.circle(), coord.arc_index(), false));
            options.push((coord.circle(), coord.arc_index(), true));
        }
    }

    for i in 0..outer {
        arcs.push(CircleCoord::create_with_arc_index(circles, i));
    }

    Maze {
        circles,
        arcs,
        lines,
    }
}

pub struct MazeDeserializer;

impl MazeDeserializer {
    pub fn deserialize(data: Value) -> Result<Maze, String> {
        // Check that data is an object
        let obj = data.as_object().ok_or("Input must be a JSON object")?;

        // Extract and validate 'circles' field
        let circles = obj
            .get("circles")
            .ok_or("Missing 'circles' field")?
            .as_u64()
            .ok_or("'circles' must be a number")? as usize;

        // Extract and validate 'arcs' field
        let arcs_array = obj
            .get("arcs")
            .ok_or("Missing 'arcs' field")?
            .as_array()
            .ok_or("'arcs' must be an array")?;

        // Parse arcs array
        let mut arcs = Vec::new();
        for (i, arc_obj) in arcs_array.iter().enumerate() {
            let arc_map = arc_obj
                .as_object()
                .ok_or(format!("arcs[{}] must be an object", i))?;

            let circle = arc_map
                .get("circle")
                .ok_or(format!("arcs[{}] missing 'circle' field", i))?
                .as_u64()
                .ok_or(format!("arcs[{}].circle must be a number", i))?
                as usize;

            let arc = arc_map
                .get("arc")
                .ok_or(format!("arcs[{}] missing 'arc' field", i))?
                .as_u64()
                .ok_or(format!("arcs[{}].arc must be a number", i))? as usize;

            let coord = CircleCoord::create_with_arc_index(circle, arc);
            arcs.push(coord);
        }

        // Extract and validate 'lines' field
        let lines_array = obj
            .get("lines")
            .ok_or("Missing 'lines' field")?
            .as_array()
            .ok_or("'lines' must be an array")?;

        // Parse lines array
        let mut lines = Vec::new();
        for (i, line_obj) in lines_array.iter().enumerate() {
            let line_map = line_obj
                .as_object()
                .ok_or(format!("lines[{}] must be an object", i))?;

            let circle = line_map
                .get("circle")
                .ok_or(format!("lines[{}] missing 'circle' field", i))?
                .as_u64()
                .ok_or(format!("lines[{}].circle must be a number", i))?
                as usize;

            let arc = line_map
                .get("arc")
                .ok_or(format!("lines[{}] missing 'arc' field", i))?
                .as_u64()
                .ok_or(format!("lines[{}].arc must be a number", i))?
                as usize;

            let coord = CircleCoord::create_with_arc_index(circle, arc);
            lines.push(coord);
        }

        Ok(Maze {
            circles,
            arcs,
            lines,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_creates_spanning_tree() {
        use rand::SeedableRng;
        use rand::rngs::StdRng;

        let circles = 5;
        let mut rng = StdRng::seed_from_u64(42);
        let maze = factory(circles, &mut rng);

        let mut vertice_count = 0;
        for c in 1..=circles {
            vertice_count += calc_total_arcs(c);
        }

        assert_eq!(
            vertice_count,
            maze.arcs().len() + maze.lines().len(),
            "Maze should form a spanning tree"
        );
    }

    #[test]
    fn test_deserialize_maze_with_three_circles() {
        let json_str = include_str!("../tests/fixtures/maze_3_circles.json");
        let json_data: Value = serde_json::from_str(json_str).unwrap();

        let result = MazeDeserializer::deserialize(json_data);

        assert!(result.is_ok(), "Deserialization should succeed");
        let maze = result.unwrap();
        assert_eq!(maze.circles(), 3, "Maze should have 3 circles");
    }

    #[test]
    fn test_accessible_neighbours_4() {
        let json_str = include_str!("../tests/fixtures/maze_5_circles.json");
        let json_data: Value = serde_json::from_str(json_str).unwrap();

        let maze = MazeDeserializer::deserialize(json_data).unwrap();
        let coord = CircleCoord::create_with_arc_index(3, 4);
        let neighbours = maze.accessible_neighbours(&coord);

        assert_eq!(neighbours.len(), 4);
        assert!(neighbours.contains(&CircleCoord::create_with_arc_index(3, 3)));
        assert!(neighbours.contains(&CircleCoord::create_with_arc_index(3, 5)));
        assert!(neighbours.contains(&CircleCoord::create_with_arc_index(4, 8)));
        assert!(neighbours.contains(&CircleCoord::create_with_arc_index(4, 9)));
    }

    #[test]
    fn test_accessible_neighbours_9() {
        let json_str = include_str!("../tests/fixtures/maze_5_circles.json");
        let json_data: Value = serde_json::from_str(json_str).unwrap();

        let maze = MazeDeserializer::deserialize(json_data).unwrap();
        let coord = CircleCoord::create_with_arc_index(3, 9);
        let neighbours = maze.accessible_neighbours(&coord);

        assert_eq!(neighbours.len(), 3);
        assert!(neighbours.contains(&CircleCoord::create_with_arc_index(2, 9)));
        assert!(neighbours.contains(&CircleCoord::create_with_arc_index(4, 18)));
        assert!(neighbours.contains(&CircleCoord::create_with_arc_index(4, 19)));
    }

    #[test]
    fn test_accessible_neighbours_arc_10() {
        let json_str = include_str!("../tests/fixtures/maze_5_circles.json");
        let json_data: Value = serde_json::from_str(json_str).unwrap();

        let maze = MazeDeserializer::deserialize(json_data).unwrap();
        let coord = CircleCoord::create_with_arc_index(3, 10);
        let neighbours = maze.accessible_neighbours(&coord);

        assert_eq!(neighbours.len(), 2);
        assert!(neighbours.contains(&CircleCoord::create_with_arc_index(2, 10)));
        assert!(neighbours.contains(&CircleCoord::create_with_arc_index(4, 20)));
    }

    #[test]
    fn test_accessible_neighbours_arc_11() {
        let json_str = include_str!("../tests/fixtures/maze_5_circles.json");
        let json_data: Value = serde_json::from_str(json_str).unwrap();

        let maze = MazeDeserializer::deserialize(json_data).unwrap();
        let coord = CircleCoord::create_with_arc_index(3, 11);
        let neighbours = maze.accessible_neighbours(&coord);

        assert_eq!(neighbours.len(), 1);
        assert!(neighbours.contains(&CircleCoord::create_with_arc_index(3, 0)));
    }

    #[test]
    fn test_accessible_neighbours_circle_0() {
        let json_str = include_str!("../tests/fixtures/maze_5_circles.json");
        let json_data: Value = serde_json::from_str(json_str).unwrap();

        let maze = MazeDeserializer::deserialize(json_data).unwrap();
        let coord = CircleCoord::create_with_arc_index(0, 0);
        let neighbours = maze.accessible_neighbours_circle_0(&coord);

        assert_eq!(neighbours.len(), 1);
        assert!(neighbours.contains(&CircleCoord::create_with_arc_index(1, 4)));
    }
}
