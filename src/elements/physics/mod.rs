#![cfg(feature = "physics")]
//! Physics container and body nodes wrapper for the Motion Canvas engine.
//!
//! This module integrates the 2D physics engine (powered by Rapier2D) into the reactive
//! signal-based scene graph of `motion-canvas-rs`. It enables physics-driven simulations
//! (gravity, collisions, friction, restitution) to seamlessly co-exist and transition with
//! traditional signal-driven animations.

pub mod dynamic_body;
pub mod static_body;
pub mod traits;
pub mod wrapper;

pub use dynamic_body::RigidBodyNode;
pub use static_body::StaticBodyNode;
pub use traits::PhysicsBody;
pub use wrapper::BodyWrapper;

use crate::core::animation::{Node, Signal, Tweenable};
use crate::core::physics::PhysicsEngine;
use glam::Vec2;
use kurbo::Affine;
use rapier2d::prelude::{
    ColliderBuilder, RigidBodyBuilder, RigidBodyHandle, RigidBodyType, Vector,
};
use std::sync::Arc;
use std::time::Duration;

#[cfg(feature = "runtime")]
use vello::Scene;

/// Default bounciness (restitution coefficient) for newly created rigid bodies.
pub const DEFAULT_BOUNCINESS: f32 = 0.5;

/// Default gravity acceleration along the Y-axis (pixels/second^2).
pub use crate::core::physics::DEFAULT_GRAVITY_Y;

/// Default friction coefficient for newly created colliders.
pub const DEFAULT_FRICTION: f32 = 0.5;

/// Default simulation timestep in seconds (1/60s).
pub const DEFAULT_TIMESTEP_SECS: f32 = 1.0 / 60.0;

/// Controls the operational layout ownership framework of a physical item mid-timeline.
///
/// This mode is reactive (stored in a `Signal`) and can be changed dynamically during playback
/// to achieve smooth handoffs between keyframe animations and physics simulations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PhysicsMode {
    /// Item is ignored by physics calculations. Driven entirely by standard `.to()` tweens.
    Disabled,
    /// Item acts as an animated obstacle. It impacts dynamic bodies, but its position maps manually to tweens or binding links.
    Kinematic,
    /// Rapier owns the item spatial parameters. Forces, gravity, and contacts dictate layout behavior.
    Dynamic,
}

impl Tweenable for PhysicsMode {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        if t >= 1.0 {
            *b
        } else {
            *a
        }
    }
    fn state_hash(&self) -> u64 {
        *self as u64
    }
}

/// Bounding shapes used to represent physical colliders in the simulation.
#[derive(Clone)]
pub enum PhysicsShape {
    /// A rectangle/box collider defined by its half-extents (width / 2, height / 2).
    Cuboid(Vec2),
    /// A circular collider defined by its radius.
    Ball(f32),
    /// A custom collider shape defined by a custom closure returning a ColliderBuilder.
    Custom(Arc<dyn Fn() -> ColliderBuilder + Send + Sync>),
}

impl std::fmt::Debug for PhysicsShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cuboid(v) => f.debug_tuple("Cuboid").field(v).finish(),
            Self::Ball(r) => f.debug_tuple("Ball").field(r).finish(),
            Self::Custom(_) => f.debug_tuple("Custom").finish(),
        }
    }
}

impl PhysicsShape {
    /// Converts this shape into a Rapier `ColliderBuilder` configuration.
    fn to_collider(&self) -> ColliderBuilder {
        match self {
            PhysicsShape::Cuboid(half) => ColliderBuilder::cuboid(half.x, half.y),
            PhysicsShape::Ball(r) => ColliderBuilder::ball(*r),
            PhysicsShape::Custom(f) => f(),
        }
    }
}

/// The main coordinate space container driving real-time 2D physics simulations.
///
/// `PhysicsNode` coordinates updates between the Rapier2D solver and the visual node graph.
/// It supports custom simulation timesteps, gravity, and opacity.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// let container = PhysicsNode::new()
///     .with_timestep(1.0 / 120.0) // sub-stepping for extra collision fidelity
///     .with_gravity(Vec2::new(0.0, 9.81 * 100.0));
/// ```
pub struct PhysicsNode {
    /// Visual opacity of all elements contained in the simulation.
    pub opacity: Signal<f32>,
    /// The underlying Rapier2D simulator physics pipeline wrapper.
    engine: PhysicsEngine,
    /// Registered bodies currently synchronized within the engine.
    entries: Vec<(RigidBodyHandle, BodyWrapper)>,
    /// Cached original states used to perform accurate visual reset sequences.
    initial_states: Vec<(RigidBodyHandle, Vector<f32>, f32, Vector<f32>, f32)>,
    /// Constant step integration time (e.g. 1/60s). Simulators require fixed updates for deterministic results.
    pub timestep: f32,
    /// Frame time accumulator buffer used to guarantee perfect deterministic reproduction.
    accumulator: f32,
}

impl Default for PhysicsNode {
    fn default() -> Self {
        Self {
            opacity: Signal::new(1.0),
            engine: PhysicsEngine::default(),
            entries: Vec::new(),
            initial_states: Vec::new(),
            timestep: DEFAULT_TIMESTEP_SECS,
            accumulator: 0.0,
        }
    }
}

impl Clone for PhysicsNode {
    fn clone(&self) -> Self {
        let mut cloned = PhysicsNode {
            opacity: self.opacity.clone(),
            engine: PhysicsEngine::new(self.engine.gravity.x, self.engine.gravity.y),
            timestep: self.timestep,
            accumulator: self.accumulator,
            ..Default::default()
        };

        for ((handle, entry), (_, init_pos, init_rot, init_linvel, init_angvel)) in
            self.entries.iter().zip(self.initial_states.iter())
        {
            let Some(rb) = self.engine.rigid_body_set.get(*handle) else {
                continue;
            };

            let rb_builder = if rb.is_dynamic() {
                RigidBodyBuilder::dynamic()
                    .translation(*init_pos)
                    .rotation(*init_rot)
                    .linvel(*init_linvel)
                    .angvel(*init_angvel)
            } else if rb.is_kinematic() {
                RigidBodyBuilder::kinematic_position_based()
                    .translation(*init_pos)
                    .rotation(*init_rot)
            } else {
                RigidBodyBuilder::fixed()
                    .translation(*init_pos)
                    .rotation(*init_rot)
            };

            let col_builder = entry
                .shape()
                .to_collider()
                .restitution(entry.bounciness())
                .friction(entry.friction());
            cloned.add_entry_internal(entry.clone(), rb_builder, col_builder);
        }
        cloned
    }
}

impl PhysicsNode {
    /// Creates a new physics simulation node with standard default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the fixed simulation timestep (default: 1/60s).
    pub fn with_timestep(mut self, timestep: f32) -> Self {
        self.timestep = timestep;
        self
    }

    /// Sets the gravity acceleration vector (pixels/second^2).
    pub fn with_gravity(mut self, gravity: Vec2) -> Self {
        self.engine.gravity = Vector::new(gravity.x, gravity.y);
        self
    }

    /// Sets the visual opacity of all bodies nested in this physics container.
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

    /// Core internal routine inserting built Rapier bodies and colliders into the active pipeline.
    fn add_entry_internal(
        &mut self,
        wrapper: BodyWrapper,
        rb_builder: RigidBodyBuilder,
        col_builder: ColliderBuilder,
    ) {
        let rb = rb_builder.build();
        let initial_trans = *rb.translation();
        let initial_rot = rb.rotation().angle();
        let initial_linvel = *rb.linvel();
        let initial_angvel = rb.angvel();

        let handle = self.engine.rigid_body_set.insert(rb);
        self.engine.collider_set.insert_with_parent(
            col_builder.build(),
            handle,
            &mut self.engine.rigid_body_set,
        );

        self.entries.push((handle, wrapper));
        self.initial_states.push((
            handle,
            initial_trans,
            initial_rot,
            initial_linvel,
            initial_angvel,
        ));
    }

    /// Adds a dynamic (or kinematic/disabled) rigid body to the physics simulation workspace.
    pub fn add_dynamic(&mut self, rb: RigidBodyNode) {
        let pos = rb.position.get();
        let rot = rb.rotation.get();
        let linvel = rb.initial_velocity;
        let angvel = rb.initial_angular_velocity;

        let col = rb
            .shape
            .to_collider()
            .restitution(rb.bounciness)
            .friction(rb.friction);

        let initial_type = match rb.mode.get() {
            PhysicsMode::Disabled | PhysicsMode::Dynamic => RigidBodyType::Dynamic,
            PhysicsMode::Kinematic => RigidBodyType::KinematicPositionBased,
        };

        let builder = RigidBodyBuilder::new(initial_type)
            .translation(Vector::new(pos.x, pos.y))
            .rotation(rot)
            .linvel(Vector::new(linvel.x, linvel.y))
            .angvel(angvel);

        self.add_entry_internal(BodyWrapper::Dynamic(rb), builder, col);
    }

    /// Adds an immovable static body obstacle to the physics simulation workspace.
    pub fn add_static(&mut self, sb: StaticBodyNode) {
        let pos = sb.position.get();
        let rot = sb.rotation.get();
        let col = sb
            .shape
            .to_collider()
            .restitution(sb.bounciness)
            .friction(sb.friction);
        let builder = RigidBodyBuilder::fixed()
            .translation(Vector::new(pos.x, pos.y))
            .rotation(rot);

        self.add_entry_internal(BodyWrapper::Static(sb), builder, col);
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

        for (handle, wrapper) in &self.entries {
            let (pos, rot) = if wrapper.mode() == PhysicsMode::Disabled {
                (
                    wrapper.position_signal().get(),
                    wrapper.rotation_signal().get(),
                )
            } else {
                let rb = match self.engine.rigid_body_set.get(*handle) {
                    Some(rb) => rb,
                    None => continue,
                };
                let trans = rb.translation();
                (Vec2::new(trans.x, trans.y), rb.rotation().angle())
            };

            let local =
                Affine::translate((pos.x as f64, pos.y as f64)) * Affine::rotate(rot as f64);

            wrapper.render(scene, parent_transform * local, combined_opacity);
        }
    }

    fn update(&mut self, dt: Duration) {
        let dt_secs = dt.as_secs_f32();
        if self.opacity.get() <= 0.0 || dt_secs <= 0.0 {
            return;
        }

        // Core tick pipeline layout modifications pass down to children first
        for (_, wrapper) in &mut self.entries {
            wrapper.update(dt);
        }

        self.accumulator += dt_secs;

        while self.accumulator >= self.timestep {
            // ─── Step 1: Sync Signals down to Rapier Core ───
            for (handle, wrapper) in &mut self.entries {
                if let Some(rb) = self.engine.rigid_body_set.get_mut(*handle) {
                    wrapper.sync_to_rapier(rb, &mut self.engine.collider_set);
                }
            }

            // ─── Step 2: Step Headless Physics Simulation Engine ───
            self.engine.step(self.timestep);

            // ─── Step 3: Flush Simulation Outputs to Layout Signals ───
            for (handle, wrapper) in &mut self.entries {
                if wrapper.mode() == PhysicsMode::Dynamic {
                    if let Some(rb) = self.engine.rigid_body_set.get(*handle) {
                        let rb_pos = rb.translation();
                        let rb_rot = rb.rotation().angle();

                        wrapper.position_signal().set(Vec2::new(rb_pos.x, rb_pos.y));
                        wrapper.rotation_signal().set(rb_rot);
                    }
                }
            }

            self.accumulator -= self.timestep;
        }
    }

    fn state_hash(&self) -> u64 {
        use crate::assets::hash::Hasher;
        let mut h = Hasher::new();
        h.update_u64(self.opacity.state_hash());

        for (handle, wrapper) in &self.entries {
            if let Some(rb) = self.engine.rigid_body_set.get(*handle) {
                let trans = rb.translation();
                h.update_u64(trans.x.to_bits() as u64);
                h.update_u64(trans.y.to_bits() as u64);
                h.update_u64(rb.rotation().angle().to_bits() as u64);
            }
            h.update_u64(wrapper.state_hash());
        }
        h.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn reset(&mut self) {
        // Reset sub-nodes/children first
        for (_, wrapper) in &mut self.entries {
            wrapper.reset();
        }

        // Recreate clean PhysicsEngine with gravity preserved to clear out internal contact manifold & islands caches
        let mut new_engine = PhysicsEngine::new(self.engine.gravity.x, self.engine.gravity.y);
        let mut new_entries = Vec::new();
        let mut new_initial_states = Vec::new();

        for ((handle, entry), (_, init_pos, init_rot, init_linvel, init_angvel)) in
            self.entries.iter().zip(&self.initial_states)
        {
            let Some(rb) = self.engine.rigid_body_set.get(*handle) else {
                continue;
            };

            let rb_builder = if rb.is_dynamic() {
                RigidBodyBuilder::dynamic()
                    .translation(*init_pos)
                    .rotation(*init_rot)
                    .linvel(*init_linvel)
                    .angvel(*init_angvel)
            } else if rb.is_kinematic() {
                RigidBodyBuilder::kinematic_position_based()
                    .translation(*init_pos)
                    .rotation(*init_rot)
            } else {
                RigidBodyBuilder::fixed()
                    .translation(*init_pos)
                    .rotation(*init_rot)
            };

            let col_builder = entry
                .shape()
                .to_collider()
                .restitution(entry.bounciness())
                .friction(entry.friction());

            let rb_built = rb_builder.build();
            let new_handle = new_engine.rigid_body_set.insert(rb_built);
            new_engine.collider_set.insert_with_parent(
                col_builder.build(),
                new_handle,
                &mut new_engine.rigid_body_set,
            );

            new_entries.push((new_handle, entry.clone()));
            new_initial_states.push((new_handle, *init_pos, *init_rot, *init_linvel, *init_angvel));
        }

        self.engine = new_engine;
        self.entries = new_entries;
        self.initial_states = new_initial_states;
        self.accumulator = 0.0;
    }
}
