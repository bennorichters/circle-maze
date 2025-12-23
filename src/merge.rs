use crate::building_blocks::CircleCoordinate;
use crate::maze::Maze;

fn coordinates_equal(a: &CircleCoordinate, b: &CircleCoordinate) -> bool {
    a.circle() == b.circle() && a.angle() == b.angle()
}

fn merge_tuples(
    tuples: Vec<(CircleCoordinate, CircleCoordinate)>,
) -> Vec<(CircleCoordinate, CircleCoordinate)> {
    let mut current = tuples;

    loop {
        let mut merged = false;

        for i in 0..current.len() {
            if let Some(j) = current.iter().skip(i + 1).position(|next| {
                coordinates_equal(&current[i].1, &next.0)
            }) {
                let j = j + i + 1;
                let new_tuple = (current[i].0.clone(), current[j].1.clone());
                let mut result = Vec::new();

                for (idx, tuple) in current.into_iter().enumerate() {
                    if idx == i {
                        result.push(new_tuple.clone());
                    } else if idx != j {
                        result.push(tuple);
                    }
                }

                current = result;
                merged = true;
                break;
            }
        }

        if !merged {
            return current;
        }
    }
}

pub fn merge_lines(maze: Maze) -> Vec<(CircleCoordinate, CircleCoordinate)> {
    let tuples: Vec<(CircleCoordinate, CircleCoordinate)> = maze
        .lines()
        .iter()
        .map(|line| {
            let original = CircleCoordinate::create_with_fraction(
                line.circle(),
                *line.angle(),
            )
            .expect("Failed to create coordinate");
            let next = line.next_out().expect("Failed to create next coordinate");
            (original, next)
        })
        .collect();

    merge_tuples(tuples)
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
}
