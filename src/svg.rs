use crate::maze::Maze;
use crate::merge::{merge_arcs, merge_lines};
use std::fs::File;
use std::io::Write;
use std::f64::consts::PI;

pub fn render(maze: &Maze) -> std::io::Result<()> {
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

    svg_content.push_str("</g></svg>\n");

    let mut file = File::create("maze.svg")?;
    file.write_all(svg_content.as_bytes())?;

    Ok(())
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

            let (start_x, start_y) = polar_to_cartesian(radius, start_angle);
            let (end_x, end_y) = polar_to_cartesian(radius, end_angle);

            content.push_str(&format!(
                r#"  <path d="M {:.2},{:.2} A {},{} 0 0 1 {:.2},{:.2}"/>
"#,
                start_x, start_y, radius, radius, end_x, end_y
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

fn angle_to_radians(angle: &fraction::Fraction) -> f64 {
    let degrees = (*angle.numer().unwrap() as f64) / (*angle.denom().unwrap() as f64);
    degrees * PI / 180.0
}

fn polar_to_cartesian(radius: usize, angle: &fraction::Fraction) -> (f64, f64) {
    let angle_rad = angle_to_radians(angle);
    let x = radius as f64 * angle_rad.cos();
    let y = radius as f64 * angle_rad.sin();
    (if x.abs() < 1e-10 { 0.0 } else { x }, if y.abs() < 1e-10 { 0.0 } else { y })
}
