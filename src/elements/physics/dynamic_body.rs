use crate::core::animation::{Node, Signal};
use crate::elements::physics::traits::PhysicsBody;
use crate::elements::physics::{PhysicsMode, PhysicsShape, DEFAULT_BOUNCINESS, DEFAULT_FRICTION};
use glam::Vec2;
use kurbo::Affine;
use rapier2d::prelude::{ColliderSet, RigidBody, RigidBodyType, Vector};
use std::time::Duration;

#[cfg(feature = "runtime")]
use vello::Scene;

/// A node wrapping any visual element as a dynamic/kinematic rigid body in the Rapier2d simulation.
///
/// Dynamic bodies fall under gravity, bounce off obstacles, and react to forces. Their reactive
/// properties like position, rotation, and physics mode can be controlled and tweened via Signals.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use motion_canvas_rs::elements::shapes::Rect;
/// let body = RigidBodyNode::new(Box::new(Rect::default()))
///     .with_position(Vec2::new(100.0, 50.0))
///     .with_shape(PhysicsShape::Cuboid(Vec2::new(50.0, 50.0)))
///     .with_bounciness(0.8)
///     .with_mode(PhysicsMode::Disabled); // Starts disabled for signal-driven tweens first
/// ```
pub struct RigidBodyNode {
    /// The nested visual node that is rendered.
    pub inner: Box<dyn Node>,
    /// The reactive position of the body. Can be animated/tweened directly in `Disabled` or `Kinematic` mode.
    pub position: Signal<Vec2>,
    /// The reactive rotation of the body in radians. Can be animated/tweened directly in `Disabled` or `Kinematic` mode.
    pub rotation: Signal<f32>,
    /// Controls whether the physics engine is tracking or driving this body.
    pub mode: Signal<PhysicsMode>,
    /// The geometric bounds for collider queries.
    pub shape: PhysicsShape,
    /// Bounciness / restitution coefficient. Defines how much energy is conserved upon impact (normally between 0.0 and 1.0).
    pub bounciness: f32,
    /// Friction coefficient. Controls sliding behavior against other surfaces (normally between 0.0 and 1.0).
    pub friction: f32,
    /// Initial linear velocity applied at the beginning of the simulation (pixels/second).
    pub initial_velocity: Vec2,
    /// Initial angular velocity applied at the beginning of the simulation (radians/second).
    pub initial_angular_velocity: f32,
}

impl Clone for RigidBodyNode {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_node(),
            position: self.position.clone(),
            rotation: self.rotation.clone(),
            mode: self.mode.clone(),
            shape: self.shape.clone(),
            bounciness: self.bounciness,
            friction: self.friction,
            initial_velocity: self.initial_velocity,
            initial_angular_velocity: self.initial_angular_velocity,
        }
    }
}

impl RigidBodyNode {
    /// Creates a new rigid body wrapping a visual node, initializing it with standard default properties.
    pub fn new(inner: Box<dyn Node>) -> Self {
        Self {
            inner,
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            mode: Signal::new(PhysicsMode::Dynamic),
            shape: PhysicsShape::Cuboid(Vec2::new(25.0, 25.0)),
            bounciness: DEFAULT_BOUNCINESS,
            friction: DEFAULT_FRICTION,
            initial_velocity: Vec2::ZERO,
            initial_angular_velocity: 0.0,
        }
    }

    /// Sets the initial reactive position signal of the rigid body.
    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.position = Signal::new(pos);
        self
    }

    /// Sets the initial reactive rotation signal of the rigid body in radians.
    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = Signal::new(rotation);
        self
    }

    /// Sets the bounding shape of the rigid body's physical collider.
    pub fn with_shape(mut self, shape: PhysicsShape) -> Self {
        self.shape = shape;
        self
    }

    /// Configures the bounciness (restitution coefficient) of the body.
    pub fn with_bounciness(mut self, bounciness: f32) -> Self {
        self.bounciness = bounciness;
        self
    }

    /// Configures the friction coefficient of the body.
    pub fn with_friction(mut self, friction: f32) -> Self {
        self.friction = friction;
        self
    }

    /// Sets the initial linear velocity (pixels/second) applied to the dynamic body.
    pub fn with_initial_velocity(mut self, vel: Vec2) -> Self {
        self.initial_velocity = vel;
        self
    }

    /// Sets the initial angular velocity (radians/second) applied to the dynamic body.
    pub fn with_initial_angular_velocity(mut self, ang_vel: f32) -> Self {
        self.initial_angular_velocity = ang_vel;
        self
    }

    /// Configures the initial operational mode of the physics body.
    pub fn with_mode(mut self, mode: PhysicsMode) -> Self {
        self.mode = Signal::new(mode);
        self
    }
}

impl Node for RigidBodyNode {
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
        self.mode.reset();
        self.inner.reset();
    }
}

impl PhysicsBody for RigidBodyNode {
    fn mode(&self) -> PhysicsMode {
        self.mode.get()
    }

    fn position_signal(&self) -> &Signal<Vec2> {
        &self.position
    }

    fn rotation_signal(&self) -> &Signal<f32> {
        &self.rotation
    }

    fn sync_to_rapier(&mut self, rb: &mut RigidBody, collider_set: &mut ColliderSet) {
        let visual_pos = self.position.get();
        let visual_rot = self.rotation.get();

        let current_mode = self.mode.get();
        match current_mode {
            PhysicsMode::Disabled => {
                if !rb.is_kinematic() {
                    rb.set_body_type(RigidBodyType::KinematicPositionBased, false);
                }
                let current_pos = rb.translation();
                let current_rot = rb.rotation().angle();
                if (current_pos.x - visual_pos.x).abs() > 1e-5
                    || (current_pos.y - visual_pos.y).abs() > 1e-5
                    || (current_rot - visual_rot).abs() > 1e-5
                {
                    rb.set_translation(Vector::new(visual_pos.x, visual_pos.y), false);
                    rb.set_rotation(rapier2d::math::Rotation::new(visual_rot), false);
                    rb.sleep();
                }

                // Set colliders as sensors so they don't collide or generate contact forces
                let mut needs_update = false;
                for &col_handle in rb.colliders() {
                    if let Some(collider) = collider_set.get(col_handle) {
                        if !collider.is_sensor() {
                            needs_update = true;
                            break;
                        }
                    }
                }
                if needs_update {
                    for &col_handle in rb.colliders() {
                        if let Some(collider) = collider_set.get_mut(col_handle) {
                            collider.set_sensor(true);
                        }
                    }
                }
            }
            PhysicsMode::Kinematic => {
                if !rb.is_kinematic() {
                    rb.set_body_type(RigidBodyType::KinematicPositionBased, true);
                }
                rb.set_next_kinematic_translation(Vector::new(visual_pos.x, visual_pos.y));
                rb.set_next_kinematic_rotation(rapier2d::math::Rotation::new(visual_rot));

                // Set colliders as solid
                let mut needs_update = false;
                for &col_handle in rb.colliders() {
                    if let Some(collider) = collider_set.get(col_handle) {
                        if collider.is_sensor() {
                            needs_update = true;
                            break;
                        }
                    }
                }
                if needs_update {
                    for &col_handle in rb.colliders() {
                        if let Some(collider) = collider_set.get_mut(col_handle) {
                            collider.set_sensor(false);
                        }
                    }
                }
            }
            PhysicsMode::Dynamic => {
                if !rb.is_dynamic() {
                    rb.set_body_type(RigidBodyType::Dynamic, true);
                    rb.set_translation(Vector::new(visual_pos.x, visual_pos.y), true);
                    rb.set_rotation(rapier2d::math::Rotation::new(visual_rot), true);
                    rb.wake_up(true);
                }

                // Set colliders as solid
                let mut needs_update = false;
                for &col_handle in rb.colliders() {
                    if let Some(collider) = collider_set.get(col_handle) {
                        if collider.is_sensor() {
                            needs_update = true;
                            break;
                        }
                    }
                }
                if needs_update {
                    for &col_handle in rb.colliders() {
                        if let Some(collider) = collider_set.get_mut(col_handle) {
                            collider.set_sensor(false);
                        }
                    }
                }
            }
        }
    }
}
