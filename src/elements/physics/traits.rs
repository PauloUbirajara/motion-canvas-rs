use crate::core::animation::Signal;
use crate::elements::physics::PhysicsMode;
use glam::Vec2;
use rapier2d::prelude::{ColliderSet, RigidBody};

/// A trait implemented by elements participating in the Rapier 2D simulation.
pub trait PhysicsBody: crate::core::animation::Node {
    /// Returns the current active physics mode.
    fn mode(&self) -> PhysicsMode;

    /// Accesses the underlying reactive position signal.
    fn position_signal(&self) -> &Signal<Vec2>;

    /// Accesses the underlying reactive rotation signal in radians.
    fn rotation_signal(&self) -> &Signal<f32>;

    /// Synchronizes the visual signals and physics mode settings down to the Rapier rigid body and colliders.
    fn sync_to_rapier(&mut self, rb: &mut RigidBody, collider_set: &mut ColliderSet);
}
