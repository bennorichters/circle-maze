use crate::building_blocks::CircleCoordinate;
use crate::maze::Maze;

fn coordinates_equal(a: &CircleCoordinate, b: &CircleCoordinate) -> bool {
    a.circle() == b.circle() && a.angle() == b.angle()
}

fn merge_tuples(
    tuples: Vec<(CircleCoordinate, CircleCoordinate)>,
) -> Vec<(CircleCoordinate, CircleCoordinate)> {
    let mut result = Vec::new();
    let mut iter = tuples.into_iter();

    if let Some(mut current) = iter.next() {
        for next in iter {
            if coordinates_equal(&current.1, &next.0) {
                current = (current.0, next.1);
            } else {
                result.push(current);
                current = next;
            }
        }
        result.push(current);
    }

    result
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
