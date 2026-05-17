#![cfg(feature = "physics")]
//! Headless physics simulation pipeline wrapper for the Motion Canvas engine.
//!
//! This module encapsulates all platform-agnostic Rapier2D simulation structures,
//! resource sets, and stepping operations, keeping the layout algorithms decoupled
//! from visual rendering logic.

use rapier2d::prelude::*;

/// A purely headless container for the underlying Rapier2D simulation context.
pub struct PhysicsEngine {
    pub gravity: Vector<f32>,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
}

impl Default for PhysicsEngine {
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

    /// Steps the mathematical simulation forward by the specified delta time frame factor.
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
