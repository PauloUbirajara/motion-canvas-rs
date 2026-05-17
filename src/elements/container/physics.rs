#![cfg(feature = "physics")]
//! Physics container and body nodes wrapper for the Motion Canvas engine.
//!
//! This module provides a simplified, higher-level abstraction over the 2D Rapier physics engine,
//! allowing nodes to participate in realistic physics simulations (gravity, collisions, friction, restitution)
//! without requiring direct interaction with complex Rapier builder patterns or simulation loops.

use crate::core::animation::{Node, Signal};
use glam::Vec2;
use kurbo::Affine;
use rapier2d::prelude::{
    BroadPhase, CCDSolver, ColliderBuilder, ColliderSet, ImpulseJointSet, IntegrationParameters,
    IslandManager, MultibodyJointSet, NarrowPhase, PhysicsPipeline, QueryPipeline,
    RigidBodyBuilder, RigidBodyHandle, RigidBodySet, Vector,
};
use std::time::Duration;

#[cfg(feature = "runtime")]
use vello::Scene;

/// The default coefficient of restitution (bounciness) for rigid bodies in the physics simulation.
///
/// A value of `0.0` represents no bounce (perfectly inelastic collision),
/// while `1.0` represents a fully elastic collision where no kinetic energy is lost.
pub const DEFAULT_BOUNCINESS: f32 = 0.5;

/// The default acceleration due to gravity along the vertical Y axis (in pixels per second squared).
///
/// Motion Canvas matches screenspace coordinates, where positive Y points downwards.
/// Therefore, a positive vertical gravity value causes objects to fall downwards.
pub const DEFAULT_GRAVITY_Y: f32 = 981.0;

/// The default coefficient of friction for rigid and static bodies in the physics simulation.
///
/// Determines the resistance to sliding motion when two surfaces are in contact.
/// A higher value causes objects to slow down or stop faster when sliding against each other.
pub const DEFAULT_FRICTION: f32 = 0.5;

// ─── Shape Abstraction ───────────────────────────────────

/// Represents the geometric shape used for rigid or static physical body colliders.
///
/// These shapes abstract Rapier's low-level collider geometry, mapping directly to
/// standard 2D vector graphic structures (such as circles and rectangles).
#[derive(Clone, Debug)]
pub enum PhysicsShape {
    /// A rectangle/box shape defined by its half-extents (distance from the center to its edges on both axes).
    Cuboid(Vec2),
    /// A circular shape defined by its radius.
    Ball(f32),
}

impl PhysicsShape {
    /// Helper to convert our abstract `PhysicsShape` into a Rapier `ColliderBuilder`.
    fn to_collider(&self) -> ColliderBuilder {
        match self {
            PhysicsShape::Cuboid(half) => ColliderBuilder::cuboid(half.x, half.y),
            PhysicsShape::Ball(r) => ColliderBuilder::ball(*r),
        }
    }
}

// ─── Wrapper Nodes ───────────────────────────────────────

/// A physics node wrapper that represents a dynamic rigid body.
///
/// A `RigidBodyNode` moves in response to gravity, collisions, applied velocities, and spin.
/// It wraps any arbitrary visual `Node` (e.g., `Circle`, `Rect`, `TextNode`), synchronizing
/// the visual representation's position and rotation with the underlying physics simulation
/// state at each step.
///
/// ### Builder Pattern
/// Like other layout and element nodes in this library, `RigidBodyNode` uses a builder pattern.
/// The caller does not need to handle raw Rapier rigid body builders or insert routines.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// let circle_node = Circle::default().with_radius(20.0).with_fill(Color::RED);
/// let rigid_body = RigidBodyNode::new(Box::new(circle_node))
///     .with_position(Vec2::new(100.0, 200.0))
///     .with_shape(PhysicsShape::Ball(20.0))
///     .with_initial_velocity(Vec2::new(150.0, 0.0))
///     .with_bounciness(0.7);
/// ```
pub struct RigidBodyNode {
    /// The nested visual node that is rendered.
    pub inner: Box<dyn Node>,
    /// The initial starting position of this rigid body.
    pub position: Vec2,
    /// The initial rotation of this rigid body in radians.
    pub rotation: f32,
    /// The physical collider shape representation.
    pub shape: PhysicsShape,
    /// The coefficient of restitution (bounciness), defaults to `DEFAULT_BOUNCINESS`.
    pub bounciness: f32,
    /// The coefficient of friction, defaults to `DEFAULT_FRICTION`.
    pub friction: f32,
    /// The starting linear velocity vector of the body in pixels per second.
    pub initial_velocity: Vec2,
    /// The starting angular velocity (spin) in radians per second.
    pub initial_angular_velocity: f32,
}

impl RigidBodyNode {
    /// Creates a new `RigidBodyNode` wrapping a visual `Node`, with default properties.
    pub fn new(inner: Box<dyn Node>) -> Self {
        Self {
            inner,
            position: Vec2::ZERO,
            rotation: 0.0,
            shape: PhysicsShape::Cuboid(Vec2::new(25.0, 25.0)),
            bounciness: DEFAULT_BOUNCINESS,
            friction: DEFAULT_FRICTION,
            initial_velocity: Vec2::ZERO,
            initial_angular_velocity: 0.0,
        }
    }

    /// Sets the initial translation/position.
    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.position = pos;
        self
    }

    /// Sets the initial rotation in radians.
    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    /// Configures the shape used for the physical collider bounds.
    pub fn with_shape(mut self, shape: PhysicsShape) -> Self {
        self.shape = shape;
        self
    }

    /// Sets the bounciness (restitution coefficient).
    pub fn with_bounciness(mut self, bounciness: f32) -> Self {
        self.bounciness = bounciness;
        self
    }

    /// Sets the friction coefficient.
    pub fn with_friction(mut self, friction: f32) -> Self {
        self.friction = friction;
        self
    }

    /// Sets the starting linear velocity.
    pub fn with_initial_velocity(mut self, vel: Vec2) -> Self {
        self.initial_velocity = vel;
        self
    }

    /// Sets the starting angular velocity (spin).
    pub fn with_initial_angular_velocity(mut self, ang_vel: f32) -> Self {
        self.initial_angular_velocity = ang_vel;
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
        Box::new(Self {
            inner: self.inner.clone_node(),
            position: self.position,
            rotation: self.rotation,
            shape: self.shape.clone(),
            bounciness: self.bounciness,
            friction: self.friction,
            initial_velocity: self.initial_velocity,
            initial_angular_velocity: self.initial_angular_velocity,
        })
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

/// A physics node wrapper that represents a static (immovable) body.
///
/// A `StaticBodyNode` acts as an immovable boundary or obstacle in the simulation.
/// Dynamic rigid bodies will bump, bounce, and roll off of it, but static bodies
/// are completely unaffected by gravity, forces, or collisions.
///
/// ### Container Animation Support
/// Static bodies can act as bounding boxes or container structures. If `PhysicsNode::is_moving_container`
/// is enabled, all static bodies in the world will rock, shake, and shift Procedurally over time,
/// imparting kinetic energy and motion to any dynamic balls or shapes trapped inside.
pub struct StaticBodyNode {
    /// The nested visual node that is rendered.
    pub inner: Box<dyn Node>,
    /// The fixed position of this static obstacle.
    pub position: Vec2,
    /// The fixed rotation of this static obstacle in radians.
    pub rotation: f32,
    /// The physical collider shape representation.
    pub shape: PhysicsShape,
    /// The bounciness coefficient when dynamic bodies collide with this static body.
    pub bounciness: f32,
    /// The friction coefficient of the surface.
    pub friction: f32,
}

impl StaticBodyNode {
    /// Creates a new `StaticBodyNode` wrapping a visual `Node` with default settings.
    pub fn new(inner: Box<dyn Node>) -> Self {
        Self {
            inner,
            position: Vec2::ZERO,
            rotation: 0.0,
            shape: PhysicsShape::Cuboid(Vec2::new(25.0, 25.0)),
            bounciness: 0.0,
            friction: DEFAULT_FRICTION,
        }
    }

    /// Sets the fixed translation/position.
    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.position = pos;
        self
    }

    /// Sets the fixed rotation in radians.
    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    /// Configures the shape used for the physical collider bounds.
    pub fn with_shape(mut self, shape: PhysicsShape) -> Self {
        self.shape = shape;
        self
    }

    /// Sets the bounciness (restitution coefficient).
    pub fn with_bounciness(mut self, bounciness: f32) -> Self {
        self.bounciness = bounciness;
        self
    }

    /// Sets the friction coefficient of the static surface.
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
        Box::new(Self {
            inner: self.inner.clone_node(),
            position: self.position,
            rotation: self.rotation,
            shape: self.shape.clone(),
            bounciness: self.bounciness,
            friction: self.friction,
        })
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

// ─── Physics World ───────────────────────────────────────

/// A container node that orchestrates a 2D physics simulation world.
///
/// `PhysicsNode` manages a complete Rapier physics pipeline, integration parameters, gravity,
/// colliders, rigid bodies, and joints. It accepts child `RigidBodyNode` and `StaticBodyNode` instances,
/// updates their positions during its layout step, and automatically handles resetting, scaling,
/// and rendering.
///
/// ### Opacity & Simulation Controls
/// The `opacity` of the `PhysicsNode` serves dual purposes:
/// 1. **Visibility**: Smoothly fades all child dynamic and static nodes during rendering.
/// 2. **Simulation Control**: When opacity is `<= 0.0`, the physics step simulation is completely skipped.
///    This allows you to construct complex presentations where different scenes appear and disappear,
///    preventing background CPU load and maintaining perfect animation sync.
///
/// ### Rocking Container Mode
/// If `with_moving_container(true)` is set, all fixed/static boundary shapes within this world
/// will automatically oscillate (rocking side-to-side and up-and-down), allowing you to simulate
/// shaking boxes, tilting ramps, or vibrating containers that push dynamic bodies around.
pub struct PhysicsNode {
    /// Opacity controls visibility AND simulation.
    /// When opacity <= 0, the physics step is skipped entirely.
    pub opacity: Signal<f32>,

    gravity: Vector<f32>,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    bodies: Vec<(RigidBodyHandle, Box<dyn Node>)>,
    initial_states: Vec<(RigidBodyHandle, Vector<f32>, f32, Vector<f32>, f32)>,

    pub is_moving_container: bool,
    time: f32,
}

impl Default for PhysicsNode {
    fn default() -> Self {
        Self {
            opacity: Signal::new(1.0),
            gravity: Vector::new(0.0, DEFAULT_GRAVITY_Y),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            bodies: Vec::new(),
            initial_states: Vec::new(),
            is_moving_container: false,
            time: 0.0,
        }
    }
}

impl Clone for PhysicsNode {
    /// Clones the PhysicsNode, sharing the opacity signal (for linked animation)
    /// while rebuilding a fresh physics world from initial states.
    fn clone(&self) -> Self {
        let mut cloned = PhysicsNode {
            opacity: self.opacity.clone(), // shared Arc — linked to timeline
            gravity: self.gravity,
            is_moving_container: self.is_moving_container,
            time: self.time,
            ..Default::default()
        };

        for ((handle, node), (_, init_pos, init_rot, init_linvel, init_angvel)) in
            self.bodies.iter().zip(self.initial_states.iter())
        {
            let Some(rb) = self.rigid_body_set.get(*handle) else {
                continue;
            };

            let rb_builder = if rb.is_dynamic() {
                RigidBodyBuilder::dynamic()
                    .translation(*init_pos)
                    .rotation(*init_rot)
                    .linvel(*init_linvel)
                    .angvel(*init_angvel)
            } else {
                RigidBodyBuilder::fixed()
                    .translation(*init_pos)
                    .rotation(*init_rot)
            };

            let col_builder = self.build_collider_from_handle(*handle);
            cloned.add_body(node.clone_node(), rb_builder, col_builder);
        }

        cloned
    }
}

impl PhysicsNode {
    /// Creates a new `PhysicsNode` with default integration settings, active gravity, and no entities.
    pub fn new() -> Self {
        Self::default()
    }

    /// Overrides the gravity vector of the physics world.
    ///
    /// The input is in pixels per second squared. Positive values along the Y axis pull downwards.
    ///
    /// ### Example
    /// ```rust
    /// # use motion_canvas_rs::prelude::*;
    /// let physics = PhysicsNode::new()
    ///     .with_gravity(Vec2::new(0.0, 500.0)); // gentler gravity
    /// ```
    pub fn with_gravity(mut self, gravity: Vec2) -> Self {
        self.gravity = Vector::new(gravity.x, gravity.y);
        self
    }

    /// Sets the initial opacity of the physics world.
    ///
    /// When opacity is set to `0.0`, physics updates/simulations are entirely skipped,
    /// which avoids consuming CPU time for inactive scenes.
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

    /// Enables or disables the moving/rocking container effect on static boundaries.
    ///
    /// When enabled, any obstacle added via `add_static` will oscillate slightly in position
    /// relative to its starting coordinates, dynamic bodies placed within these boundaries will
    /// roll, rock, and bounce due to the shifting contacts.
    pub fn with_moving_container(mut self, enabled: bool) -> Self {
        self.is_moving_container = enabled;
        self
    }

    /// Internal helper to register a new body and its collider in the Rapier sets and initial states.
    fn add_body(
        &mut self,
        node: Box<dyn Node>,
        rb_builder: RigidBodyBuilder,
        col_builder: ColliderBuilder,
    ) {
        let rb = rb_builder.build();
        let initial_trans = *rb.translation();
        let initial_rot = rb.rotation().angle();
        let initial_linvel = *rb.linvel();
        let initial_angvel = rb.angvel();

        let handle = self.rigid_body_set.insert(rb);
        self.collider_set
            .insert_with_parent(col_builder.build(), handle, &mut self.rigid_body_set);
        self.bodies.push((handle, node));
        self.initial_states.push((
            handle,
            initial_trans,
            initial_rot,
            initial_linvel,
            initial_angvel,
        ));
    }

    /// Registers a new dynamic rigid body in the physics world.
    ///
    /// This dynamic body will automatically be simulated with gravity, momentum, bounciness, and friction.
    ///
    /// ### Example
    /// ```rust
    /// # use motion_canvas_rs::prelude::*;
    /// let mut physics = PhysicsNode::new();
    /// let circle_body = RigidBodyNode::new(Box::new(Circle::default().with_radius(10.0)))
    ///     .with_position(Vec2::new(0.0, -100.0))
    ///     .with_shape(PhysicsShape::Ball(10.0));
    /// physics.add_dynamic(circle_body);
    /// ```
    pub fn add_dynamic(&mut self, rb: RigidBodyNode) {
        let pos = rb.position;
        let rot = rb.rotation;
        let linvel = rb.initial_velocity;
        let angvel = rb.initial_angular_velocity;
        let col = rb
            .shape
            .to_collider()
            .restitution(rb.bounciness)
            .friction(rb.friction);
        let builder = RigidBodyBuilder::dynamic()
            .translation(Vector::new(pos.x, pos.y))
            .rotation(rot)
            .linvel(Vector::new(linvel.x, linvel.y))
            .angvel(angvel);
        self.add_body(Box::new(rb), builder, col);
    }

    /// Registers a new static boundary or obstacle in the physics world.
    ///
    /// Static bodies are immovable colliders that dynamic rigid bodies can collide with.
    ///
    /// ### Example
    /// ```rust
    /// # use motion_canvas_rs::prelude::*;
    /// let mut physics = PhysicsNode::new();
    /// let ramp = StaticBodyNode::new(Box::new(Rect::default().with_size(Vec2::new(300.0, 20.0))))
    ///     .with_position(Vec2::new(0.0, 100.0))
    ///     .with_rotation(0.2) // slightly tilted
    ///     .with_shape(PhysicsShape::Cuboid(Vec2::new(150.0, 10.0)));
    /// physics.add_static(ramp);
    /// ```
    pub fn add_static(&mut self, sb: StaticBodyNode) {
        let pos = sb.position;
        let rot = sb.rotation;
        let col = sb
            .shape
            .to_collider()
            .restitution(sb.bounciness)
            .friction(sb.friction);
        let builder = RigidBodyBuilder::fixed()
            .translation(Vector::new(pos.x, pos.y))
            .rotation(rot);
        self.add_body(Box::new(sb), builder, col);
    }
}

impl Node for PhysicsNode {
    #[cfg(feature = "runtime")]
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let opacity = self.opacity.get();
        let combined_opacity = parent_opacity * opacity;
        if combined_opacity <= 0.0 {
            return;
        }

        for (handle, node) in &self.bodies {
            let rb = match self.rigid_body_set.get(*handle) {
                Some(rb) => rb,
                None => continue,
            };

            let trans = rb.translation();
            let rot = rb.rotation().angle();
            let local =
                Affine::translate((trans.x as f64, trans.y as f64)) * Affine::rotate(rot as f64);

            node.render(scene, parent_transform * local, combined_opacity);
        }
    }

    fn update(&mut self, dt: Duration) {
        let dt_secs = dt.as_secs_f32();

        // Only simulate when visible and time is advancing
        if self.opacity.get() > 0.0 && dt_secs > 0.0 {
            self.time += dt_secs;

            if self.is_moving_container {
                // Rock and shake container! Horizontal and vertical sine/cos oscillations
                let dx = (self.time * 2.5).sin() * 70.0;
                let dy = (self.time * 4.5).cos() * 15.0;

                for (handle, initial_pos, _, _, _) in &self.initial_states {
                    let Some(rb) = self.rigid_body_set.get_mut(*handle) else {
                        continue;
                    };
                    if rb.is_fixed() {
                        let new_pos = Vector::new(initial_pos.x + dx, initial_pos.y + dy);
                        rb.set_translation(new_pos, true);
                    }
                }
            }

            self.integration_parameters.dt = dt_secs;
            self.physics_pipeline.step(
                &self.gravity,
                &self.integration_parameters,
                &mut self.island_manager,
                &mut self.broad_phase,
                &mut self.narrow_phase,
                &mut self.rigid_body_set,
                &mut self.collider_set,
                &mut self.impulse_joint_set,
                &mut self.multibody_joint_set,
                &mut self.ccd_solver,
                Some(&mut self.query_pipeline),
                &(),
                &(),
            );
        }

        for (_, node) in &mut self.bodies {
            node.update(dt);
        }
    }

    fn state_hash(&self) -> u64 {
        use crate::assets::hash::Hasher;
        let mut h = Hasher::new();
        h.update_u64(self.opacity.state_hash());

        for (handle, node) in &self.bodies {
            if let Some(rb) = self.rigid_body_set.get(*handle) {
                let trans = rb.translation();
                h.update_u64(trans.x.to_bits() as u64);
                h.update_u64(trans.y.to_bits() as u64);
                h.update_u64(rb.rotation().angle().to_bits() as u64);
            }
            h.update_u64(node.state_hash());
        }

        h.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        let mut cloned = PhysicsNode::new()
            .with_gravity(Vec2::new(self.gravity.x, self.gravity.y))
            .with_opacity(self.opacity.get())
            .with_moving_container(self.is_moving_container);
        cloned.time = self.time;

        for ((handle, node), (_, init_pos, init_rot, init_linvel, init_angvel)) in
            self.bodies.iter().zip(self.initial_states.iter())
        {
            let Some(rb) = self.rigid_body_set.get(*handle) else {
                continue;
            };

            let rb_builder = if rb.is_dynamic() {
                RigidBodyBuilder::dynamic()
                    .translation(*init_pos)
                    .rotation(*init_rot)
                    .linvel(*init_linvel)
                    .angvel(*init_angvel)
            } else {
                RigidBodyBuilder::fixed()
                    .translation(*init_pos)
                    .rotation(*init_rot)
            };

            let col_builder = self.build_collider_from_handle(*handle);
            cloned.add_body(node.clone_node(), rb_builder, col_builder);
        }

        Box::new(cloned)
    }

    fn reset(&mut self) {
        self.time = 0.0;
        for (handle, pos, rot, linvel, angvel) in &self.initial_states {
            let Some(rb) = self.rigid_body_set.get_mut(*handle) else {
                continue;
            };
            rb.set_translation(*pos, true);
            rb.set_rotation(rapier2d::math::Rotation::new(*rot), true);
            rb.set_linvel(*linvel, true);
            rb.set_angvel(*angvel, true);
        }

        for (_, node) in &mut self.bodies {
            node.reset();
        }
    }
}

// ─── Private Helpers ─────────────────────────────────────

impl PhysicsNode {
    fn build_collider_from_handle(&self, handle: RigidBodyHandle) -> ColliderBuilder {
        for (_, collider) in self.collider_set.iter() {
            if collider.parent() != Some(handle) {
                continue;
            }

            let restitution = collider.restitution();
            let friction = collider.friction();
            let shape = collider.shape();

            if let Some(ball) = shape.as_ball() {
                return ColliderBuilder::ball(ball.radius)
                    .restitution(restitution)
                    .friction(friction);
            }

            if let Some(cuboid) = shape.as_cuboid() {
                return ColliderBuilder::cuboid(cuboid.half_extents.x, cuboid.half_extents.y)
                    .restitution(restitution)
                    .friction(friction);
            }
        }

        // Fallback — should never happen in practice
        ColliderBuilder::cuboid(1.0, 1.0)
    }
}
