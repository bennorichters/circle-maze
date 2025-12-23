use crate::building_blocks::CircleCoordinate;
use crate::maze::Maze;

pub fn merge_lines(maze: Maze) -> Vec<(CircleCoordinate, CircleCoordinate)> {
    maze.lines()
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
        .collect()
}
