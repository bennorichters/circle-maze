use crate::circle_coord::CircleCoord;

use super::geometry::{
    calc_arc_angle_diff, calc_display_angle, calc_display_radius, calc_large_arc_flag, clockwise,
    create_svg_arc_path, fraction_to_degrees, polar_to_cartesian, HALF_RADIUS_STEP,
};

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
        angle: fraction::Fraction,
    },
}

impl PathSegment {
    fn render(&self) -> String {
        match self {
            PathSegment::Arc { radius, start_angle, end_angle, clockwise } => {
                render_solution_arc(*radius, start_angle, end_angle, *clockwise)
            }
            PathSegment::Line { start_radius, end_radius, angle } => {
                render_solution_line(*start_radius, *end_radius, angle)
            }
        }
    }

    fn end_angle(&self) -> fraction::Fraction {
        match self {
            PathSegment::Arc { end_angle, .. } => *end_angle,
            PathSegment::Line { angle, .. } => *angle,
        }
    }
}

pub fn render_solution_path(path: &[CircleCoord]) -> String {
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
    angle: &fraction::Fraction,
) -> String {
    let start = polar_to_cartesian(start_radius, angle);
    let end = polar_to_cartesian(end_radius, angle);

    format!(
        r#"  <line x1="{:.8}" y1="{:.8}" x2="{:.8}" y2="{:.8}"/>
"#,
        start.x, start.y, end.x, end.y
    )
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

    let start_angle = segments.last().map(|s| s.end_angle())
        .unwrap_or_else(|| calc_display_angle(start));
    let end_angle = calc_display_angle(&path[end_idx]);

    segments.push(PathSegment::Arc {
        radius,
        start_angle,
        end_angle,
        clockwise: is_clockwise,
    });
}

fn adjust_line_angle(
    start_radius: usize,
    end_radius: usize,
    start_angle: fraction::Fraction,
    end_angle: fraction::Fraction,
) -> fraction::Fraction {
    if start_radius < end_radius { end_angle } else { start_angle }
}

fn add_connecting_arc_if_needed(
    segments: &mut Vec<PathSegment>,
    start: &CircleCoord,
    start_radius: usize,
    line_angle: fraction::Fraction,
) -> usize {
    if let Some(PathSegment::Arc { end_angle, .. }) = segments.last_mut() {
        *end_angle = line_angle;
        return start_radius;
    }

    if let Some(PathSegment::Line { angle, end_radius, .. }) = segments.last() {
        let prev_angle = *angle;
        let prev_end_radius = *end_radius;

        if start_radius > 0 {
            let is_cw = clockwise(fraction_to_degrees(&prev_angle), fraction_to_degrees(&line_angle));
            segments.push(PathSegment::Arc {
                radius: start_radius,
                start_angle: prev_angle,
                end_angle: line_angle,
                clockwise: is_cw,
            });
            return start_radius;
        } else if prev_end_radius == 0 {
            if let Some(PathSegment::Line { end_radius, .. }) = segments.last_mut() {
                *end_radius = HALF_RADIUS_STEP;
            }
            let is_cw = clockwise(fraction_to_degrees(&prev_angle), fraction_to_degrees(&line_angle));
            segments.push(PathSegment::Arc {
                radius: HALF_RADIUS_STEP,
                start_angle: prev_angle,
                end_angle: line_angle,
                clockwise: is_cw,
            });
            return HALF_RADIUS_STEP;
        }
        return start_radius;
    }

    let ea = calc_display_angle(start);
    let is_cw = clockwise(fraction_to_degrees(&ea), fraction_to_degrees(&line_angle));
    segments.push(PathSegment::Arc {
        radius: start_radius,
        start_angle: ea,
        end_angle: line_angle,
        clockwise: is_cw,
    });
    start_radius
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

    let angle = adjust_line_angle(start_radius, end_radius, initial_start_angle, initial_end_angle);

    let adjusted_start_radius = add_connecting_arc_if_needed(segments, start, start_radius, angle);

    segments.push(PathSegment::Line {
        start_radius: adjusted_start_radius,
        end_radius,
        angle,
    });
}

fn add_final_arc_if_needed(segments: &mut Vec<PathSegment>, path: &[CircleCoord]) {
    if let Some(PathSegment::Line { angle, .. }) = segments.last() {
        let last_coord = &path[path.len() - 1];
        let finish_radius = calc_display_radius(last_coord.circle());
        let finish_angle = calc_display_angle(last_coord);
        let is_cw = clockwise(fraction_to_degrees(angle), fraction_to_degrees(&finish_angle));

        segments.push(PathSegment::Arc {
            radius: finish_radius,
            start_angle: *angle,
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
