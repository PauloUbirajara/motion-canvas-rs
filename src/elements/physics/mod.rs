#![cfg(feature = "physics")]
//! Physics container and body nodes wrapper for the Motion Canvas engine.

use crate::core::animation::{Node, Signal, Tweenable};
use crate::core::physics::PhysicsEngine;
use glam::Vec2;
use kurbo::Affine;
use rapier2d::prelude::{
    ColliderBuilder, RigidBodyBuilder, RigidBodyHandle, RigidBodyType, Vector,
};
use std::time::Duration;

#[cfg(feature = "runtime")]
use vello::Scene;

pub const DEFAULT_BOUNCINESS: f32 = 0.5;
pub const DEFAULT_GRAVITY_Y: f32 = 981.0;
pub const DEFAULT_FRICTION: f32 = 0.5;
pub const DEFAULT_TIMESTEP_SECS: f32 = 1.0 / 60.0;

/// Controls the operational layout ownership framework of a physical item mid-timeline.
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

#[derive(Clone, Debug)]
pub enum PhysicsShape {
    Cuboid(Vec2),
    Ball(f32),
}

impl PhysicsShape {
    fn to_collider(&self) -> ColliderBuilder {
        match self {
            PhysicsShape::Cuboid(half) => ColliderBuilder::cuboid(half.x, half.y),
            PhysicsShape::Ball(r) => ColliderBuilder::ball(*r),
        }
    }
}

/// A composite tracking entry storing a visual element wrapper state configuration.
#[derive(Clone)]
pub enum BodyWrapper {
    Dynamic(RigidBodyNode),
    Static(StaticBodyNode),
}

impl BodyWrapper {
    pub fn mode(&self) -> PhysicsMode {
        match self {
            BodyWrapper::Dynamic(n) => n.mode.get(),
            BodyWrapper::Static(_) => PhysicsMode::Disabled,
        }
    }

    pub fn position_signal(&self) -> &Signal<Vec2> {
        match self {
            BodyWrapper::Dynamic(n) => &n.position,
            BodyWrapper::Static(n) => &n.position,
        }
    }

    pub fn rotation_signal(&self) -> &Signal<f32> {
        match self {
            BodyWrapper::Dynamic(n) => &n.rotation,
            BodyWrapper::Static(n) => &n.rotation,
        }
    }
}

pub struct RigidBodyNode {
    pub inner: Box<dyn Node>,
    pub position: Signal<Vec2>,
    pub rotation: Signal<f32>,
    pub mode: Signal<PhysicsMode>,
    pub shape: PhysicsShape,
    pub bounciness: f32,
    pub friction: f32,
    pub initial_velocity: Vec2,
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

    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.position = Signal::new(pos);
        self
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = Signal::new(rotation);
        self
    }

    pub fn with_shape(mut self, shape: PhysicsShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn with_bounciness(mut self, bounciness: f32) -> Self {
        self.bounciness = bounciness;
        self
    }

    pub fn with_friction(mut self, friction: f32) -> Self {
        self.friction = friction;
        self
    }

    pub fn with_initial_velocity(mut self, vel: Vec2) -> Self {
        self.initial_velocity = vel;
        self
    }

    pub fn with_initial_angular_velocity(mut self, ang_vel: f32) -> Self {
        self.initial_angular_velocity = ang_vel;
        self
    }

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

pub struct StaticBodyNode {
    pub inner: Box<dyn Node>,
    pub position: Signal<Vec2>,
    pub rotation: Signal<f32>,
    pub shape: PhysicsShape,
    pub bounciness: f32,
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

    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.position = Signal::new(pos);
        self
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = Signal::new(rotation);
        self
    }

    pub fn with_shape(mut self, shape: PhysicsShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn with_bounciness(mut self, bounciness: f32) -> Self {
        self.bounciness = bounciness;
        self
    }

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

pub struct PhysicsNode {
    pub opacity: Signal<f32>,
    engine: PhysicsEngine,
    entries: Vec<(RigidBodyHandle, BodyWrapper)>,
    initial_states: Vec<(RigidBodyHandle, Vector<f32>, f32, Vector<f32>, f32)>,
    pub timestep: f32,
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

            let col_builder = self.build_collider_from_handle(*handle);
            cloned.add_entry_internal(entry.clone(), rb_builder, col_builder);
        }
        cloned
    }
}

impl PhysicsNode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_timestep(mut self, timestep: f32) -> Self {
        self.timestep = timestep;
        self
    }

    pub fn with_gravity(mut self, gravity: Vec2) -> Self {
        self.engine.gravity = Vector::new(gravity.x, gravity.y);
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

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

    fn build_collider_from_handle(&self, handle: RigidBodyHandle) -> ColliderBuilder {
        for (_, collider) in self.engine.collider_set.iter() {
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
        ColliderBuilder::cuboid(1.0, 1.0)
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
            let rb = match self.engine.rigid_body_set.get(*handle) {
                Some(rb) => rb,
                None => continue,
            };

            let trans = rb.translation();
            let rot = rb.rotation().angle();
            let local =
                Affine::translate((trans.x as f64, trans.y as f64)) * Affine::rotate(rot as f64);

            match wrapper {
                BodyWrapper::Dynamic(n) => {
                    n.inner
                        .render(scene, parent_transform * local, combined_opacity)
                }
                BodyWrapper::Static(n) => {
                    n.inner
                        .render(scene, parent_transform * local, combined_opacity)
                }
            }
        }
    }

    fn update(&mut self, dt: Duration) {
        let dt_secs = dt.as_secs_f32();
        if self.opacity.get() <= 0.0 || dt_secs <= 0.0 {
            return;
        }

        // Core tick pipeline layout modifications pass down to children first
        for (_, wrapper) in &mut self.entries {
            match wrapper {
                BodyWrapper::Dynamic(n) => n.update(dt),
                BodyWrapper::Static(n) => n.update(dt),
            }
        }

        self.accumulator += dt_secs;

        while self.accumulator >= self.timestep {
            // ─── Step 1: Sync Signals down to Rapier Core ───
            for (handle, wrapper) in &mut self.entries {
                let visual_pos = wrapper.position_signal().get();
                let visual_rot = wrapper.rotation_signal().get();

                if let Some(rb) = self.engine.rigid_body_set.get_mut(*handle) {
                    match wrapper {
                        BodyWrapper::Static(_) => {
                            if !rb.is_fixed() {
                                rb.set_body_type(RigidBodyType::Fixed, true);
                            }
                            rb.set_translation(Vector::new(visual_pos.x, visual_pos.y), true);
                            rb.set_rotation(rapier2d::math::Rotation::new(visual_rot), true);
                        }
                        BodyWrapper::Dynamic(db) => {
                            let current_mode = db.mode.get();
                            match current_mode {
                                PhysicsMode::Disabled => {
                                    if !rb.is_kinematic() {
                                        rb.set_body_type(
                                            RigidBodyType::KinematicPositionBased,
                                            false,
                                        );
                                    }
                                    rb.sleep();
                                }
                                PhysicsMode::Kinematic => {
                                    if !rb.is_kinematic() {
                                        rb.set_body_type(
                                            RigidBodyType::KinematicPositionBased,
                                            true,
                                        );
                                    }
                                    rb.set_next_kinematic_translation(Vector::new(
                                        visual_pos.x,
                                        visual_pos.y,
                                    ));
                                    rb.set_next_kinematic_rotation(rapier2d::math::Rotation::new(
                                        visual_rot,
                                    ));
                                }
                                PhysicsMode::Dynamic => {
                                    if !rb.is_dynamic() {
                                        rb.set_body_type(RigidBodyType::Dynamic, true);
                                        rb.set_translation(
                                            Vector::new(visual_pos.x, visual_pos.y),
                                            true,
                                        );
                                        rb.set_rotation(
                                            rapier2d::math::Rotation::new(visual_rot),
                                            true,
                                        );
                                        rb.wake_up(true);
                                    }
                                }
                            }
                        }
                    }
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
            match wrapper {
                BodyWrapper::Dynamic(n) => h.update_u64(n.inner.state_hash()),
                BodyWrapper::Static(n) => h.update_u64(n.inner.state_hash()),
            }
        }
        h.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn reset(&mut self) {
        // Reset sub-nodes/children first
        for (_, wrapper) in &mut self.entries {
            match wrapper {
                BodyWrapper::Dynamic(n) => n.reset(),
                BodyWrapper::Static(n) => n.reset(),
            }
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

            let col_builder = self.build_collider_from_handle(*handle);

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
