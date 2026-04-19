use motion_canvas_rs::prelude::*;
use std::time::Duration;

/// A simple helper node that synchronizes two signals every frame with a mapping function.
/// This allows "derived" properties that react to a source signal.
struct SignalLink<T, S> {
    source: Signal<S>,
    target: Signal<T>,
    mapper: Box<dyn Fn(S) -> T + Send + Sync>,
}

impl<T: Tweenable + PartialEq, S: Tweenable + PartialEq> SignalLink<T, S> {
    fn new<F>(source: Signal<S>, target: Signal<T>, mapper: F) -> Self
    where
        F: Fn(S) -> T + Send + Sync + 'static,
    {
        // Perform an immediate sync so the target has the correct value on the first frame
        target.set(mapper(source.get()));

        Self {
            source,
            target,
            mapper: Box::new(mapper),
        }
    }
}

impl<T: Tweenable + PartialEq, S: Tweenable + PartialEq> Node for SignalLink<T, S> {
    fn render(&self, _scene: &mut Scene, _parent_transform: Affine, _parent_opacity: f32) {
        // Redraw check: ensure sync even if update hasn't run yet
        let current = self.source.get();
        let desired = (self.mapper)(current);
        if self.target.get() != desired {
            self.target.set(desired);
        }
    }

    fn update(&mut self, _dt: Duration) {
        let current_source = self.source.get();
        let desired_target = (self.mapper)(current_source);
        if self.target.get() != desired_target {
            self.target.set(desired_target);
        }
    }

    fn state_hash(&self) -> u64 {
        // Hashing the source signal ensures the scene knows it needs a redraw
        // when the source value changes, even if no other node has updated yet.
        self.source.state_hash()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        // Cloned link will still point to same signals but closure needs to be handled
        // For simplicity in this example, we don't clone the links.
        panic!("LinkNode cloning not implemented for this example");
    }
}

fn main() {
    let mut project = Project::default()
        .with_title("Signals")
        .with_dimensions(800, 600)
        .close_on_finish();

    // 1. Create independent signals for our "Coordinate System"
    // These are NOT tied to any node yet.
    let x_var = Signal::new(0.0f32);
    let y_var = Signal::new(0.0f32);

    // 2. Create the Axis lines (Fixed)
    let x_axis = Line::default()
        .with_start(Vec2::new(-300.0, 0.0))
        .with_end(Vec2::new(300.0, 0.0))
        .with_stroke(Color::rgba8(255, 255, 255, 100), 2.0);

    let y_axis = Line::default()
        .with_start(Vec2::new(0.0, -200.0))
        .with_end(Vec2::new(0.0, 200.0))
        .with_stroke(Color::rgba8(255, 255, 255, 100), 2.0);

    // Group the axes at the center
    let axes = GroupNode::default()
        .with_nodes(vec![Box::new(x_axis), Box::new(y_axis)])
        .with_position(Vec2::new(400.0, 300.0));

    // 3. Create a representation of the point (P)
    let point = Circle::default()
        .with_radius(8.0)
        .with_fill(Color::rgb8(0xe1, 0x32, 0x38)); // Red

    // 4. Create text labels that show the values
    let x_label = TextNode::default()
        .with_text("X: 0")
        .with_font_size(24.0)
        .with_fill(Color::WHITE);

    let y_label = TextNode::default()
        .with_text("Y: 0")
        .with_font_size(24.0)
        .with_fill(Color::WHITE);

    // 5. Use SignalLink to derive node properties from our variables
    // This is the "magic" that makes node positions react to x_var and y_var.

    // Circle position follows both
    let y_clone = y_var.clone();
    let circle_pos_link = SignalLink::new(x_var.clone(), point.position.clone(), move |x| {
        Vec2::new(x + 400.0, y_clone.get() + 300.0)
    });

    // X Label follows X but stays at bottom
    let x_label_pos_link = SignalLink::new(x_var.clone(), x_label.position.clone(), |x| {
        Vec2::new(x + 400.0, 550.0)
    });

    // Y Label stays at left but follows Y
    let y_label_pos_link = SignalLink::new(y_var.clone(), y_label.position.clone(), |y| {
        Vec2::new(50.0, y + 300.0)
    });

    // Dynamic text update for labels
    let x_text_link = SignalLink::new(x_var.clone(), x_label.text.clone(), |x| {
        format!("X: {:.1}", x)
    });
    let y_text_link = SignalLink::new(y_var.clone(), y_label.text.clone(), |y| {
        format!("Y: {:.1}", y)
    });

    // Add everything to the scene
    project.scene.add(Box::new(axes));
    project.scene.add(Box::new(point));
    project.scene.add(Box::new(x_label));
    project.scene.add(Box::new(y_label));

    // Add the links as "invisible" nodes that just perform the sync
    project.scene.add(Box::new(circle_pos_link));
    project.scene.add(Box::new(x_label_pos_link));
    project.scene.add(Box::new(y_label_pos_link));
    project.scene.add(Box::new(x_text_link));
    project.scene.add(Box::new(y_text_link));

    // 6. Animate our independent variables!
    project.scene.video_timeline.add(chain![
        // Move X
        x_var
            .to(200.0, Duration::from_secs(1))
            .ease(easings::cubic_out),
        // Wait
        wait(Duration::from_millis(500)),
        // Move Y
        y_var
            .to(-150.0, Duration::from_secs(1))
            .ease(easings::back_out),
        // Move both together
        all![
            x_var
                .to(-250.0, Duration::from_secs(2))
                .ease(easings::expo_in_out),
            y_var
                .to(100.0, Duration::from_secs(2))
                .ease(easings::sine_in_out),
        ],
        // Reset
        all![
            x_var.to(0.0, Duration::from_secs(1)),
            y_var.to(0.0, Duration::from_secs(1)),
        ],
        // Final wait to see the result
        wait(Duration::from_secs(1)),
    ]);

    project.show().expect("Failed to render");
}
