use crate::circle_coord::{CircleCoord, calc_total_arcs};
use crate::maze::Maze;
use crate::merge::{merge_arcs, merge_lines};
use std::f64::consts::PI;

pub fn render_with_path(maze: &Maze, path: &[CircleCoord]) -> String {
    build_svg_content(maze, Some(path))
}

fn build_svg_content(maze: &Maze, path: Option<&[CircleCoord]>) -> String {
    let circles = maze.circles();
    let max_radius = circles * 10;
    let view_size = max_radius * 2 + 20;

    let mut svg_content = String::new();
    svg_content.push_str(&format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="{} {} {} {}" width="100%" height="100%" preserveAspectRatio="xMidYMid meet" shape-rendering="geometricPrecision">
<g fill="none" stroke="black" stroke-width="1" stroke-linecap="round">
"#,
        -(view_size as i32) / 2,
        -(view_size as i32) / 2,
        view_size,
        view_size
    ));

    svg_content.push_str(&render_arcs(maze));
    svg_content.push_str(&render_lines(maze));
    svg_content.push_str("</g>\n");

    if let Some(path_coords) = path {
        svg_content.push_str(r#"<g fill="none" stroke="purple" stroke-width="2" stroke-linecap="round">
"#);

        let (path_arcs, path_lines) = merge_path_segments(path_coords);

        for (start, end, is_clockwise) in path_arcs {
            let radius = calc_display_radius(start.circle());
            let start_angle = calc_display_angle(&start);
            let end_angle = calc_display_angle(&end);

            let start_degrees = fraction_to_degrees(&start_angle);
            let end_degrees = fraction_to_degrees(&end_angle);

            let sweep_flag = if is_clockwise { 1 } else { 0 };

            let angle_diff = if is_clockwise {
                if end_degrees >= start_degrees {
                    end_degrees - start_degrees
                } else {
                    end_degrees + 360.0 - start_degrees
                }
            } else {
                if start_degrees >= end_degrees {
                    start_degrees - end_degrees
                } else {
                    start_degrees + 360.0 - end_degrees
                }
            };

            let large_arc_flag = if angle_diff > 180.0 { 1 } else { 0 };

            let (start_x, start_y) = polar_to_cartesian(radius, &start_angle);
            let (end_x, end_y) = polar_to_cartesian(radius, &end_angle);

            svg_content.push_str(&format!(
                r#"  <path d="M {:.2},{:.2} A {},{} 0 {} {} {:.2},{:.2}"/>
"#,
                start_x, start_y, radius, radius, large_arc_flag, sweep_flag, end_x, end_y
            ));
        }

        for (start, end) in path_lines {
            let start_radius = calc_display_radius(start.circle());
            let start_angle = calc_display_angle(&start);
            let end_radius = calc_display_radius(end.circle());
            let end_angle = calc_display_angle(&end);

            let (start_x, start_y) = polar_to_cartesian(start_radius, &start_angle);
            let (end_x, end_y) = polar_to_cartesian(end_radius, &end_angle);

            svg_content.push_str(&format!(
                r#"  <line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}"/>
"#,
                start_x, start_y, end_x, end_y
            ));
        }

        svg_content.push_str("</g>\n");

        for (index, coord) in path_coords.iter().enumerate() {
            if index == 0 || index == path_coords.len() - 1 {
                let radius = calc_display_radius(coord.circle());
                let angle = calc_display_angle(coord);
                let (x, y) = polar_to_cartesian(radius, &angle);

                svg_content.push_str(&format!(
                    r#"<circle cx="{:.2}" cy="{:.2}" r="3" fill="red"/>
"#,
                    x, y
                ));
            }
        }
    }

    svg_content.push_str("</svg>\n");
    svg_content
}

fn calc_display_radius(circle: usize) -> usize {
    if circle == 0 { 0 } else { circle * 10 + 5 }
}

fn calc_display_angle(coord: &CircleCoord) -> fraction::Fraction {
    if coord.circle() == 0 {
        *coord.angle()
    } else {
        let total_arcs = calc_total_arcs(coord.circle());
        let angle_step = fraction::Fraction::from(360) / fraction::Fraction::from(total_arcs);
        let half_step = angle_step / fraction::Fraction::from(2);
        coord.angle() + half_step
    }
}

fn render_arcs(maze: &Maze) -> String {
    let mut content = String::new();
    let merged_arcs = merge_arcs(maze);

    for (start, end) in merged_arcs {
        let radius = start.circle() * 10;

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

            let mut angle_diff = end_degrees - start_degrees;
            if angle_diff < 0.0 {
                angle_diff += 360.0;
            }

            let large_arc_flag = if angle_diff > 180.0 { 1 } else { 0 };

            let (start_x, start_y) = polar_to_cartesian(radius, start_angle);
            let (end_x, end_y) = polar_to_cartesian(radius, end_angle);

            content.push_str(&format!(
                r#"  <path d="M {:.2},{:.2} A {},{} 0 {} 1 {:.2},{:.2}"/>
"#,
                start_x, start_y, radius, radius, large_arc_flag, end_x, end_y
            ));
        }
    }

    content
}

fn render_lines(maze: &Maze) -> String {
    let mut content = String::new();
    let merged_lines = merge_lines(maze);

    for (start, end) in merged_lines {
        let start_angle = start.angle();
        let start_radius = start.circle() * 10;
        let end_angle = end.angle();
        let end_radius = end.circle() * 10;

        let (start_x, start_y) = polar_to_cartesian(start_radius, start_angle);
        let (end_x, end_y) = polar_to_cartesian(end_radius, end_angle);

        content.push_str(&format!(
            r#"  <line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}"/>
"#,
            start_x, start_y, end_x, end_y
        ));
    }

    content
}

fn merge_path_segments(
    path: &[CircleCoord],
) -> (Vec<(CircleCoord, CircleCoord, bool)>, Vec<(CircleCoord, CircleCoord)>) {
    let mut arcs = Vec::new();
    let mut lines = Vec::new();

    if path.len() < 2 {
        return (arcs, lines);
    }

    let mut i = 0;
    while i < path.len() - 1 {
        let start = path[i].clone();
        let mut j = i + 1;

        if path[i].circle() == path[j].circle() {
            while j < path.len() - 1 && path[j].circle() == path[j + 1].circle() {
                j += 1;
            }

            let total_arcs = calc_total_arcs(start.circle());
            let start_idx = start.arc_index();
            let next_idx = path[i + 1].arc_index();

            let forward_dist = (next_idx + total_arcs - start_idx) % total_arcs;
            let is_clockwise = forward_dist > 0 && forward_dist <= total_arcs / 2;

            arcs.push((start, path[j].clone(), is_clockwise));
        } else {
            if start.angle() == path[j].angle() {
                while j < path.len() - 1
                    && path[j].circle() != path[j + 1].circle()
                    && path[j].angle() == path[j + 1].angle()
                {
                    j += 1;
                }
            }
            lines.push((start, path[j].clone()));
        }

        i = j;
    }

    (arcs, lines)
}

fn fraction_to_degrees(angle: &fraction::Fraction) -> f64 {
    (*angle.numer().unwrap() as f64) / (*angle.denom().unwrap() as f64)
}

fn angle_to_radians(angle: &fraction::Fraction) -> f64 {
    let degrees = fraction_to_degrees(angle);
    degrees * PI / 180.0
}

fn polar_to_cartesian(radius: usize, angle: &fraction::Fraction) -> (f64, f64) {
    let angle_rad = angle_to_radians(angle);
    let x = radius as f64 * angle_rad.cos();
    let y = radius as f64 * angle_rad.sin();
    (
        if x.abs() < 1e-10 { 0.0 } else { x },
        if y.abs() < 1e-10 { 0.0 } else { y },
    )
}
