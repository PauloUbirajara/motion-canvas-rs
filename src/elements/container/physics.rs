#![cfg(feature = "physics")]
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

pub const DEFAULT_BOUNCINESS: f32 = 0.5;
pub const DEFAULT_GRAVITY_Y: f32 = 981.0;
pub const DEFAULT_FRICTION: f32 = 0.5;

// ─── Shape Abstraction ───────────────────────────────────

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

// ─── Wrapper Nodes ───────────────────────────────────────

pub struct RigidBodyNode {
    pub inner: Box<dyn Node>,
    pub position: Vec2,
    pub rotation: f32, // in radians
    pub shape: PhysicsShape,
    pub bounciness: f32,
    pub friction: f32,
    pub initial_velocity: Vec2,
    pub initial_angular_velocity: f32,
}

impl RigidBodyNode {
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

    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.position = pos;
        self
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
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

pub struct StaticBodyNode {
    pub inner: Box<dyn Node>,
    pub position: Vec2,
    pub rotation: f32, // in radians
    pub shape: PhysicsShape,
    pub bounciness: f32,
    pub friction: f32,
}

impl StaticBodyNode {
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

    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.position = pos;
        self
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_gravity(mut self, gravity: Vec2) -> Self {
        self.gravity = Vector::new(gravity.x, gravity.y);
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

    pub fn with_moving_container(mut self, enabled: bool) -> Self {
        self.is_moving_container = enabled;
        self
    }

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
