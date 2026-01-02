use crate::circle_coord::CircleCoord;

use super::geometry::{calc_display_angle, calc_display_radius, polar_to_cartesian};

pub const MARKER_RADIUS: usize = 3;

pub fn render_path_markers(path: &[CircleCoord]) -> String {
    let mut content = String::new();
    content.push_str(r#"<g id="start-finish-markers" fill="red">
"#);

    for coord in [path.first(), path.last()].into_iter().flatten() {
        let radius = calc_display_radius(coord.circle());
        let angle = calc_display_angle(coord);
        let point = polar_to_cartesian(radius, &angle);

        content.push_str(&format!(
            r#"  <circle cx="{:.8}" cy="{:.8}" r="{}"/>
"#,
            point.x, point.y, MARKER_RADIUS
        ));
    }

    content.push_str("</g>\n");
    content
}
