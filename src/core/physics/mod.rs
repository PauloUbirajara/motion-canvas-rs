#![cfg(feature = "physics")]
//! Headless physics simulation pipeline wrapper for the Motion Canvas engine.
//!
//! This module encapsulates all platform-agnostic Rapier2D simulation structures,
//! resource sets, and stepping operations, keeping the layout algorithms decoupled
//! from visual rendering logic.
//!
//! ### Core Components
//! - [`PhysicsEngine`]: Headless coordinator wrapping Rapier2D datasets (bodies, colliders, joints)
//!   and updating them systematically with stable timestep sub-stepping.

use rapier2d::prelude::*;

/// A purely headless container for the underlying Rapier2D simulation context.
///
/// It aggregates the necessary sets and solvers needed to simulate rigid bodies,
/// colliders, joints, and continuous collision detection (CCD).
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::core::physics::PhysicsEngine;
/// let mut engine = PhysicsEngine::new(0.0, 981.0);
/// engine.step(1.0 / 60.0);
/// ```
pub struct PhysicsEngine {
    /// The global gravity acceleration vector (pixels/second^2).
    pub gravity: Vector<f32>,
    /// The set containing all active rigid bodies in the simulation.
    pub rigid_body_set: RigidBodySet,
    /// The set containing all active colliders in the simulation.
    pub collider_set: ColliderSet,
    /// Parameters controlling integration math, such as the timestep and solver iteration count.
    pub integration_parameters: IntegrationParameters,
    /// The main pipeline structure managing execution steps of the physics loop.
    pub physics_pipeline: PhysicsPipeline,
    /// Dynamic grouping manager to optimize sleeping and active simulation islands.
    pub island_manager: IslandManager,
    /// Broad-phase collision detection system for fast coordinate-based overlap filters.
    pub broad_phase: BroadPhase,
    /// Narrow-phase collision detection system determining exact contact manifolds.
    pub narrow_phase: NarrowPhase,
    /// Impulse joints set representing structural constraints like pins, hinges, and springs.
    pub impulse_joint_set: ImpulseJointSet,
    /// Multibody joints set for complex articulated coordinate links.
    pub multibody_joint_set: MultibodyJointSet,
    /// Continuous Collision Detection solver preventing fast-moving bodies from passing through walls.
    pub ccd_solver: CCDSolver,
    /// Spatial query pipeline optimized for real-time raycasting, point projection, and region intersections.
    pub query_pipeline: QueryPipeline,
}

impl Default for PhysicsEngine {
    /// Creates a default physics engine with standard gravity (981 px/s^2 downwards) and empty resource sets.
    fn default() -> Self {
        Self {
            gravity: Vector::new(0.0, 981.0),
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
        }
    }
}

impl PhysicsEngine {
    /// Creates a fresh instance of the headless simulation pipeline with custom gravity parameters.
    pub fn new(gravity_x: f32, gravity_y: f32) -> Self {
        Self {
            gravity: Vector::new(gravity_x, gravity_y),
            ..Default::default()
        }
    }

    /// Steps the mathematical simulation forward by the specified delta time frame factor in seconds.
    pub fn step(&mut self, dt_secs: f32) {
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
}
