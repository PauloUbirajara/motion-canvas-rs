use motion_canvas_rs::prelude::*;
use std::time::Duration;

#[test]
fn test_any_node_conversions() {
    let circle = Circle::default()
        .with_position(Vec2::new(10.0, 20.0))
        .with_radius(42.0);

    // Test conversion from concrete node to AnyNode
    let any_node: AnyNode = circle.clone().into();

    // The hash state must match
    assert_eq!(any_node.state_hash(), circle.state_hash());
}

#[test]
fn test_any_node_reference_conversions() {
    let circle = Circle::default()
        .with_position(Vec2::new(10.0, 20.0))
        .with_radius(42.0);

    // Test conversion from &Circle reference to AnyNode
    let any_node_ref: AnyNode = (&circle).into();
    assert_eq!(any_node_ref.state_hash(), circle.state_hash());

    // Test conversion from &AnyNode reference to AnyNode
    let any_node_ref2: AnyNode = (&any_node_ref).into();
    assert_eq!(any_node_ref2.state_hash(), circle.state_hash());
}

#[test]
fn test_any_node_custom_wrapping() {
    #[derive(Clone, Default)]
    struct CustomWidget {
        val: f32,
    }

    impl Node for CustomWidget {
        fn state_hash(&self) -> u64 {
            self.val.to_bits() as u64
        }

        fn update(&mut self, dt: Duration) {
            self.val += dt.as_secs_f32();
        }

        fn clone_node(&self) -> Box<dyn Node> {
            Box::new(self.clone())
        }

        fn reset(&mut self) {}

        #[cfg(feature = "runtime")]
        fn render(
            &self,
            _scene: &mut vello::Scene,
            _parent_transform: kurbo::Affine,
            _parent_opacity: f32,
        ) {
        }
    }

    let widget = CustomWidget { val: 100.0 };
    let mut any_node: AnyNode = (Box::new(widget) as Box<dyn Node>).into();

    assert_eq!(any_node.state_hash(), 100.0f32.to_bits() as u64);

    // Test update method updates underlying custom widget
    any_node.update(Duration::from_secs(5));
    assert_eq!(any_node.state_hash(), 105.0f32.to_bits() as u64);
}

#[test]
fn test_any_animation_tween_unboxing() {
    let signal = Signal::new(10.0f32);

    // Convert raw tween to AnyAnimation
    let tween = signal.to(20.0, Duration::from_secs(2));
    let any_anim: AnyAnimation = tween.into();
    assert_eq!(any_anim.duration(), Duration::from_secs(2));

    // Convert boxed tween to AnyAnimation (should unbox successfully)
    let tween2 = signal.to(30.0, Duration::from_secs(3));
    let boxed_tween = Box::new(tween2);
    let any_anim_from_box: AnyAnimation = boxed_tween.into();
    assert_eq!(any_anim_from_box.duration(), Duration::from_secs(3));
}

#[test]
fn test_any_animation_wait_unboxing() {
    let w = wait(Duration::from_secs(4));
    let any_anim: AnyAnimation = w.into();
    assert_eq!(any_anim.duration(), Duration::from_secs(4));
}
