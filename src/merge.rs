use crate::building_blocks::CircleCoordinate;
use crate::maze::Maze;

fn merge_coordinates<F>(
    coordinates: &[CircleCoordinate],
    get_next: F,
    allow_closed: bool,
) -> Vec<(CircleCoordinate, CircleCoordinate)>
where
    F: Fn(&CircleCoordinate) -> CircleCoordinate,
{
    let mut result: Vec<(CircleCoordinate, CircleCoordinate)> = Vec::new();

    for coord in coordinates.iter() {
        let start = coord.clone();
        let end = get_next(coord);

        let start_match = result.iter().position(|(_, e)| e == &start);
        let end_match = result.iter().position(|(s, _)| s == &end);

        match (start_match, end_match) {
            (Some(i), Some(j)) if i == j => {
                if allow_closed {
                    result[i].1 = end;
                } else {
                    continue;
                }
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

pub fn merge_lines(maze: &Maze) -> Vec<(CircleCoordinate, CircleCoordinate)> {
    merge_coordinates(maze.lines(), |line| line.next_out(), false)
}

pub fn merge_arcs(maze: &Maze) -> Vec<(CircleCoordinate, CircleCoordinate)> {
    merge_coordinates(maze.arcs(), |arc| arc.next_clockwise(), true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::MazeDeserializer;
    use serde_json::json;

    fn create_test_maze() -> Maze {
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

        MazeDeserializer::deserialize(json_data).expect("Failed to deserialize maze")
    }

    #[test]
    fn test_merge_lines_with_four_circles() {
        let maze = create_test_maze();
        let result = merge_lines(&maze);

        assert_eq!(result.len(), 9, "Expected 9 merged line pairs");
    }

    #[test]
    fn test_merge_arcs_with_four_circles() {
        let maze = create_test_maze();
        let result = merge_arcs(&maze);

        assert_eq!(result.len(), 8, "Expected 8 merged arc pairs");
    }
}
