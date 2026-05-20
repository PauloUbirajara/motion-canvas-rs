use crate::core::animation::{Node, Signal};
use crate::elements::physics::traits::PhysicsBody;
use crate::elements::physics::{PhysicsMode, PhysicsShape, RigidBodyNode, StaticBodyNode};
use glam::Vec2;
use kurbo::Affine;
use rapier2d::prelude::{ColliderSet, RigidBody};
use std::time::Duration;

#[cfg(feature = "runtime")]
use vello::Scene;

/// Internal composite enum representing physical wrapper nodes registered in the engine.
#[derive(Clone)]
pub enum BodyWrapper {
    /// A dynamic/kinematic rigid body wrapper.
    Dynamic(RigidBodyNode),
    /// A static fixed body wrapper.
    Static(StaticBodyNode),
}

impl BodyWrapper {
    /// Accesses the underlying visual shape configuration.
    pub fn shape(&self) -> &PhysicsShape {
        match self {
            Self::Dynamic(n) => &n.shape,
            Self::Static(n) => &n.shape,
        }
    }

    /// Accesses the restitution coefficient.
    pub fn bounciness(&self) -> f32 {
        match self {
            Self::Dynamic(n) => n.bounciness,
            Self::Static(n) => n.bounciness,
        }
    }

    /// Accesses the friction coefficient.
    pub fn friction(&self) -> f32 {
        match self {
            Self::Dynamic(n) => n.friction,
            Self::Static(n) => n.friction,
        }
    }
}

impl Node for BodyWrapper {
    #[cfg(feature = "runtime")]
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        match self {
            BodyWrapper::Dynamic(n) => n.render(scene, parent_transform, parent_opacity),
            BodyWrapper::Static(n) => n.render(scene, parent_transform, parent_opacity),
        }
    }

    fn update(&mut self, dt: Duration) {
        match self {
            BodyWrapper::Dynamic(n) => n.update(dt),
            BodyWrapper::Static(n) => n.update(dt),
        }
    }

    fn state_hash(&self) -> u64 {
        match self {
            BodyWrapper::Dynamic(n) => n.state_hash(),
            BodyWrapper::Static(n) => n.state_hash(),
        }
    }

    fn clone_node(&self) -> Box<dyn Node> {
        match self {
            BodyWrapper::Dynamic(n) => Box::new(n.clone()),
            BodyWrapper::Static(n) => Box::new(n.clone()),
        }
    }

    fn reset(&mut self) {
        match self {
            BodyWrapper::Dynamic(n) => n.reset(),
            BodyWrapper::Static(n) => n.reset(),
        }
    }
}

impl PhysicsBody for BodyWrapper {
    fn mode(&self) -> PhysicsMode {
        match self {
            BodyWrapper::Dynamic(n) => n.mode(),
            BodyWrapper::Static(n) => n.mode(),
        }
    }

    fn position_signal(&self) -> &Signal<Vec2> {
        match self {
            BodyWrapper::Dynamic(n) => n.position_signal(),
            BodyWrapper::Static(n) => n.position_signal(),
        }
    }

    fn rotation_signal(&self) -> &Signal<f32> {
        match self {
            BodyWrapper::Dynamic(n) => n.rotation_signal(),
            BodyWrapper::Static(n) => n.rotation_signal(),
        }
    }

    fn sync_to_rapier(&mut self, rb: &mut RigidBody, collider_set: &mut ColliderSet) {
        match self {
            BodyWrapper::Dynamic(n) => n.sync_to_rapier(rb, collider_set),
            BodyWrapper::Static(n) => n.sync_to_rapier(rb, collider_set),
        }
    }
}
