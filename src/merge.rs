use crate::building_blocks::CircleCoordinate;
use crate::maze::Maze;

pub fn merge_lines(maze: Maze) -> Vec<(CircleCoordinate, CircleCoordinate)> {
    let mut result: Vec<(CircleCoordinate, CircleCoordinate)> = Vec::new();

    for line in maze.lines().iter() {
        let start = line.clone();
        let end = line.next_out().expect("Failed to create next coordinate");

        let start_match = result.iter().position(|(_, e)| e == &start);
        let end_match = result.iter().position(|(s, _)| s == &end);

        match (start_match, end_match) {
            (Some(i), Some(j)) if i == j => {
                continue;
            }
            (Some(i), Some(j)) => {
                let merged_tuple = (result[i].0.clone(), result[j].1.clone());
                let (first, second) = if i < j { (i, j) } else { (j, i) };
                result.remove(second);
                result.remove(first);
                result.push(merged_tuple);
            }
            (Some(i), None) => {
                result[i].1 = end;
            }
            (None, Some(j)) => {
                result[j].0 = start;
            }
            (None, None) => {
                result.push((start, end));
            }
        }
    }

    result
}

pub fn merge_arcs(maze: Maze) -> Vec<(CircleCoordinate, CircleCoordinate)> {
    let mut result: Vec<(CircleCoordinate, CircleCoordinate)> = Vec::new();

    for arc in maze.arcs().iter() {
        let start = arc.clone();
        let end = arc.next_clockwise().expect("Failed to create next coordinate");

        let start_match = result.iter().position(|(_, e)| e == &start);
        let end_match = result.iter().position(|(s, _)| s == &end);

        match (start_match, end_match) {
            (Some(i), Some(j)) if i == j => {
                result[i].1 = end;
            }
            (Some(i), Some(j)) => {
                let merged_tuple = (result[i].0.clone(), result[j].1.clone());
                let (first, second) = if i < j { (i, j) } else { (j, i) };
                result.remove(second);
                result.remove(first);
                result.push(merged_tuple);
            }
            (Some(i), None) => {
                result[i].1 = end;
            }
            (None, Some(j)) => {
                result[j].0 = start;
            }
            (None, None) => {
                result.push((start, end));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::MazeDeserializer;
    use serde_json::json;

    #[test]
    fn test_merge_lines_with_four_circles() {
        let json_data = json!({
            "circles": 4,
            "arcs": [
                {"circle": 1, "arc": 0},
                {"circle": 1, "arc": 3},
                {"circle": 1, "arc": 4},
                {"circle": 1, "arc": 5},
                {"circle": 2, "arc": 1},
                {"circle": 2, "arc": 2},
                {"circle": 2, "arc": 4},
                {"circle": 2, "arc": 5},
                {"circle": 2, "arc": 6},
                {"circle": 2, "arc": 9},
                {"circle": 2, "arc": 11},
                {"circle": 3, "arc": 3},
                {"circle": 3, "arc": 4},
                {"circle": 3, "arc": 8},
                {"circle": 3, "arc": 9},
                {"circle": 3, "arc": 10},
                {"circle": 4, "arc": 0},
                {"circle": 4, "arc": 1},
                {"circle": 4, "arc": 2},
                {"circle": 4, "arc": 3},
                {"circle": 4, "arc": 4},
                {"circle": 4, "arc": 5},
                {"circle": 4, "arc": 6},
                {"circle": 4, "arc": 7},
                {"circle": 4, "arc": 8},
                {"circle": 4, "arc": 9},
                {"circle": 4, "arc": 10},
                {"circle": 4, "arc": 11},
                {"circle": 4, "arc": 12},
                {"circle": 4, "arc": 13},
                {"circle": 4, "arc": 14},
                {"circle": 4, "arc": 15},
                {"circle": 4, "arc": 16},
                {"circle": 4, "arc": 17},
                {"circle": 4, "arc": 18},
                {"circle": 4, "arc": 19},
                {"circle": 4, "arc": 20},
                {"circle": 4, "arc": 21},
                {"circle": 4, "arc": 22},
                {"circle": 4, "arc": 23}
            ],
            "lines": [
                {"circle": 1, "arc": 0},
                {"circle": 1, "arc": 1},
                {"circle": 1, "arc": 2},
                {"circle": 1, "arc": 3},
                {"circle": 2, "arc": 0},
                {"circle": 2, "arc": 2},
                {"circle": 2, "arc": 5},
                {"circle": 2, "arc": 6},
                {"circle": 2, "arc": 8},
                {"circle": 2, "arc": 10},
                {"circle": 3, "arc": 1},
                {"circle": 3, "arc": 2},
                {"circle": 3, "arc": 7},
                {"circle": 3, "arc": 8}
            ]
        });

        let maze = MazeDeserializer::deserialize(json_data)
            .expect("Failed to deserialize maze");

        let result = merge_lines(maze);

        assert_eq!(result.len(), 9, "Expected 9 merged line pairs");
    }

    #[test]
    fn test_merge_arcs_with_four_circles() {
        let json_data = json!({
            "circles": 4,
            "arcs": [
                {"circle": 1, "arc": 0},
                {"circle": 1, "arc": 3},
                {"circle": 1, "arc": 4},
                {"circle": 1, "arc": 5},
                {"circle": 2, "arc": 1},
                {"circle": 2, "arc": 2},
                {"circle": 2, "arc": 4},
                {"circle": 2, "arc": 5},
                {"circle": 2, "arc": 6},
                {"circle": 2, "arc": 9},
                {"circle": 2, "arc": 11},
                {"circle": 3, "arc": 3},
                {"circle": 3, "arc": 4},
                {"circle": 3, "arc": 8},
                {"circle": 3, "arc": 9},
                {"circle": 3, "arc": 10},
                {"circle": 4, "arc": 0},
                {"circle": 4, "arc": 1},
                {"circle": 4, "arc": 2},
                {"circle": 4, "arc": 3},
                {"circle": 4, "arc": 4},
                {"circle": 4, "arc": 5},
                {"circle": 4, "arc": 6},
                {"circle": 4, "arc": 7},
                {"circle": 4, "arc": 8},
                {"circle": 4, "arc": 9},
                {"circle": 4, "arc": 10},
                {"circle": 4, "arc": 11},
                {"circle": 4, "arc": 12},
                {"circle": 4, "arc": 13},
                {"circle": 4, "arc": 14},
                {"circle": 4, "arc": 15},
                {"circle": 4, "arc": 16},
                {"circle": 4, "arc": 17},
                {"circle": 4, "arc": 18},
                {"circle": 4, "arc": 19},
                {"circle": 4, "arc": 20},
                {"circle": 4, "arc": 21},
                {"circle": 4, "arc": 22},
                {"circle": 4, "arc": 23}
            ],
            "lines": [
                {"circle": 1, "arc": 0},
                {"circle": 1, "arc": 1},
                {"circle": 1, "arc": 2},
                {"circle": 1, "arc": 3},
                {"circle": 2, "arc": 0},
                {"circle": 2, "arc": 2},
                {"circle": 2, "arc": 5},
                {"circle": 2, "arc": 6},
                {"circle": 2, "arc": 8},
                {"circle": 2, "arc": 10},
                {"circle": 3, "arc": 1},
                {"circle": 3, "arc": 2},
                {"circle": 3, "arc": 7},
                {"circle": 3, "arc": 8}
            ]
        });

        let maze = MazeDeserializer::deserialize(json_data)
            .expect("Failed to deserialize maze");

        let result = merge_arcs(maze);

        assert_eq!(result.len(), 8, "Expected 8 merged arc pairs");
    }
}
