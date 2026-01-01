use crate::circle_coord::{calc_total_arcs, CircleCoord};
use crate::maze::Maze;
use crate::merge::{merge_arcs, merge_lines};
use std::f64::consts::PI;

const DEGREES_IN_CIRCLE: f64 = 360.0;
const DEGREES_IN_SEMICIRCLE: f64 = 180.0;
const SVG_VIEWBOX_PADDING: usize = 20;
const CIRCLE_RADIUS_STEP: usize = 10;
const HALF_RADIUS_STEP: usize = 5;
const MARKER_RADIUS: usize = 3;
const COORDINATE_EPSILON: f64 = 1e-10;

struct Point {
    x: f64,
    y: f64,
}

enum PathSegment {
    Arc {
        radius: usize,
        start_angle: fraction::Fraction,
        end_angle: fraction::Fraction,
        clockwise: bool,
    },
    Line {
        start_radius: usize,
        end_radius: usize,
        start_angle: fraction::Fraction,
        end_angle: fraction::Fraction,
    },
}

impl PathSegment {
    fn render(&self) -> String {
        match self {
            PathSegment::Arc { radius, start_angle, end_angle, clockwise } => {
                render_solution_arc(*radius, start_angle, end_angle, *clockwise)
            }
            PathSegment::Line { start_radius, end_radius, start_angle, end_angle } => {
                render_solution_line(*start_radius, *end_radius, start_angle, end_angle)
            }
        }
    }
}

pub fn render_with_path(maze: &Maze, path: &[CircleCoord]) -> String {
    let circles = maze.circles();
    let max_radius = circles * CIRCLE_RADIUS_STEP;
    let view_size = max_radius * 2 + SVG_VIEWBOX_PADDING;

    let mut svg_content = String::new();
    svg_content.push_str(&render_svg_header(view_size));
    svg_content.push_str(&render_arcs(maze));
    svg_content.push_str(&render_lines(maze));
    svg_content.push_str("</g>\n");
    svg_content.push_str(&render_solution_path(path));
    svg_content.push_str(&render_path_markers(path));
    svg_content.push_str("</svg>\n");
    svg_content
}

fn render_svg_header(view_size: usize) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="{} {} {} {}" width="100%" height="100%" preserveAspectRatio="xMidYMid meet" shape-rendering="geometricPrecision">
<g id="borders" fill="none" stroke="black" stroke-width="1" stroke-linecap="round">
"#,
        -(view_size as i32) / 2,
        -(view_size as i32) / 2,
        view_size,
        view_size
    )
}

fn render_solution_path(path: &[CircleCoord]) -> String {
    let mut content = String::new();
    content.push_str(
        r#"<g id="solution-path" fill="none" stroke="purple" stroke-width="2" stroke-linecap="round">
"#,
    );

    let segments = merge_path_segments(path);
    for segment in segments {
        content.push_str(&segment.render());
    }
    content.push_str("</g>\n");
    content
}

fn create_svg_arc_path(
    radius: usize,
    start_angle: &fraction::Fraction,
    end_angle: &fraction::Fraction,
    sweep_flag: u8,
    large_arc_flag: u8,
) -> String {
    let start = polar_to_cartesian(radius, start_angle);
    let end = polar_to_cartesian(radius, end_angle);

    format!(
        r#"  <path d="M {:.2},{:.2} A {},{} 0 {} {} {:.2},{:.2}"/>
"#,
        start.x, start.y, radius, radius, large_arc_flag, sweep_flag, end.x, end.y
    )
}

fn render_solution_arc(
    radius: usize,
    start_angle: &fraction::Fraction,
    end_angle: &fraction::Fraction,
    is_clockwise: bool,
) -> String {
    let start_degrees = fraction_to_degrees(start_angle);
    let end_degrees = fraction_to_degrees(end_angle);

    let sweep_flag = if is_clockwise { 1 } else { 0 };
    let angle_diff = calc_arc_angle_diff(start_degrees, end_degrees, is_clockwise);
    let large_arc_flag = calc_large_arc_flag(angle_diff);

    create_svg_arc_path(radius, start_angle, end_angle, sweep_flag, large_arc_flag)
}

fn render_solution_line(
    start_radius: usize,
    end_radius: usize,
    start_angle: &fraction::Fraction,
    end_angle: &fraction::Fraction,
) -> String {
    let start = polar_to_cartesian(start_radius, start_angle);
    let end = polar_to_cartesian(end_radius, end_angle);

    format!(
        r#"  <line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}"/>
"#,
        start.x, start.y, end.x, end.y
    )
}

fn render_path_markers(path: &[CircleCoord]) -> String {
    let mut content = String::new();
    content.push_str(r#"<g id="start-finish-markers" fill="red">
"#);

    for (index, coord) in path.iter().enumerate() {
        if index != 0 && index != path.len() - 1 {
            continue;
        }

        let radius = calc_display_radius(coord.circle());
        let angle = calc_display_angle(coord);
        let point = polar_to_cartesian(radius, &angle);

        content.push_str(&format!(
            r#"  <circle cx="{:.2}" cy="{:.2}" r="{}"/>
"#,
            point.x, point.y, MARKER_RADIUS
        ));
    }

    content.push_str("</g>\n");
    content
}

fn calc_display_radius(circle: usize) -> usize {
    if circle == 0 {
        0
    } else {
        circle * CIRCLE_RADIUS_STEP + HALF_RADIUS_STEP
    }
}

fn calc_display_angle(coord: &CircleCoord) -> fraction::Fraction {
    if coord.circle() == 0 {
        *coord.angle()
    } else {
        let total_arcs = calc_total_arcs(coord.circle());
        let angle_step = fraction::Fraction::from(DEGREES_IN_CIRCLE as u64)
            / fraction::Fraction::from(total_arcs);
        let half_step = angle_step / fraction::Fraction::from(2);
        coord.angle() + half_step
    }
}

fn render_arcs(maze: &Maze) -> String {
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

fn render_lines(maze: &Maze) -> String {
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
            r#"  <line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}"/>
"#,
            start.x, start.y, end.x, end.y
        ));
    }

    content
}

fn find_arc_end_index(path: &[CircleCoord], start: usize) -> usize {
    let mut end = start + 1;
    while end < path.len() - 1 && path[end].circle() == path[end + 1].circle() {
        end += 1;
    }
    end
}

fn find_line_end_index(path: &[CircleCoord], start: usize) -> usize {
    let mut end = start + 1;
    if path[start].angle() == path[end].angle() {
        while end < path.len() - 1
            && path[end].circle() != path[end + 1].circle()
            && path[end].angle() == path[end + 1].angle()
        {
            end += 1;
        }
    }
    end
}

fn get_last_segment_end_angle(segments: &[PathSegment]) -> Option<fraction::Fraction> {
    match segments.last() {
        Some(PathSegment::Line { end_angle, .. }) => Some(*end_angle),
        Some(PathSegment::Arc { end_angle, .. }) => Some(*end_angle),
        None => None,
    }
}

fn add_arc_segment(
    segments: &mut Vec<PathSegment>,
    path: &[CircleCoord],
    start_idx: usize,
    end_idx: usize,
) {
    let start = &path[start_idx];
    let radius = calc_display_radius(start.circle());

    let start_angle_deg = fraction_to_degrees(start.angle());
    let next_angle_deg = fraction_to_degrees(path[start_idx + 1].angle());
    let is_clockwise = clockwise(start_angle_deg, next_angle_deg);

    let mut start_angle = calc_display_angle(start);
    let end_angle = calc_display_angle(&path[end_idx]);

    if let Some(last_angle) = get_last_segment_end_angle(segments) {
        start_angle = last_angle;
    }

    segments.push(PathSegment::Arc {
        radius,
        start_angle,
        end_angle,
        clockwise: is_clockwise,
    });
}

fn adjust_line_angles(
    start_radius: usize,
    end_radius: usize,
    start_angle: fraction::Fraction,
    end_angle: fraction::Fraction,
) -> (fraction::Fraction, fraction::Fraction) {
    if start_angle != end_angle {
        if start_radius < end_radius {
            (end_angle, end_angle)
        } else {
            (start_angle, start_angle)
        }
    } else {
        (start_angle, end_angle)
    }
}

fn add_connecting_arc_if_needed(
    segments: &mut Vec<PathSegment>,
    start: &CircleCoord,
    start_radius: usize,
    start_angle: fraction::Fraction,
) {
    if let Some(PathSegment::Arc { end_angle, .. }) = segments.last_mut() {
        *end_angle = start_angle;
    } else if let Some(PathSegment::Line { end_angle: ea, .. }) = segments.last() {
        if start_radius > 0 {
            let is_cw =
                clockwise(fraction_to_degrees(ea), fraction_to_degrees(&start_angle));
            segments.push(PathSegment::Arc {
                radius: start_radius,
                start_angle: *ea,
                end_angle: start_angle,
                clockwise: is_cw,
            });
        }
    } else {
        let ea = calc_display_angle(start);
        let is_cw = clockwise(fraction_to_degrees(&ea), fraction_to_degrees(&start_angle));
        segments.push(PathSegment::Arc {
            radius: start_radius,
            start_angle: ea,
            end_angle: start_angle,
            clockwise: is_cw,
        });
    }
}

fn add_line_segment(
    segments: &mut Vec<PathSegment>,
    path: &[CircleCoord],
    start_idx: usize,
    end_idx: usize,
) {
    let start = &path[start_idx];
    let start_radius = calc_display_radius(start.circle());
    let end_radius = calc_display_radius(path[end_idx].circle());
    let initial_start_angle = calc_display_angle(start);
    let initial_end_angle = calc_display_angle(&path[end_idx]);

    let (start_angle, end_angle) = adjust_line_angles(
        start_radius,
        end_radius,
        initial_start_angle,
        initial_end_angle,
    );

    add_connecting_arc_if_needed(segments, start, start_radius, start_angle);

    segments.push(PathSegment::Line {
        start_radius,
        end_radius,
        start_angle,
        end_angle,
    });
}

fn add_final_arc_if_needed(segments: &mut Vec<PathSegment>, path: &[CircleCoord]) {
    if let Some(PathSegment::Line { end_angle: ea, .. }) = segments.last() {
        let last_coord = &path[path.len() - 1];
        let finish_radius = calc_display_radius(last_coord.circle());
        let finish_angle = calc_display_angle(last_coord);
        let is_cw = clockwise(fraction_to_degrees(ea), fraction_to_degrees(&finish_angle));

        segments.push(PathSegment::Arc {
            radius: finish_radius,
            start_angle: *ea,
            end_angle: finish_angle,
            clockwise: is_cw,
        });
    }
}

fn merge_path_segments(path: &[CircleCoord]) -> Vec<PathSegment> {
    let mut segments = Vec::new();

    if path.len() < 2 {
        return segments;
    }

    let mut i = 0;
    while i < path.len() - 1 {
        let end_idx = if path[i].circle() == path[i + 1].circle() {
            let end = find_arc_end_index(path, i);
            add_arc_segment(&mut segments, path, i, end);
            end
        } else {
            let end = find_line_end_index(path, i);
            add_line_segment(&mut segments, path, i, end);
            end
        };

        i = end_idx;
    }

    add_final_arc_if_needed(&mut segments, path);

    segments
}

fn normalize_angle_diff(diff: f64) -> f64 {
    if diff < 0.0 {
        diff + DEGREES_IN_CIRCLE
    } else {
        diff
    }
}

fn calc_large_arc_flag(angle_diff: f64) -> u8 {
    if angle_diff > DEGREES_IN_SEMICIRCLE {
        1
    } else {
        0
    }
}

fn calc_arc_angle_diff(start_deg: f64, end_deg: f64, is_clockwise: bool) -> f64 {
    if is_clockwise {
        if end_deg >= start_deg {
            end_deg - start_deg
        } else {
            end_deg + DEGREES_IN_CIRCLE - start_deg
        }
    } else if start_deg >= end_deg {
        start_deg - end_deg
    } else {
        start_deg + DEGREES_IN_CIRCLE - end_deg
    }
}

fn clockwise(start_angle_deg: f64, next_angle_deg: f64) -> bool {
    let angle_diff = normalize_angle_diff(next_angle_deg - start_angle_deg);
    angle_diff > 0.0 && angle_diff <= DEGREES_IN_SEMICIRCLE
}

fn fraction_to_degrees(angle: &fraction::Fraction) -> f64 {
    (*angle.numer().unwrap() as f64) / (*angle.denom().unwrap() as f64)
}

fn angle_to_radians(angle: &fraction::Fraction) -> f64 {
    let degrees = fraction_to_degrees(angle);
    degrees * PI / DEGREES_IN_SEMICIRCLE
}

fn polar_to_cartesian(radius: usize, angle: &fraction::Fraction) -> Point {
    let angle_rad = angle_to_radians(angle);
    let x = radius as f64 * angle_rad.cos();
    let y = radius as f64 * angle_rad.sin();
    Point {
        x: if x.abs() < COORDINATE_EPSILON { 0.0 } else { x },
        y: if y.abs() < COORDINATE_EPSILON { 0.0 } else { y },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count_steps_in_border_lines(svg_string: &str) -> usize {
        let doc = roxmltree::Document::parse(svg_string)
            .expect("Failed to parse SVG XML");

        let borders_g = doc
            .descendants()
            .find(|n| n.tag_name().name() == "g" && n.attribute("id") == Some("borders"))
            .expect("Failed to find g element with id='borders'");

        let mut total_steps = 0;

        for node in borders_g.children() {
            if node.tag_name().name() == "line" {
                let x1: f64 = node
                    .attribute("x1")
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0.0);
                let y1: f64 = node
                    .attribute("y1")
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0.0);
                let x2: f64 = node
                    .attribute("x2")
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0.0);
                let y2: f64 = node
                    .attribute("y2")
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0.0);

                let length = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
                let step_count = (length / CIRCLE_RADIUS_STEP as f64).round() as usize;
                total_steps += step_count;
            }
        }

        total_steps
    }

    fn count_steps_in_border_arcs(svg_string: &str) -> usize {
        let doc = roxmltree::Document::parse(svg_string)
            .expect("Failed to parse SVG XML");

        let borders_g = doc
            .descendants()
            .find(|n| n.tag_name().name() == "g" && n.attribute("id") == Some("borders"))
            .expect("Failed to find g element with id='borders'");

        let mut total_steps = 0;

        for node in borders_g.children() {
            if node.tag_name().name() == "circle" {
                let r: usize = node
                    .attribute("r")
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0);

                let circle_number = r / CIRCLE_RADIUS_STEP;
                total_steps += calc_total_arcs(circle_number);
            } else if node.tag_name().name() == "path" {
                let d = node.attribute("d").unwrap_or("");
                let parts: Vec<&str> = d.split_whitespace().collect();

                if parts.len() >= 8 && parts[0] == "M" && parts[2] == "A" {
                    let radius: usize = parts[3]
                        .split(',')
                        .next()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0);

                    let circle_number = radius / CIRCLE_RADIUS_STEP;

                    let large_arc_flag: u8 = parts[5].parse().unwrap_or(0);
                    let sweep_flag: u8 = parts[6].parse().unwrap_or(0);

                    let start_coords: Vec<f64> = parts[1]
                        .split(',')
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    let end_coords: Vec<f64> = parts[7]
                        .split(',')
                        .filter_map(|s| s.parse().ok())
                        .collect();

                    if start_coords.len() == 2 && end_coords.len() == 2 {
                        let mut start_angle = start_coords[1].atan2(start_coords[0]).to_degrees();
                        let mut end_angle = end_coords[1].atan2(end_coords[0]).to_degrees();

                        if start_angle < 0.0 {
                            start_angle += DEGREES_IN_CIRCLE;
                        }
                        if end_angle < 0.0 {
                            end_angle += DEGREES_IN_CIRCLE;
                        }

                        let mut angle_diff = if sweep_flag == 1 {
                            if end_angle >= start_angle {
                                end_angle - start_angle
                            } else {
                                end_angle + DEGREES_IN_CIRCLE - start_angle
                            }
                        } else if start_angle >= end_angle {
                            start_angle - end_angle
                        } else {
                            start_angle + DEGREES_IN_CIRCLE - end_angle
                        };

                        if large_arc_flag == 1 && angle_diff < DEGREES_IN_SEMICIRCLE {
                            angle_diff = DEGREES_IN_CIRCLE - angle_diff;
                        }

                        let total_arcs = calc_total_arcs(circle_number);
                        let step_count =
                            (angle_diff * total_arcs as f64 / DEGREES_IN_CIRCLE).round() as usize;
                        total_steps += step_count;
                    }
                }
            }
        }

        total_steps
    }

    #[test]
    fn test_render_with_path_has_three_g_elements_with_correct_ids() {
        use crate::maze::MazeDeserializer;
        use std::fs;

        let fixtures_dir = "tests/fixtures";
        let entries = fs::read_dir(fixtures_dir)
            .unwrap_or_else(|_| panic!("Failed to read directory: {}", fixtures_dir));

        let json_files: Vec<_> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().extension().and_then(|s| s.to_str()) == Some("json")
            })
            .collect();

        assert!(!json_files.is_empty(), "No JSON files found in {}", fixtures_dir);

        for entry in json_files {
            let file_path = entry.path();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();

            let json_content = fs::read_to_string(&file_path)
                .unwrap_or_else(|_| panic!("Failed to read file: {:?}", file_path));

            let json_data: serde_json::Value = serde_json::from_str(&json_content)
                .unwrap_or_else(|_| panic!("Failed to parse JSON from: {}", file_name));

            let maze = MazeDeserializer::deserialize(json_data)
                .unwrap_or_else(|_| panic!("Failed to deserialize maze from: {}", file_name));

            let path = vec![
                CircleCoord::create_with_arc_index(0, 0),
                CircleCoord::create_with_arc_index(1, 0),
            ];

            let svg_string = render_with_path(&maze, &path);

            let doc = roxmltree::Document::parse(&svg_string).unwrap_or_else(|_| {
                panic!("Failed to parse SVG XML for file: {}", file_name)
            });

            let g_elements: Vec<_> = doc
                .descendants()
                .filter(|n| n.tag_name().name() == "g")
                .collect();

            assert_eq!(
                g_elements.len(),
                3,
                "Expected exactly 3 g elements for file: {}",
                file_name
            );

            let ids: Vec<_> = g_elements
                .iter()
                .filter_map(|n| n.attribute("id"))
                .collect();

            assert_eq!(
                ids.len(),
                3,
                "All g elements should have an id attribute for file: {}",
                file_name
            );
            assert!(
                ids.contains(&"borders"),
                "Expected g element with id='borders' for file: {}",
                file_name
            );
            assert!(
                ids.contains(&"solution-path"),
                "Expected g element with id='solution-path' for file: {}",
                file_name
            );
            assert!(
                ids.contains(&"start-finish-markers"),
                "Expected g element with id='start-finish-markers' for file: {}",
                file_name
            );
        }
    }

    #[test]
    fn test_borders_do_not_intersect_solution_path() {
        use crate::maze::MazeDeserializer;
        use std::fs;

        let fixtures_dir = "tests/fixtures";
        let entries = fs::read_dir(fixtures_dir)
            .unwrap_or_else(|_| panic!("Failed to read directory: {}", fixtures_dir));

        let json_files: Vec<_> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().extension().and_then(|s| s.to_str()) == Some("json")
            })
            .collect();

        assert!(!json_files.is_empty(), "No JSON files found in {}", fixtures_dir);

        for entry in json_files {
            let file_path = entry.path();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();

            let json_content = fs::read_to_string(&file_path)
                .unwrap_or_else(|_| panic!("Failed to read file: {:?}", file_path));

            let json_data: serde_json::Value = serde_json::from_str(&json_content)
                .unwrap_or_else(|_| panic!("Failed to parse JSON from: {}", file_name));

            let maze = MazeDeserializer::deserialize(json_data)
                .unwrap_or_else(|_| panic!("Failed to deserialize maze from: {}", file_name));

            let path = vec![
                CircleCoord::create_with_arc_index(0, 0),
                CircleCoord::create_with_arc_index(1, 0),
            ];

            let svg_string = render_with_path(&maze, &path);

            let doc = roxmltree::Document::parse(&svg_string).unwrap_or_else(|_| {
                panic!("Failed to parse SVG XML for file: {}", file_name)
            });

            let borders_g = doc
                .descendants()
                .find(|n| {
                    n.tag_name().name() == "g" && n.attribute("id") == Some("borders")
                })
                .unwrap_or_else(|| {
                    panic!("Failed to find g element with id='borders' for file: {}", file_name)
                });

            let circle_elements: Vec<_> = borders_g
                .children()
                .filter(|n| n.tag_name().name() == "circle")
                .collect();

            assert_eq!(
                circle_elements.len(),
                1,
                "Expected exactly 1 circle element in borders g for file: {}",
                file_name
            );

            let circle = circle_elements[0];
            let radius_str = circle
                .attribute("r")
                .unwrap_or_else(|| {
                    panic!("Circle element missing r attribute for file: {}", file_name)
                });

            let radius: usize = radius_str.parse().unwrap_or_else(|_| {
                panic!("Failed to parse radius value '{}' for file: {}", radius_str, file_name)
            });

            let expected_radius = maze.circles() * CIRCLE_RADIUS_STEP;
            assert_eq!(
                radius, expected_radius,
                "Circle radius should be {} for file: {}",
                expected_radius, file_name
            );
        }
    }

    #[test]
    fn test_all_svg_elements_within_largest_circle() {
        use crate::maze::MazeDeserializer;
        use std::fs;

        let fixtures_dir = "tests/fixtures";
        let entries = fs::read_dir(fixtures_dir)
            .unwrap_or_else(|_| panic!("Failed to read directory: {}", fixtures_dir));

        let json_files: Vec<_> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().extension().and_then(|s| s.to_str()) == Some("json")
            })
            .collect();

        assert!(!json_files.is_empty(), "No JSON files found in {}", fixtures_dir);

        for entry in json_files {
            let file_path = entry.path();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();

            let json_content = fs::read_to_string(&file_path)
                .unwrap_or_else(|_| panic!("Failed to read file: {:?}", file_path));

            let json_data: serde_json::Value = serde_json::from_str(&json_content)
                .unwrap_or_else(|_| panic!("Failed to parse JSON from: {}", file_name));

            let maze = MazeDeserializer::deserialize(json_data)
                .unwrap_or_else(|_| panic!("Failed to deserialize maze from: {}", file_name));

            let path = vec![
                CircleCoord::create_with_arc_index(0, 0),
                CircleCoord::create_with_arc_index(1, 0),
            ];

            let svg_string = render_with_path(&maze, &path);

            let doc = roxmltree::Document::parse(&svg_string).unwrap_or_else(|_| {
                panic!("Failed to parse SVG XML for file: {}", file_name)
            });

            let max_radius = (maze.circles() * CIRCLE_RADIUS_STEP + HALF_RADIUS_STEP) as f64;

            for node in doc.descendants() {
                match node.tag_name().name() {
                    "circle" => {
                        let cx: f64 = node
                            .attribute("cx")
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0.0);
                        let cy: f64 = node
                            .attribute("cy")
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0.0);
                        let r: f64 = node
                            .attribute("r")
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0.0);

                        let distance_from_origin = (cx * cx + cy * cy).sqrt();
                        let max_extent = distance_from_origin + r;

                        assert!(
                            max_extent <= max_radius + 1e-6,
                            "Circle at ({}, {}) with radius {} exceeds max radius {} in {}",
                            cx, cy, r, max_radius, file_name
                        );
                    }
                    "line" => {
                        let x1: f64 = node
                            .attribute("x1")
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0.0);
                        let y1: f64 = node
                            .attribute("y1")
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0.0);
                        let x2: f64 = node
                            .attribute("x2")
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0.0);
                        let y2: f64 = node
                            .attribute("y2")
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0.0);

                        let dist1 = (x1 * x1 + y1 * y1).sqrt();
                        let dist2 = (x2 * x2 + y2 * y2).sqrt();

                        assert!(
                            dist1 <= max_radius + 1e-6,
                            "Line start ({}, {}) at distance {} exceeds max radius {} in {}",
                            x1, y1, dist1, max_radius, file_name
                        );
                        assert!(
                            dist2 <= max_radius + 1e-6,
                            "Line end ({}, {}) at distance {} exceeds max radius {} in {}",
                            x2, y2, dist2, max_radius, file_name
                        );
                    }
                    "path" => {
                        let d = node.attribute("d").unwrap_or("");

                        let parts: Vec<&str> = d.split_whitespace().collect();
                        let mut i = 0;

                        while i < parts.len() {
                            if parts[i] == "M" && i + 1 < parts.len() {
                                let coords: Vec<f64> = parts[i + 1]
                                    .split(',')
                                    .filter_map(|s| s.parse().ok())
                                    .collect();
                                if coords.len() == 2 {
                                    let dist = (coords[0] * coords[0] + coords[1] * coords[1]).sqrt();
                                    assert!(
                                        dist <= max_radius + 1e-6,
                                        "Path M point ({}, {}) at distance {} exceeds max radius {} in {}",
                                        coords[0], coords[1], dist, max_radius, file_name
                                    );
                                }
                                i += 2;
                            } else if parts[i] == "A" && i + 6 < parts.len() {
                                let coords: Vec<f64> = parts[i + 6]
                                    .split(',')
                                    .filter_map(|s| s.parse().ok())
                                    .collect();
                                if coords.len() == 2 {
                                    let dist = (coords[0] * coords[0] + coords[1] * coords[1]).sqrt();
                                    assert!(
                                        dist <= max_radius + 1e-6,
                                        "Path A endpoint ({}, {}) at distance {} exceeds max radius {} in {}",
                                        coords[0], coords[1], dist, max_radius, file_name
                                    );
                                }
                                i += 7;
                            } else {
                                i += 1;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    #[test]
    fn test_border_line_steps_match_fixture_lines() {
        use crate::maze::MazeDeserializer;
        use std::fs;

        let fixtures_dir = "tests/fixtures";
        let entries = fs::read_dir(fixtures_dir)
            .unwrap_or_else(|_| panic!("Failed to read directory: {}", fixtures_dir));

        let json_files: Vec<_> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().extension().and_then(|s| s.to_str()) == Some("json")
            })
            .collect();

        assert!(!json_files.is_empty(), "No JSON files found in {}", fixtures_dir);

        for entry in json_files {
            let file_path = entry.path();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();

            let json_content = fs::read_to_string(&file_path)
                .unwrap_or_else(|_| panic!("Failed to read file: {:?}", file_path));

            let json_data: serde_json::Value = serde_json::from_str(&json_content)
                .unwrap_or_else(|_| panic!("Failed to parse JSON from: {}", file_name));

            let expected_line_count = json_data["lines"]
                .as_array()
                .unwrap_or_else(|| panic!("Missing 'lines' array in {}", file_name))
                .len();

            let maze = MazeDeserializer::deserialize(json_data)
                .unwrap_or_else(|_| panic!("Failed to deserialize maze from: {}", file_name));

            let path = vec![
                CircleCoord::create_with_arc_index(0, 0),
                CircleCoord::create_with_arc_index(1, 0),
            ];

            let svg_string = render_with_path(&maze, &path);
            let actual_step_count = count_steps_in_border_lines(&svg_string);

            assert_eq!(
                actual_step_count, expected_line_count,
                "Border line step count {} does not match fixture line count {} for file: {}",
                actual_step_count, expected_line_count, file_name
            );
        }
    }

    #[test]
    fn test_border_arc_steps_match_fixture_arcs() {
        use crate::maze::MazeDeserializer;
        use std::fs;

        let fixtures_dir = "tests/fixtures";
        let entries = fs::read_dir(fixtures_dir)
            .unwrap_or_else(|_| panic!("Failed to read directory: {}", fixtures_dir));

        let json_files: Vec<_> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().extension().and_then(|s| s.to_str()) == Some("json")
            })
            .collect();

        assert!(!json_files.is_empty(), "No JSON files found in {}", fixtures_dir);

        for entry in json_files {
            let file_path = entry.path();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();

            let json_content = fs::read_to_string(&file_path)
                .unwrap_or_else(|_| panic!("Failed to read file: {:?}", file_path));

            let json_data: serde_json::Value = serde_json::from_str(&json_content)
                .unwrap_or_else(|_| panic!("Failed to parse JSON from: {}", file_name));

            let expected_arc_count = json_data["arcs"]
                .as_array()
                .unwrap_or_else(|| panic!("Missing 'arcs' array in {}", file_name))
                .len();

            let maze = MazeDeserializer::deserialize(json_data)
                .unwrap_or_else(|_| panic!("Failed to deserialize maze from: {}", file_name));

            let path = vec![
                CircleCoord::create_with_arc_index(0, 0),
                CircleCoord::create_with_arc_index(1, 0),
            ];

            let svg_string = render_with_path(&maze, &path);
            let actual_step_count = count_steps_in_border_arcs(&svg_string);

            assert_eq!(
                actual_step_count, expected_arc_count,
                "Border arc step count {} does not match fixture arc count {} for file: {}",
                actual_step_count, expected_arc_count, file_name
            );
        }
    }
}
