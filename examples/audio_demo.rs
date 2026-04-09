use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::default()
        .with_dimensions(800, 450)
        .with_title("Audio Demo")
        .with_background(Color::rgb8(20, 20, 25));

    // Setup Video Timeline
    let rect = Rect::new(Vec2::new(100.0, 100.0), Vec2::new(200.0, 200.0), Color::RED);
    project.scene.add(Box::new(rect.clone()));

    project.scene.video_timeline.add(chain!(
        all![
            rect.transform
                .to(Affine::translate((150.0, 150.0)), Duration::from_secs(1)),
            rect.size.to(Vec2::new(200.0, 50.0), Duration::from_secs(1)),
            rect.color.to(Color::BLUE, Duration::from_secs(1)),
        ],
        all![
            rect.transform
                .to(Affine::translate((350.0, 150.0)), Duration::from_secs(1)),
            rect.color.to(Color::RED, Duration::from_secs(1)),
        ]
    ));

    // Setup Audio Timeline using new macros and builder
    project.scene.audio_timeline.add(chain!(
        play!(AudioNode::new("./examples/audios/combo-1.mp3").with_volume(0.5)),
        audio_wait!(0.05),
        play!(AudioNode::new("./examples/audios/combo-2.mp3").with_volume(1.0))
    ));

    println!("Project configured with separate video and audio timelines.");
    println!(
        "Video duration: {:?}",
        project.scene.video_timeline.duration()
    );
    println!(
        "Audio duration: {:?}",
        project.scene.audio_timeline.duration()
    );

    project.show().expect("Failed to run audio demo");
}
