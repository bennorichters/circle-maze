use crate::maze::Maze;
use crate::merge::{merge_arcs, merge_lines};

use super::geometry::{
    calc_large_arc_flag, create_svg_arc_path, fraction_to_degrees, normalize_angle_diff,
    polar_to_cartesian, CIRCLE_RADIUS_STEP,
};

pub fn render_arcs(maze: &Maze) -> String {
    let mut content = String::new();
    let merged_arcs = merge_arcs(maze);

    for (start, end) in merged_arcs {
        let radius = start.circle() * CIRCLE_RADIUS_STEP;

        if start == end {
            content.push_str(&format!(
                r#"  <circle cx="0" cy="0" r="{}"/>
"#,
                radius
            ));
        } else {
            let start_angle = start.angle();
            let end_angle = end.angle();

            let start_degrees = fraction_to_degrees(start_angle);
            let end_degrees = fraction_to_degrees(end_angle);

            let angle_diff = normalize_angle_diff(end_degrees - start_degrees);
            let large_arc_flag = calc_large_arc_flag(angle_diff);

            content.push_str(&create_svg_arc_path(
                radius,
                start_angle,
                end_angle,
                1,
                large_arc_flag,
            ));
        }
    }

    content
}

pub fn render_lines(maze: &Maze) -> String {
    let mut content = String::new();
    let merged_lines = merge_lines(maze);

    for (start, end) in merged_lines {
        let start_angle = start.angle();
        let start_radius = start.circle() * CIRCLE_RADIUS_STEP;
        let end_angle = end.angle();
        let end_radius = end.circle() * CIRCLE_RADIUS_STEP;

        let start = polar_to_cartesian(start_radius, start_angle);
        let end = polar_to_cartesian(end_radius, end_angle);

        content.push_str(&format!(
            r#"  <line x1="{:.8}" y1="{:.8}" x2="{:.8}" y2="{:.8}"/>
"#,
            start.x, start.y, end.x, end.y
        ));
    }

    content
}
