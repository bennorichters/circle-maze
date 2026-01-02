use crate::circle_coord::{CircleCoord, calc_total_arcs};
use rand::{Rng, seq::SliceRandom};
use serde_json::Value;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Maze {
    circles: usize,
    arcs: HashSet<CircleCoord>,
    lines: HashSet<CircleCoord>,
}

impl Maze {
    pub fn circles(&self) -> usize {
        self.circles
    }

    pub fn arcs(&self) -> &HashSet<CircleCoord> {
        &self.arcs
    }

    pub fn lines(&self) -> &HashSet<CircleCoord> {
        &self.lines
    }

    pub fn accessible_neighbours(&self, coord: &CircleCoord) -> Vec<CircleCoord> {
        if coord.circle() == 0 {
            return self.accessible_neighbours_circle_0(coord);
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
            if coord.circle() == 1 {
                neighbours.push(CircleCoord::create_with_arc_index(0, 0));
            } else {
                neighbours.push(coord.next_in());
            }
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

    pub fn find_path(&self, start: CircleCoord, finish: CircleCoord) -> Vec<CircleCoord> {
        use std::collections::{VecDeque, HashMap};

        if start == finish {
            return vec![start];
        }

        let mut queue = VecDeque::new();
        let mut parent = HashMap::new();

        queue.push_back(start.clone());
        parent.insert(start.clone(), None);

        while let Some(current) = queue.pop_front() {
            if current == finish {
                let mut path = Vec::new();
                let mut node = Some(current);

                while let Some(n) = node {
                    path.push(n.clone());
                    node = parent.get(&n).and_then(|p| p.clone());
                }

                path.reverse();
                return path;
            }

            for neighbor in self.accessible_neighbours(&current) {
                if !parent.contains_key(&neighbor) {
                    parent.insert(neighbor.clone(), Some(current.clone()));
                    queue.push_back(neighbor);
                }
            }
        }

        vec![]
    }

    pub fn tree_diameter(&self) -> Vec<CircleCoord> {
        let start = CircleCoord::create_with_arc_index(0, 0);
        let first_end = self.find_farthest_node(&start);
        self.find_farthest_with_path(&first_end)
    }

    fn find_farthest_node(&self, start: &CircleCoord) -> CircleCoord {
        use std::collections::{VecDeque, HashSet};

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut farthest = start.clone();

        queue.push_back(start.clone());
        visited.insert(start.clone());

        while let Some(current) = queue.pop_front() {
            farthest = current.clone();

            for neighbor in self.accessible_neighbours(&current) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back(neighbor);
                }
            }
        }

        farthest
    }

    fn find_farthest_with_path(&self, start: &CircleCoord) -> Vec<CircleCoord> {
        use std::collections::{VecDeque, HashMap};

        let mut queue = VecDeque::new();
        let mut parent: HashMap<CircleCoord, Option<CircleCoord>> = HashMap::new();
        let mut farthest = start.clone();

        queue.push_back(start.clone());
        parent.insert(start.clone(), None);

        while let Some(current) = queue.pop_front() {
            farthest = current.clone();

            for neighbor in self.accessible_neighbours(&current) {
                if !parent.contains_key(&neighbor) {
                    parent.insert(neighbor.clone(), Some(current.clone()));
                    queue.push_back(neighbor);
                }
            }
        }

        let mut path = Vec::new();
        let mut node = Some(farthest);

        while let Some(n) = node {
            path.push(n.clone());
            node = parent.get(&n).and_then(|p| p.clone());
        }

        path.reverse();
        path
    }
}

fn coord_to_index(circle: usize, arc_index: usize, outer: usize) -> usize {
    (circle - 1) * outer + arc_index
}

fn initialize_tracking_vectors(circles: usize, outer: usize) -> (Vec<bool>, Vec<bool>) {
    let total = outer * circles;
    let path = vec![false; total];
    let mut used = vec![false; total];
    used[(total - outer)..].fill(true);
    (path, used)
}

fn generate_shuffled_coordinates<R: Rng>(circles: usize, rng: &mut R) -> Vec<(usize, usize)> {
    let mut free = Vec::new();
    for c in 1..circles {
        let t = calc_total_arcs(c);
        for arc_index in 0..t {
            free.push((c, arc_index));
        }
    }
    free.shuffle(rng);
    free
}

#[derive(Debug)]
enum Direction {
    Out,
    In,
    Clockwise,
    CounterClockwise,
}

impl Direction {
    fn calculate_edge(&self, coord: &CircleCoord) -> Option<(CircleCoord, CircleCoord)> {
        match self {
            Direction::Out => Some((coord.clone(), coord.next_out())),
            Direction::In => {
                if coord.circle() == 1 {
                    None
                } else {
                    Some((coord.next_in(), coord.clone()))
                }
            }
            Direction::Clockwise => Some((coord.clone(), coord.next_clockwise())),
            Direction::CounterClockwise => Some((coord.next_clockwise(), coord.clone())),
        }
    }

    fn is_arc_direction(&self) -> bool {
        matches!(self, Direction::Clockwise | Direction::CounterClockwise)
    }

    fn uses_branch(&self) -> bool {
        matches!(self, Direction::Out | Direction::Clockwise)
    }
}

fn create_direction_candidates(circle: usize, arc_index: usize) -> Vec<(usize, usize, Direction)> {
    vec![
        (circle, arc_index, Direction::Out),
        (circle, arc_index, Direction::Clockwise),
        (circle, arc_index, Direction::In),
        (circle, arc_index, Direction::CounterClockwise),
    ]
}

fn perform_random_walk<R: Rng>(
    start: (usize, usize),
    outer: usize,
    path: &mut [bool],
    used: &mut [bool],
    lines: &mut HashSet<CircleCoord>,
    arcs: &mut HashSet<CircleCoord>,
    rng: &mut R,
) {
    path.fill(false);

    let mut candidates = create_direction_candidates(start.0, start.1);
    let start_index = coord_to_index(start.0, start.1, outer);
    path[start_index] = true;
    used[start_index] = true;

    loop {
        let candidate_index = rng.random_range(0..candidates.len());
        let candidate = candidates.swap_remove(candidate_index);

        let candidate_coord = CircleCoord::create_with_arc_index(candidate.0, candidate.1);
        let edge_option = candidate.2.calculate_edge(&candidate_coord);

        if edge_option.is_none() {
            continue;
        }

        let (branch, leaf) = edge_option.unwrap();

        let leaf_index = coord_to_index(leaf.circle(), leaf.arc_index(), outer);
        if path[leaf_index] {
            continue;
        }

        candidates.extend(create_direction_candidates(leaf.circle(), leaf.arc_index()));

        let edge = if candidate.2.uses_branch() { branch } else { leaf };
        if candidate.2.is_arc_direction() {
            arcs.insert(edge);
        } else {
            lines.insert(edge);
        }

        if used[leaf_index] {
            break;
        }

        path[leaf_index] = true;
        used[leaf_index] = true;
    }
}

fn build_spanning_tree<R: Rng>(
    free: Vec<(usize, usize)>,
    outer: usize,
    path: &mut [bool],
    used: &mut [bool],
    rng: &mut R,
) -> (HashSet<CircleCoord>, HashSet<CircleCoord>) {
    let mut lines = HashSet::new();
    let mut arcs = HashSet::new();

    for f in free {
        let index = coord_to_index(f.0, f.1, outer);
        if used[index] {
            continue;
        }

        perform_random_walk(f, outer, path, used, &mut lines, &mut arcs, rng);
    }

    (lines, arcs)
}

fn add_outer_boundary(mut arcs: HashSet<CircleCoord>, circles: usize) -> HashSet<CircleCoord> {
    let outer = calc_total_arcs(circles);
    for i in 0..outer {
        arcs.insert(CircleCoord::create_with_arc_index(circles, i));
    }
    arcs
}

pub fn factory<R: Rng>(circles: usize, rng: &mut R) -> Maze {
    let outer = calc_total_arcs(circles);
    let (mut path, mut used) = initialize_tracking_vectors(circles, outer);
    let free = generate_shuffled_coordinates(circles, rng);

    let (lines, arcs) = build_spanning_tree(free, outer, &mut path, &mut used, rng);
    let arcs = add_outer_boundary(arcs, circles);

    Maze {
        circles,
        arcs,
        lines,
    }
}

pub struct MazeDeserializer;

impl MazeDeserializer {
    pub fn deserialize(data: Value) -> Result<Maze, String> {
        let obj = data.as_object().ok_or("Input must be a JSON object")?;

        let circles = obj
            .get("circles")
            .ok_or("Missing 'circles' field")?
            .as_u64()
            .ok_or("'circles' must be a number")? as usize;

        let arcs_array = obj
            .get("arcs")
            .ok_or("Missing 'arcs' field")?
            .as_array()
            .ok_or("'arcs' must be an array")?;

        let mut arcs = HashSet::new();
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
            arcs.insert(coord);
        }

        let lines_array = obj
            .get("lines")
            .ok_or("Missing 'lines' field")?
            .as_array()
            .ok_or("'lines' must be an array")?;

        let mut lines = HashSet::new();
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
            lines.insert(coord);
        }

        Ok(Maze {
            circles,
            arcs,
            lines,
        })
    }
}

pub struct MazeSerializer;

impl MazeSerializer {
    pub fn serialize(maze: &Maze) -> Value {
        use serde_json::json;

        let arcs_array: Vec<Value> = maze
            .arcs()
            .iter()
            .map(|coord| {
                json!({
                    "circle": coord.circle(),
                    "arc": coord.arc_index()
                })
            })
            .collect();

        let lines_array: Vec<Value> = maze
            .lines()
            .iter()
            .map(|coord| {
                json!({
                    "circle": coord.circle(),
                    "arc": coord.arc_index()
                })
            })
            .collect();

        json!({
            "circles": maze.circles(),
            "arcs": arcs_array,
            "lines": lines_array
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
        use std::collections::HashSet;

        let seeds = [42, 123, 456, 789, 1024];

        for circles in 3..10 {
            for &seed in &seeds {
                let mut rng = StdRng::seed_from_u64(seed);
                let maze = factory(circles, &mut rng);

                let mut vertice_count = 0;
                for c in 1..=circles {
                    vertice_count += calc_total_arcs(c);
                }

                assert_eq!(
                    vertice_count,
                    maze.arcs().len() + maze.lines().len(),
                    "Maze with {} circles and seed {} should form a spanning tree",
                    circles,
                    seed
                );

                let mut visited = HashSet::new();
                let mut stack = vec![CircleCoord::create_with_arc_index(0, 0)];

                while let Some(v) = stack.pop() {
                    if visited.contains(&v) {
                        continue;
                    }
                    let neighbors = maze.accessible_neighbours(&v);
                    visited.insert(v);
                    for neighbor in neighbors {
                        stack.push(neighbor);
                    }
                }

                let mut reachable_vertice_count = 1;
                for c in 1..circles {
                    reachable_vertice_count += calc_total_arcs(c);
                }

                assert_eq!(
                    visited.len(),
                    reachable_vertice_count,
                    "All reachable vertices should be visited for {} circles with seed {}",
                    circles,
                    seed
                );
            }
        }
    }

    #[test]
    fn test_deserialize_maze_with_three_circles() {
        let json_str = include_str!("../tests/fixtures/maze_03_circles_00.json");
        let json_data: Value = serde_json::from_str(json_str).unwrap();

        let result = MazeDeserializer::deserialize(json_data);

        assert!(result.is_ok(), "Deserialization should succeed");
        let maze = result.unwrap();
        assert_eq!(maze.circles(), 3, "Maze should have 3 circles");
    }

    #[test]
    fn test_accessible_neighbours_4() {
        let json_str = include_str!("../tests/fixtures/maze_05_circles_00.json");
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
        let json_str = include_str!("../tests/fixtures/maze_05_circles_00.json");
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
        let json_str = include_str!("../tests/fixtures/maze_05_circles_00.json");
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
        let json_str = include_str!("../tests/fixtures/maze_05_circles_00.json");
        let json_data: Value = serde_json::from_str(json_str).unwrap();

        let maze = MazeDeserializer::deserialize(json_data).unwrap();
        let coord = CircleCoord::create_with_arc_index(3, 11);
        let neighbours = maze.accessible_neighbours(&coord);

        assert_eq!(neighbours.len(), 1);
        assert!(neighbours.contains(&CircleCoord::create_with_arc_index(3, 0)));
    }

    #[test]
    fn test_accessible_neighbours_circle_0() {
        let json_str = include_str!("../tests/fixtures/maze_05_circles_00.json");
        let json_data: Value = serde_json::from_str(json_str).unwrap();

        let maze = MazeDeserializer::deserialize(json_data).unwrap();
        let coord = CircleCoord::create_with_arc_index(0, 0);
        let neighbours = maze.accessible_neighbours(&coord);

        assert_eq!(neighbours.len(), 1);
        assert!(neighbours.contains(&CircleCoord::create_with_arc_index(1, 4)));
    }

    #[test]
    fn test_accessible_neighbours_circle_1() {
        let json_str = include_str!("../tests/fixtures/maze_05_circles_00.json");
        let json_data: Value = serde_json::from_str(json_str).unwrap();

        let maze = MazeDeserializer::deserialize(json_data).unwrap();
        let coord = CircleCoord::create_with_arc_index(1, 4);
        let neighbours = maze.accessible_neighbours(&coord);

        assert_eq!(neighbours.len(), 3);
        assert!(neighbours.contains(&CircleCoord::create_with_arc_index(0, 0)));
        assert!(neighbours.contains(&CircleCoord::create_with_arc_index(2, 8)));
        assert!(neighbours.contains(&CircleCoord::create_with_arc_index(2, 9)));
    }

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let json_str = include_str!("../tests/fixtures/maze_03_circles_00.json");
        let original_json: Value = serde_json::from_str(json_str).unwrap();
        let maze = MazeDeserializer::deserialize(original_json.clone()).unwrap();
        let serialized = MazeSerializer::serialize(&maze);
        let deserialized_maze = MazeDeserializer::deserialize(serialized).unwrap();

        assert_eq!(maze.circles(), deserialized_maze.circles());
        assert_eq!(maze.arcs(), deserialized_maze.arcs());
        assert_eq!(maze.lines(), deserialized_maze.lines());
    }

    #[test]
    fn test_serialize_generated_maze() {
        use rand::SeedableRng;
        use rand::rngs::StdRng;

        let mut rng = StdRng::seed_from_u64(42);
        let maze = factory(4, &mut rng);
        let serialized = MazeSerializer::serialize(&maze);
        let deserialized = MazeDeserializer::deserialize(serialized).unwrap();

        assert_eq!(maze.circles(), deserialized.circles());
        assert_eq!(maze.arcs(), deserialized.arcs());
        assert_eq!(maze.lines(), deserialized.lines());
    }
}
