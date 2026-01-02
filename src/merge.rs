use crate::circle_coord::CircleCoord;
use crate::maze::Maze;

fn merge_coordinates<'a, I, F>(
    coordinates: I,
    get_next: F,
    allow_closed: bool,
) -> Vec<(CircleCoord, CircleCoord)>
where
    I: IntoIterator<Item = &'a CircleCoord>,
    F: Fn(&CircleCoord) -> CircleCoord,
{
    let mut result: Vec<(CircleCoord, CircleCoord)> = Vec::new();

    for coord in coordinates {
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

pub fn merge_lines(maze: &Maze) -> Vec<(CircleCoord, CircleCoord)> {
    merge_coordinates(maze.lines(), |line| line.next_out(), false)
}

pub fn merge_arcs(maze: &Maze) -> Vec<(CircleCoord, CircleCoord)> {
    merge_coordinates(maze.arcs(), |arc| arc.next_clockwise(), true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::MazeDeserializer;

    fn create_test_maze() -> Maze {
        let json_str = include_str!("../tests/fixtures/maze_04_circles_00.json");
        let json_data: serde_json::Value = serde_json::from_str(json_str).unwrap();

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
