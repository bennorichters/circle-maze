use crate::maze::Maze;
use std::fs::File;
use std::io::Write;

pub fn render(maze: &Maze) -> std::io::Result<()> {
    let circles = maze.circles();

    // Calculate the size needed to fit all circles
    let max_radius = circles * 10;
    let view_size = max_radius * 2 + 20; // Add some padding

    // Create SVG content
    let mut svg_content = String::new();
    svg_content.push_str(&format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="{} {} {} {}" width="100%" height="100%" preserveAspectRatio="xMidYMid meet">
"#,
        -(view_size as i32) / 2,
        -(view_size as i32) / 2,
        view_size,
        view_size
    ));

    // Add circles
    for i in 1..=circles {
        let radius = i * 10;
        svg_content.push_str(&format!(
            r#"  <circle cx="0" cy="0" r="{}" fill="none" stroke="black" stroke-width="1"/>
"#,
            radius
        ));
    }

    svg_content.push_str("</svg>\n");

    // Write to file
    let mut file = File::create("maze.svg")?;
    file.write_all(svg_content.as_bytes())?;

    Ok(())
}
