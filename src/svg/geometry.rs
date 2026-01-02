use crate::circle_coord::{calc_total_arcs, CircleCoord};
use std::f64::consts::PI;

pub const DEGREES_IN_CIRCLE: f64 = 360.0;
pub const DEGREES_IN_SEMICIRCLE: f64 = 180.0;
pub const CIRCLE_RADIUS_STEP: usize = 10;
pub const HALF_RADIUS_STEP: usize = 5;
const COORDINATE_EPSILON: f64 = 1e-10;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn fraction_to_degrees(angle: &fraction::Fraction) -> f64 {
    (*angle.numer().unwrap() as f64) / (*angle.denom().unwrap() as f64)
}

fn angle_to_radians(angle: &fraction::Fraction) -> f64 {
    let degrees = fraction_to_degrees(angle);
    degrees * PI / DEGREES_IN_SEMICIRCLE
}

pub fn polar_to_cartesian(radius: usize, angle: &fraction::Fraction) -> Point {
    let angle_rad = angle_to_radians(angle);
    let x = radius as f64 * angle_rad.cos();
    let y = radius as f64 * angle_rad.sin();
    Point {
        x: if x.abs() < COORDINATE_EPSILON { 0.0 } else { x },
        y: if y.abs() < COORDINATE_EPSILON { 0.0 } else { y },
    }
}

pub fn normalize_angle_diff(diff: f64) -> f64 {
    if diff < 0.0 {
        diff + DEGREES_IN_CIRCLE
    } else {
        diff
    }
}

pub fn calc_large_arc_flag(angle_diff: f64) -> u8 {
    u8::from(angle_diff > DEGREES_IN_SEMICIRCLE)
}

pub fn calc_arc_angle_diff(start_deg: f64, end_deg: f64, is_clockwise: bool) -> f64 {
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

pub fn clockwise(start_angle_deg: f64, next_angle_deg: f64) -> bool {
    let angle_diff = normalize_angle_diff(next_angle_deg - start_angle_deg);
    angle_diff > 0.0 && angle_diff <= DEGREES_IN_SEMICIRCLE
}

pub fn create_svg_arc_path(
    radius: usize,
    start_angle: &fraction::Fraction,
    end_angle: &fraction::Fraction,
    sweep_flag: u8,
    large_arc_flag: u8,
) -> String {
    let start = polar_to_cartesian(radius, start_angle);
    let end = polar_to_cartesian(radius, end_angle);

    format!(
        r#"  <path d="M {:.8},{:.8} A {},{} 0 {} {} {:.8},{:.8}"/>
"#,
        start.x, start.y, radius, radius, large_arc_flag, sweep_flag, end.x, end.y
    )
}

pub fn calc_display_radius(circle: usize) -> usize {
    if circle == 0 {
        0
    } else {
        circle * CIRCLE_RADIUS_STEP + HALF_RADIUS_STEP
    }
}

pub fn calc_display_angle(coord: &CircleCoord) -> fraction::Fraction {
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
