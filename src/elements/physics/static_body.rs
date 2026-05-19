use crate::core::animation::{Node, Signal};
use crate::elements::physics::traits::PhysicsBody;
use crate::elements::physics::{PhysicsMode, PhysicsShape, DEFAULT_FRICTION};
use glam::Vec2;
use kurbo::Affine;
use rapier2d::prelude::{ColliderSet, RigidBody, RigidBodyType, Vector};
use std::time::Duration;

#[cfg(feature = "runtime")]
use vello::Scene;

/// A node wrapping any visual element as an immovable fixed obstacle in the Rapier2d simulation.
///
/// Useful for floors, walls, ramps, and platforms. Although static bodies do not fall or slide
/// under forces, they collide with dynamic bodies.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use motion_canvas_rs::elements::shapes::Rect;
/// let wall = StaticBodyNode::new(Box::new(Rect::default()))
///     .with_position(Vec2::new(100.0, 300.0))
///     .with_shape(PhysicsShape::Cuboid(Vec2::new(10.0, 100.0)))
///     .with_friction(0.1);
/// ```
pub struct StaticBodyNode {
    /// The nested visual node that is rendered.
    pub inner: Box<dyn Node>,
    /// Immovable position of the static body.
    pub position: Signal<Vec2>,
    /// Immovable rotation of the static body in radians.
    pub rotation: Signal<f32>,
    /// Bounding shape of the collider.
    pub shape: PhysicsShape,
    /// Restitution / bounciness coefficient when dynamic bodies collide with this obstacle.
    pub bounciness: f32,
    /// Friction coefficient.
    pub friction: f32,
}

impl Clone for StaticBodyNode {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_node(),
            position: self.position.clone(),
            rotation: self.rotation.clone(),
            shape: self.shape.clone(),
            bounciness: self.bounciness,
            friction: self.friction,
        }
    }
}

impl StaticBodyNode {
    /// Creates a new static body wrapping a visual node.
    pub fn new(inner: Box<dyn Node>) -> Self {
        Self {
            inner,
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            shape: PhysicsShape::Cuboid(Vec2::new(25.0, 25.0)),
            bounciness: 0.0,
            friction: DEFAULT_FRICTION,
        }
    }

    /// Sets the position of the static body obstacle.
    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.position = Signal::new(pos);
        self
    }

    /// Sets the rotation of the static body obstacle in radians.
    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = Signal::new(rotation);
        self
    }

    /// Sets the bounding shape of the static collider.
    pub fn with_shape(mut self, shape: PhysicsShape) -> Self {
        self.shape = shape;
        self
    }

    /// Sets the bounciness (restitution coefficient) when dynamic bodies collide with this obstacle.
    pub fn with_bounciness(mut self, bounciness: f32) -> Self {
        self.bounciness = bounciness;
        self
    }

    /// Sets the friction coefficient of the static body obstacle.
    pub fn with_friction(mut self, friction: f32) -> Self {
        self.friction = friction;
        self
    }
}

impl Node for StaticBodyNode {
    #[cfg(feature = "runtime")]
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        self.inner.render(scene, parent_transform, parent_opacity);
    }

    fn update(&mut self, dt: Duration) {
        self.inner.update(dt);
    }

    fn state_hash(&self) -> u64 {
        self.inner.state_hash()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn reset(&mut self) {
        self.position.reset();
        self.rotation.reset();
        self.inner.reset();
    }
}

impl PhysicsBody for StaticBodyNode {
    fn mode(&self) -> PhysicsMode {
        PhysicsMode::Disabled
    }

    fn position_signal(&self) -> &Signal<Vec2> {
        &self.position
    }

    fn rotation_signal(&self) -> &Signal<f32> {
        &self.rotation
    }

    fn sync_to_rapier(&mut self, rb: &mut RigidBody, _collider_set: &mut ColliderSet) {
        if !rb.is_fixed() {
            rb.set_body_type(RigidBodyType::Fixed, true);
        }
        let visual_pos = self.position.get();
        let visual_rot = self.rotation.get();
        let current_pos = rb.translation();
        let current_rot = rb.rotation().angle();
        if (current_pos.x - visual_pos.x).abs() > 1e-5
            || (current_pos.y - visual_pos.y).abs() > 1e-5
            || (current_rot - visual_rot).abs() > 1e-5
        {
            rb.set_translation(Vector::new(visual_pos.x, visual_pos.y), true);
            rb.set_rotation(rapier2d::math::Rotation::new(visual_rot), true);
        }
    }
}
