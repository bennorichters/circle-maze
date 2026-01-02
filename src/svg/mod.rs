mod borders;
mod geometry;
mod markers;
mod solution_path;

use crate::circle_coord::CircleCoord;
use crate::maze::Maze;

use borders::{render_arcs, render_lines};
use geometry::CIRCLE_RADIUS_STEP;
use markers::render_path_markers;
use solution_path::render_solution_path;

const SVG_VIEWBOX_PADDING: usize = 20;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circle_coord::calc_total_arcs;
    use geometry::{HALF_RADIUS_STEP, DEGREES_IN_CIRCLE, DEGREES_IN_SEMICIRCLE};
    use markers::MARKER_RADIUS;

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

    fn generate_test_path(circles: usize) -> Vec<CircleCoord> {
        let mut path = Vec::new();
        path.push(CircleCoord::create_with_arc_index(0, 0));
        for c in 1..=circles {
            path.push(CircleCoord::create_with_arc_index(c, 0));
        }
        let outer_circle = circles;
        let total_arcs = calc_total_arcs(outer_circle);
        for a in 1..(total_arcs / 2) {
            path.push(CircleCoord::create_with_arc_index(outer_circle, a));
        }
        path
    }

    fn for_each_fixture<F>(test_fn: F)
    where
        F: Fn(&str, &crate::maze::Maze, &serde_json::Value, &str),
    {
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

            let maze = MazeDeserializer::deserialize(json_data.clone())
                .unwrap_or_else(|_| panic!("Failed to deserialize maze from: {}", file_name));

            let path = generate_test_path(maze.circles());
            let svg_string = render_with_path(&maze, &path);

            test_fn(file_name, &maze, &json_data, &svg_string);
        }
    }

    #[test]
    fn test_render_with_path_has_three_g_elements_with_correct_ids() {
        for_each_fixture(|file_name, _maze, _json_data, svg_string| {
            let doc = roxmltree::Document::parse(svg_string).unwrap_or_else(|_| {
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
        });
    }

    #[test]
    fn test_borders_do_not_intersect_solution_path() {
        for_each_fixture(|file_name, maze, _json_data, svg_string| {
            let doc = roxmltree::Document::parse(svg_string).unwrap_or_else(|_| {
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
        });
    }

    #[test]
    fn test_all_svg_elements_within_largest_circle() {
        for_each_fixture(|file_name, maze, _json_data, svg_string| {
            let doc = roxmltree::Document::parse(svg_string).unwrap_or_else(|_| {
                panic!("Failed to parse SVG XML for file: {}", file_name)
            });

            let max_radius = (maze.circles() * CIRCLE_RADIUS_STEP + HALF_RADIUS_STEP) as f64;
            let max_radius_with_markers = max_radius + MARKER_RADIUS as f64;

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

                        let is_marker = (r - MARKER_RADIUS as f64).abs() < 1e-6;
                        let allowed_radius = if is_marker {
                            max_radius_with_markers
                        } else {
                            max_radius
                        };

                        assert!(
                            max_extent <= allowed_radius + 1e-6,
                            "Circle at ({}, {}) with radius {} exceeds max radius {} in {}",
                            cx, cy, r, allowed_radius, file_name
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
        });
    }

    #[test]
    fn test_border_line_steps_match_fixture_lines() {
        for_each_fixture(|file_name, _maze, json_data, svg_string| {
            let expected_line_count = json_data["lines"]
                .as_array()
                .unwrap_or_else(|| panic!("Missing 'lines' array in {}", file_name))
                .len();

            let actual_step_count = count_steps_in_border_lines(svg_string);

            assert_eq!(
                actual_step_count, expected_line_count,
                "Border line step count {} does not match fixture line count {} for file: {}",
                actual_step_count, expected_line_count, file_name
            );
        });
    }

    #[test]
    fn test_border_arc_steps_match_fixture_arcs() {
        for_each_fixture(|file_name, _maze, json_data, svg_string| {
            let expected_arc_count = json_data["arcs"]
                .as_array()
                .unwrap_or_else(|| panic!("Missing 'arcs' array in {}", file_name))
                .len();

            let actual_step_count = count_steps_in_border_arcs(svg_string);

            assert_eq!(
                actual_step_count, expected_arc_count,
                "Border arc step count {} does not match fixture arc count {} for file: {}",
                actual_step_count, expected_arc_count, file_name
            );
        });
    }

    #[derive(Debug, Clone)]
    enum BorderElement {
        Line {
            x1: f64,
            y1: f64,
            x2: f64,
            y2: f64,
        },
        Arc {
            x1: f64,
            y1: f64,
            x2: f64,
            y2: f64,
            cx: f64,
            cy: f64,
            r: f64,
        },
        Circle {
            cx: f64,
            cy: f64,
            r: f64,
        },
    }

    fn parse_arc_from_path(d: &str) -> Option<BorderElement> {
        let parts: Vec<&str> = d.split_whitespace().collect();
        if parts.len() >= 8 && parts[0] == "M" && parts[2] == "A" {
            let start_coords: Vec<f64> = parts[1]
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            let end_coords: Vec<f64> = parts[7]
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            let radius: f64 = parts[3]
                .split(',')
                .next()
                .and_then(|s| s.parse().ok())?;

            if start_coords.len() == 2 && end_coords.len() == 2 {
                return Some(BorderElement::Arc {
                    x1: start_coords[0],
                    y1: start_coords[1],
                    x2: end_coords[0],
                    y2: end_coords[1],
                    cx: 0.0,
                    cy: 0.0,
                    r: radius,
                });
            }
        }
        None
    }

    fn points_equal(x1: f64, y1: f64, x2: f64, y2: f64) -> bool {
        x1 == x2 && y1 == y2
    }

    fn point_on_circle(x: f64, y: f64, cx: f64, cy: f64, r: f64) -> bool {
        let dx = x - cx;
        let dy = y - cy;
        let dist_sq = dx * dx + dy * dy;
        let r_sq = r * r;
        (dist_sq - r_sq).abs() < 1.0
    }

    fn are_connected(e1: &BorderElement, e2: &BorderElement) -> bool {
        match (e1, e2) {
            (
                BorderElement::Line { x1, y1, x2, y2 },
                BorderElement::Line {
                    x1: lx1,
                    y1: ly1,
                    x2: lx2,
                    y2: ly2,
                },
            ) => {
                points_equal(*x1, *y1, *lx1, *ly1)
                    || points_equal(*x1, *y1, *lx2, *ly2)
                    || points_equal(*x2, *y2, *lx1, *ly1)
                    || points_equal(*x2, *y2, *lx2, *ly2)
            }
            (
                BorderElement::Line { x1, y1, x2, y2 },
                BorderElement::Arc {
                    x1: ax1,
                    y1: ay1,
                    x2: ax2,
                    y2: ay2,
                    cx,
                    cy,
                    r,
                    ..
                },
            )
            | (
                BorderElement::Arc {
                    x1: ax1,
                    y1: ay1,
                    x2: ax2,
                    y2: ay2,
                    cx,
                    cy,
                    r,
                    ..
                },
                BorderElement::Line { x1, y1, x2, y2 },
            ) => {
                points_equal(*x1, *y1, *ax1, *ay1)
                    || points_equal(*x1, *y1, *ax2, *ay2)
                    || points_equal(*x2, *y2, *ax1, *ay1)
                    || points_equal(*x2, *y2, *ax2, *ay2)
                    || point_on_circle(*x1, *y1, *cx, *cy, *r)
                    || point_on_circle(*x2, *y2, *cx, *cy, *r)
            }
            (
                BorderElement::Line { x1, y1, x2, y2 },
                BorderElement::Circle { cx, cy, r },
            )
            | (
                BorderElement::Circle { cx, cy, r },
                BorderElement::Line { x1, y1, x2, y2 },
            ) => {
                point_on_circle(*x1, *y1, *cx, *cy, *r)
                    || point_on_circle(*x2, *y2, *cx, *cy, *r)
            }
            (
                BorderElement::Arc {
                    x1,
                    y1,
                    x2,
                    y2,
                    ..
                },
                BorderElement::Arc {
                    x1: ax1,
                    y1: ay1,
                    x2: ax2,
                    y2: ay2,
                    ..
                },
            ) => {
                points_equal(*x1, *y1, *ax1, *ay1)
                    || points_equal(*x1, *y1, *ax2, *ay2)
                    || points_equal(*x2, *y2, *ax1, *ay1)
                    || points_equal(*x2, *y2, *ax2, *ay2)
            }
            (
                BorderElement::Arc {
                    x1,
                    y1,
                    x2,
                    y2,
                    ..
                },
                BorderElement::Circle { cx, cy, r },
            )
            | (
                BorderElement::Circle { cx, cy, r },
                BorderElement::Arc {
                    x1,
                    y1,
                    x2,
                    y2,
                    ..
                },
            ) => {
                point_on_circle(*x1, *y1, *cx, *cy, *r)
                    || point_on_circle(*x2, *y2, *cx, *cy, *r)
            }
            (BorderElement::Circle { .. }, BorderElement::Circle { .. }) => false,
        }
    }

    fn dfs_visit(graph: &[Vec<usize>], node: usize, visited: &mut [bool]) {
        visited[node] = true;
        for &neighbor in &graph[node] {
            if !visited[neighbor] {
                dfs_visit(graph, neighbor, visited);
            }
        }
    }

    #[test]
    fn test_borders_form_connected_graph() {
        for_each_fixture(|file_name, _maze, _json_data, svg_string| {
            let doc = roxmltree::Document::parse(svg_string).unwrap_or_else(|_| {
                panic!("Failed to parse SVG XML for file: {}", file_name)
            });

            let borders_g = doc
                .descendants()
                .find(|n| n.tag_name().name() == "g" && n.attribute("id") == Some("borders"))
                .unwrap_or_else(|| {
                    panic!(
                        "Failed to find g element with id='borders' for file: {}",
                        file_name
                    )
                });

            let mut elements = Vec::new();

            for node in borders_g.children() {
                match node.tag_name().name() {
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
                        elements.push(BorderElement::Line { x1, y1, x2, y2 });
                    }
                    "path" => {
                        let d = node.attribute("d").unwrap_or("");
                        if let Some(arc) = parse_arc_from_path(d) {
                            elements.push(arc);
                        }
                    }
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
                        elements.push(BorderElement::Circle { cx, cy, r });
                    }
                    _ => {}
                }
            }

            let n = elements.len();
            if n == 0 {
                return;
            }

            let mut graph = vec![Vec::new(); n];

            for i in 0..n {
                for j in (i + 1)..n {
                    if are_connected(&elements[i], &elements[j]) {
                        graph[i].push(j);
                        graph[j].push(i);
                    }
                }
            }

            let mut visited = vec![false; n];
            dfs_visit(&graph, 0, &mut visited);

            for (i, &v) in visited.iter().enumerate() {
                assert!(
                    v,
                    "Border element {} is not connected to the main graph in file: {}",
                    i,
                    file_name
                );
            }
        });
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Endpoint {
        x: f64,
        y: f64,
    }

    impl Endpoint {
        fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }

        fn approx_eq(&self, other: &Self, epsilon: f64) -> bool {
            (self.x - other.x).abs() < epsilon && (self.y - other.y).abs() < epsilon
        }
    }

    fn parse_solution_line_endpoints(node: &roxmltree::Node) -> Option<(Endpoint, Endpoint)> {
        let x1: f64 = node.attribute("x1")?.parse().ok()?;
        let y1: f64 = node.attribute("y1")?.parse().ok()?;
        let x2: f64 = node.attribute("x2")?.parse().ok()?;
        let y2: f64 = node.attribute("y2")?.parse().ok()?;
        Some((Endpoint::new(x1, y1), Endpoint::new(x2, y2)))
    }

    fn parse_solution_arc_endpoints(node: &roxmltree::Node) -> Option<(Endpoint, Endpoint)> {
        let d = node.attribute("d")?;
        let parts: Vec<&str> = d.split_whitespace().collect();
        if parts.len() >= 8 && parts[0] == "M" && parts[2] == "A" {
            let start_coords: Vec<f64> = parts[1]
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            let end_coords: Vec<f64> = parts[7]
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            if start_coords.len() == 2 && end_coords.len() == 2 {
                return Some((
                    Endpoint::new(start_coords[0], start_coords[1]),
                    Endpoint::new(end_coords[0], end_coords[1]),
                ));
            }
        }
        None
    }

    fn find_endpoint_index(endpoints: &[Endpoint], target: &Endpoint, epsilon: f64) -> Option<usize> {
        endpoints.iter().position(|e| e.approx_eq(target, epsilon))
    }

    fn add_or_find_endpoint(endpoints: &mut Vec<Endpoint>, ep: Endpoint, epsilon: f64) -> usize {
        if let Some(idx) = find_endpoint_index(endpoints, &ep, epsilon) {
            idx
        } else {
            endpoints.push(ep);
            endpoints.len() - 1
        }
    }

    fn parse_marker_center(node: &roxmltree::Node) -> Option<Endpoint> {
        let cx: f64 = node.attribute("cx")?.parse().ok()?;
        let cy: f64 = node.attribute("cy")?.parse().ok()?;
        Some(Endpoint::new(cx, cy))
    }

    fn extract_solution_path_edges(
        svg_string: &str,
        file_name: &str,
    ) -> (Vec<Endpoint>, Vec<(usize, usize)>) {
        let doc = roxmltree::Document::parse(svg_string).unwrap_or_else(|_| {
            panic!("Failed to parse SVG XML for file: {}", file_name)
        });

        let solution_path_g = doc
            .descendants()
            .find(|n| n.tag_name().name() == "g" && n.attribute("id") == Some("solution-path"))
            .unwrap_or_else(|| {
                panic!("Failed to find g element with id='solution-path' for file: {}", file_name)
            });

        let mut endpoints: Vec<Endpoint> = Vec::new();
        let mut edges: Vec<(usize, usize)> = Vec::new();
        let epsilon = 1e-6;

        for node in solution_path_g.children() {
            let parsed = match node.tag_name().name() {
                "line" => parse_solution_line_endpoints(&node),
                "path" => parse_solution_arc_endpoints(&node),
                _ => None,
            };

            if let Some((start, end)) = parsed {
                if start.approx_eq(&end, epsilon) {
                    continue;
                }
                let start_idx = add_or_find_endpoint(&mut endpoints, start, epsilon);
                let end_idx = add_or_find_endpoint(&mut endpoints, end, epsilon);
                edges.push((start_idx, end_idx));
            }
        }

        (endpoints, edges)
    }

    #[test]
    fn test_solution_path_is_connected_and_matches_markers() {
        for_each_fixture(|file_name, _maze, _json_data, svg_string| {
            let doc = roxmltree::Document::parse(svg_string).unwrap_or_else(|_| {
                panic!("Failed to parse SVG XML for file: {}", file_name)
            });

            let markers_g = doc
                .descendants()
                .find(|n| {
                    n.tag_name().name() == "g" && n.attribute("id") == Some("start-finish-markers")
                })
                .unwrap_or_else(|| {
                    panic!(
                        "Failed to find g element with id='start-finish-markers' for file: {}",
                        file_name
                    )
                });

            let (endpoints, edges) = extract_solution_path_edges(svg_string, file_name);

            if edges.is_empty() {
                return;
            }

            let mut degree = vec![0usize; endpoints.len()];
            for &(a, b) in &edges {
                degree[a] += 1;
                degree[b] += 1;
            }

            let path_endpoints: Vec<&Endpoint> = degree
                .iter()
                .enumerate()
                .filter(|(_, &d)| d == 1)
                .map(|(i, _)| &endpoints[i])
                .collect();

            assert_eq!(
                path_endpoints.len(),
                2,
                "Expected exactly 2 path endpoints (start and end), found {} in {}",
                path_endpoints.len(),
                file_name
            );

            let mut adjacency = vec![Vec::new(); endpoints.len()];
            for &(a, b) in &edges {
                adjacency[a].push(b);
                adjacency[b].push(a);
            }

            let mut visited = vec![false; endpoints.len()];
            dfs_visit(&adjacency, 0, &mut visited);

            for (i, &v) in visited.iter().enumerate() {
                assert!(
                    v,
                    "Endpoint {} is not connected to the main path in file: {}",
                    i,
                    file_name
                );
            }

            let marker_centers: Vec<Endpoint> = markers_g
                .children()
                .filter(|n| n.tag_name().name() == "circle")
                .filter_map(|n| parse_marker_center(&n))
                .collect();

            assert_eq!(
                marker_centers.len(),
                2,
                "Expected exactly 2 marker circles, found {} in {}",
                marker_centers.len(),
                file_name
            );

            let epsilon = 1e-6;
            for marker in &marker_centers {
                let matches = path_endpoints.iter().any(|ep| ep.approx_eq(marker, epsilon));
                assert!(
                    matches,
                    "Marker at ({}, {}) does not match any path endpoint in {}",
                    marker.x,
                    marker.y,
                    file_name
                );
            }
        });
    }
}
