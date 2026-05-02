use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::default()
        .with_title("Grid Example")
        .with_dimensions(1280, 720)
        .close_on_finish();

    // 1. Create Grid Node
    let grid = GridNode::square(Vec2::new(640.0, 360.0), 4.0, 50.0)
        .with_stroke(Palette::DARK_GRAY, 2.0)
        .with_opacity(0.0);

    // 2. Create Text Node
    let text_node = TextNode::default()
        .with_position(Vec2::new(640.0, 100.0))
        .with_font_size(36.0)
        .with_fill(Color::WHITE)
        .with_text("Grid: 4x4 | Spacing: 50x50");

    // 3. Bind Text to Grid's rows/columns and spacing
    let cols_sig = grid.columns.clone();
    let spacing_sig = grid.spacing.clone();
    let text_link = text_node.text.bind(grid.rows.clone(), move |rows| {
        let cols = cols_sig.get();
        let spacing = spacing_sig.get();
        format!(
            "Grid: {:.0}x{:.0} | Spacing: {:.0}x{:.0}",
            cols, rows, spacing.x, spacing.y
        )
    });

    // Add to scene
    project.scene.add(Box::new(grid.clone()));
    project.scene.add(Box::new(text_node));
    project.scene.add(Box::new(text_link));

    project.scene.video_timeline.add(chain![
        grid.opacity
            .to(1.0, Duration::from_secs(1))
            .ease(easings::cubic_out),
        wait(Duration::from_millis(500)),
        all![
            grid.rows.to(16.0, Duration::from_secs(2)),
            grid.columns.to(16.0, Duration::from_secs(2)),
            grid.stroke_color.to(Palette::BLUE, Duration::from_secs(2)),
            grid.spacing
                .to(Vec2::new(100.0, 100.0), Duration::from_secs(2)),
        ],
        wait(Duration::from_millis(500)),
        all![
            grid.rows.to(8.0, Duration::from_secs(2)),
            grid.columns.to(8.0, Duration::from_secs(2)),
            grid.stroke_color
                .to(Palette::ORANGE, Duration::from_secs(2)),
            grid.spacing
                .to(Vec2::new(20.0, 20.0), Duration::from_secs(2)),
        ],
        wait(Duration::from_secs(1)),
        grid.opacity
            .to(0.0, Duration::from_secs(1))
            .ease(easings::cubic_out),
        wait(Duration::from_millis(500)),
    ]);

    project.show().expect("Failed to render");
}
