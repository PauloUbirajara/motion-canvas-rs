use crate::core::animation::tween::{FollowPath, SignalTween};
use crate::core::animation::{Animation, Node};
use crate::elements::*;
use glam::Vec2;
use kurbo::Affine;
use peniko::Color;
use std::sync::Arc;
use std::time::Duration;
#[cfg(feature = "runtime")]
use vello::Scene;

/// A static dispatch wrapper for any concrete implementation of `Node`.
///
/// This enum allows storing different types of visual nodes in a unified collection
/// without the performance overhead or lifetime complexities of `Box<dyn Node + 'static>`.
pub enum AnyNode {
    Rect(Rect),
    Circle(Circle),
    Line(Line),
    Polygon(Polygon),
    Grid(GridNode),
    Text(TextNode),
    Path(PathNode),
    Group(GroupNode),
    Camera(CameraNode),
    #[cfg(feature = "code")]
    Code(CodeNode),
    #[cfg(feature = "image")]
    Image(ImageNode),
    #[cfg(feature = "math")]
    Math(MathNode),
    #[cfg(feature = "svg")]
    Svg(SvgNode),
    #[cfg(feature = "physics")]
    RigidBody(RigidBodyNode),
    #[cfg(feature = "physics")]
    StaticBody(StaticBodyNode),
    #[cfg(feature = "physics")]
    Physics(PhysicsNode),
    Custom(Arc<std::sync::Mutex<Box<dyn Node>>>),
}

impl Clone for AnyNode {
    fn clone(&self) -> Self {
        match self {
            Self::Rect(n) => Self::Rect(n.clone()),
            Self::Circle(n) => Self::Circle(n.clone()),
            Self::Line(n) => Self::Line(n.clone()),
            Self::Polygon(n) => Self::Polygon(n.clone()),
            Self::Grid(n) => Self::Grid(n.clone()),
            Self::Text(n) => Self::Text(n.clone()),
            Self::Path(n) => Self::Path(n.clone()),
            Self::Group(n) => Self::Group(n.clone()),
            Self::Camera(n) => Self::Camera(n.clone()),
            #[cfg(feature = "code")]
            Self::Code(n) => Self::Code(n.clone()),
            #[cfg(feature = "image")]
            Self::Image(n) => Self::Image(n.clone()),
            #[cfg(feature = "math")]
            Self::Math(n) => Self::Math(n.clone()),
            #[cfg(feature = "svg")]
            Self::Svg(n) => Self::Svg(n.clone()),
            #[cfg(feature = "physics")]
            Self::RigidBody(n) => Self::RigidBody(n.clone()),
            #[cfg(feature = "physics")]
            Self::StaticBody(n) => Self::StaticBody(n.clone()),
            #[cfg(feature = "physics")]
            Self::Physics(n) => Self::Physics(n.clone()),
            Self::Custom(n) => {
                let locked = n.lock().unwrap();
                Self::Custom(Arc::new(std::sync::Mutex::new(locked.clone_node())))
            }
        }
    }
}

impl Node for AnyNode {
    #[cfg(feature = "runtime")]
    fn render(&self, vello_scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        match self {
            Self::Rect(n) => n.render(vello_scene, parent_transform, parent_opacity),
            Self::Circle(n) => n.render(vello_scene, parent_transform, parent_opacity),
            Self::Line(n) => n.render(vello_scene, parent_transform, parent_opacity),
            Self::Polygon(n) => n.render(vello_scene, parent_transform, parent_opacity),
            Self::Grid(n) => n.render(vello_scene, parent_transform, parent_opacity),
            Self::Text(n) => n.render(vello_scene, parent_transform, parent_opacity),
            Self::Path(n) => n.render(vello_scene, parent_transform, parent_opacity),
            Self::Group(n) => n.render(vello_scene, parent_transform, parent_opacity),
            Self::Camera(n) => n.render(vello_scene, parent_transform, parent_opacity),
            #[cfg(feature = "code")]
            Self::Code(n) => n.render(vello_scene, parent_transform, parent_opacity),
            #[cfg(feature = "image")]
            Self::Image(n) => n.render(vello_scene, parent_transform, parent_opacity),
            #[cfg(feature = "math")]
            Self::Math(n) => n.render(vello_scene, parent_transform, parent_opacity),
            #[cfg(feature = "svg")]
            Self::Svg(n) => n.render(vello_scene, parent_transform, parent_opacity),
            #[cfg(feature = "physics")]
            Self::RigidBody(n) => n.render(vello_scene, parent_transform, parent_opacity),
            #[cfg(feature = "physics")]
            Self::StaticBody(n) => n.render(vello_scene, parent_transform, parent_opacity),
            #[cfg(feature = "physics")]
            Self::Physics(n) => n.render(vello_scene, parent_transform, parent_opacity),
            Self::Custom(n) => {
                n.lock()
                    .unwrap()
                    .render(vello_scene, parent_transform, parent_opacity)
            }
        }
    }

    fn update(&mut self, dt: Duration) {
        match self {
            Self::Rect(n) => n.update(dt),
            Self::Circle(n) => n.update(dt),
            Self::Line(n) => n.update(dt),
            Self::Polygon(n) => n.update(dt),
            Self::Grid(n) => n.update(dt),
            Self::Text(n) => n.update(dt),
            Self::Path(n) => n.update(dt),
            Self::Group(n) => n.update(dt),
            Self::Camera(n) => n.update(dt),
            #[cfg(feature = "code")]
            Self::Code(n) => n.update(dt),
            #[cfg(feature = "image")]
            Self::Image(n) => n.update(dt),
            #[cfg(feature = "math")]
            Self::Math(n) => n.update(dt),
            #[cfg(feature = "svg")]
            Self::Svg(n) => n.update(dt),
            #[cfg(feature = "physics")]
            Self::RigidBody(n) => n.update(dt),
            #[cfg(feature = "physics")]
            Self::StaticBody(n) => n.update(dt),
            #[cfg(feature = "physics")]
            Self::Physics(n) => n.update(dt),
            Self::Custom(n) => n.lock().unwrap().update(dt),
        }
    }

    fn state_hash(&self) -> u64 {
        match self {
            Self::Rect(n) => n.state_hash(),
            Self::Circle(n) => n.state_hash(),
            Self::Line(n) => n.state_hash(),
            Self::Polygon(n) => n.state_hash(),
            Self::Grid(n) => n.state_hash(),
            Self::Text(n) => n.state_hash(),
            Self::Path(n) => n.state_hash(),
            Self::Group(n) => n.state_hash(),
            Self::Camera(n) => n.state_hash(),
            #[cfg(feature = "code")]
            Self::Code(n) => n.state_hash(),
            #[cfg(feature = "image")]
            Self::Image(n) => n.state_hash(),
            #[cfg(feature = "math")]
            Self::Math(n) => n.state_hash(),
            #[cfg(feature = "svg")]
            Self::Svg(n) => n.state_hash(),
            #[cfg(feature = "physics")]
            Self::RigidBody(n) => n.state_hash(),
            #[cfg(feature = "physics")]
            Self::StaticBody(n) => n.state_hash(),
            #[cfg(feature = "physics")]
            Self::Physics(n) => n.state_hash(),
            Self::Custom(n) => n.lock().unwrap().state_hash(),
        }
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn reset(&mut self) {
        match self {
            Self::Rect(n) => n.reset(),
            Self::Circle(n) => n.reset(),
            Self::Line(n) => n.reset(),
            Self::Polygon(n) => n.reset(),
            Self::Grid(n) => n.reset(),
            Self::Text(n) => n.reset(),
            Self::Path(n) => n.reset(),
            Self::Group(n) => n.reset(),
            Self::Camera(n) => n.reset(),
            #[cfg(feature = "code")]
            Self::Code(n) => n.reset(),
            #[cfg(feature = "image")]
            Self::Image(n) => n.reset(),
            #[cfg(feature = "math")]
            Self::Math(n) => n.reset(),
            #[cfg(feature = "svg")]
            Self::Svg(n) => n.reset(),
            #[cfg(feature = "physics")]
            Self::RigidBody(n) => n.reset(),
            #[cfg(feature = "physics")]
            Self::StaticBody(n) => n.reset(),
            #[cfg(feature = "physics")]
            Self::Physics(n) => n.reset(),
            Self::Custom(n) => n.lock().unwrap().reset(),
        }
    }
}

impl From<Rect> for AnyNode {
    fn from(n: Rect) -> Self {
        Self::Rect(n)
    }
}
impl From<Circle> for AnyNode {
    fn from(n: Circle) -> Self {
        Self::Circle(n)
    }
}
impl From<Line> for AnyNode {
    fn from(n: Line) -> Self {
        Self::Line(n)
    }
}
impl From<Polygon> for AnyNode {
    fn from(n: Polygon) -> Self {
        Self::Polygon(n)
    }
}
impl From<GridNode> for AnyNode {
    fn from(n: GridNode) -> Self {
        Self::Grid(n)
    }
}
impl From<TextNode> for AnyNode {
    fn from(n: TextNode) -> Self {
        Self::Text(n)
    }
}
impl From<PathNode> for AnyNode {
    fn from(n: PathNode) -> Self {
        Self::Path(n)
    }
}
impl From<GroupNode> for AnyNode {
    fn from(n: GroupNode) -> Self {
        Self::Group(n)
    }
}
impl From<CameraNode> for AnyNode {
    fn from(n: CameraNode) -> Self {
        Self::Camera(n)
    }
}

#[cfg(feature = "code")]
impl From<CodeNode> for AnyNode {
    fn from(n: CodeNode) -> Self {
        Self::Code(n)
    }
}
#[cfg(feature = "image")]
impl From<ImageNode> for AnyNode {
    fn from(n: ImageNode) -> Self {
        Self::Image(n)
    }
}
#[cfg(feature = "math")]
impl From<MathNode> for AnyNode {
    fn from(n: MathNode) -> Self {
        Self::Math(n)
    }
}
#[cfg(feature = "svg")]
impl From<SvgNode> for AnyNode {
    fn from(n: SvgNode) -> Self {
        Self::Svg(n)
    }
}
#[cfg(feature = "physics")]
impl From<RigidBodyNode> for AnyNode {
    fn from(n: RigidBodyNode) -> Self {
        Self::RigidBody(n)
    }
}
#[cfg(feature = "physics")]
impl From<StaticBodyNode> for AnyNode {
    fn from(n: StaticBodyNode) -> Self {
        Self::StaticBody(n)
    }
}
#[cfg(feature = "physics")]
impl From<PhysicsNode> for AnyNode {
    fn from(n: PhysicsNode) -> Self {
        Self::Physics(n)
    }
}

impl From<Box<Rect>> for AnyNode {
    fn from(n: Box<Rect>) -> Self {
        Self::Rect(*n)
    }
}
impl From<Box<Circle>> for AnyNode {
    fn from(n: Box<Circle>) -> Self {
        Self::Circle(*n)
    }
}
impl From<Box<Line>> for AnyNode {
    fn from(n: Box<Line>) -> Self {
        Self::Line(*n)
    }
}
impl From<Box<Polygon>> for AnyNode {
    fn from(n: Box<Polygon>) -> Self {
        Self::Polygon(*n)
    }
}
impl From<Box<GridNode>> for AnyNode {
    fn from(n: Box<GridNode>) -> Self {
        Self::Grid(*n)
    }
}
impl From<Box<TextNode>> for AnyNode {
    fn from(n: Box<TextNode>) -> Self {
        Self::Text(*n)
    }
}
impl From<Box<PathNode>> for AnyNode {
    fn from(n: Box<PathNode>) -> Self {
        Self::Path(*n)
    }
}
impl From<Box<GroupNode>> for AnyNode {
    fn from(n: Box<GroupNode>) -> Self {
        Self::Group(*n)
    }
}
impl From<Box<CameraNode>> for AnyNode {
    fn from(n: Box<CameraNode>) -> Self {
        Self::Camera(*n)
    }
}

#[cfg(feature = "code")]
impl From<Box<CodeNode>> for AnyNode {
    fn from(n: Box<CodeNode>) -> Self {
        Self::Code(*n)
    }
}
#[cfg(feature = "image")]
impl From<Box<ImageNode>> for AnyNode {
    fn from(n: Box<ImageNode>) -> Self {
        Self::Image(*n)
    }
}
#[cfg(feature = "math")]
impl From<Box<MathNode>> for AnyNode {
    fn from(n: Box<MathNode>) -> Self {
        Self::Math(*n)
    }
}
#[cfg(feature = "svg")]
impl From<Box<SvgNode>> for AnyNode {
    fn from(n: Box<SvgNode>) -> Self {
        Self::Svg(*n)
    }
}
#[cfg(feature = "physics")]
impl From<Box<RigidBodyNode>> for AnyNode {
    fn from(n: Box<RigidBodyNode>) -> Self {
        Self::RigidBody(*n)
    }
}
#[cfg(feature = "physics")]
impl From<Box<StaticBodyNode>> for AnyNode {
    fn from(n: Box<StaticBodyNode>) -> Self {
        Self::StaticBody(*n)
    }
}
#[cfg(feature = "physics")]
impl From<Box<PhysicsNode>> for AnyNode {
    fn from(n: Box<PhysicsNode>) -> Self {
        Self::Physics(*n)
    }
}

impl From<Box<dyn Node>> for AnyNode {
    fn from(n: Box<dyn Node>) -> Self {
        Self::Custom(Arc::new(std::sync::Mutex::new(n)))
    }
}

impl<T, S> From<crate::core::animation::binding::BindingNode<T, S>> for AnyNode
where
    T: crate::core::animation::tween::Tweenable + PartialEq,
    S: crate::core::animation::tween::Tweenable + PartialEq,
{
    fn from(n: crate::core::animation::binding::BindingNode<T, S>) -> Self {
        Self::Custom(Arc::new(std::sync::Mutex::new(Box::new(n))))
    }
}

impl<T, S> From<Box<crate::core::animation::binding::BindingNode<T, S>>> for AnyNode
where
    T: crate::core::animation::tween::Tweenable + PartialEq + 'static,
    S: crate::core::animation::tween::Tweenable + PartialEq + 'static,
{
    fn from(n: Box<crate::core::animation::binding::BindingNode<T, S>>) -> Self {
        Self::Custom(Arc::new(std::sync::Mutex::new(n)))
    }
}

impl From<&Rect> for AnyNode {
    fn from(n: &Rect) -> Self {
        Self::Rect(n.clone())
    }
}
impl From<&Circle> for AnyNode {
    fn from(n: &Circle) -> Self {
        Self::Circle(n.clone())
    }
}
impl From<&Line> for AnyNode {
    fn from(n: &Line) -> Self {
        Self::Line(n.clone())
    }
}
impl From<&Polygon> for AnyNode {
    fn from(n: &Polygon) -> Self {
        Self::Polygon(n.clone())
    }
}
impl From<&GridNode> for AnyNode {
    fn from(n: &GridNode) -> Self {
        Self::Grid(n.clone())
    }
}
impl From<&TextNode> for AnyNode {
    fn from(n: &TextNode) -> Self {
        Self::Text(n.clone())
    }
}
impl From<&PathNode> for AnyNode {
    fn from(n: &PathNode) -> Self {
        Self::Path(n.clone())
    }
}
impl From<&GroupNode> for AnyNode {
    fn from(n: &GroupNode) -> Self {
        Self::Group(n.clone())
    }
}
impl From<&CameraNode> for AnyNode {
    fn from(n: &CameraNode) -> Self {
        Self::Camera(n.clone())
    }
}

#[cfg(feature = "code")]
impl From<&CodeNode> for AnyNode {
    fn from(n: &CodeNode) -> Self {
        Self::Code(n.clone())
    }
}
#[cfg(feature = "image")]
impl From<&ImageNode> for AnyNode {
    fn from(n: &ImageNode) -> Self {
        Self::Image(n.clone())
    }
}
#[cfg(feature = "math")]
impl From<&MathNode> for AnyNode {
    fn from(n: &MathNode) -> Self {
        Self::Math(n.clone())
    }
}
#[cfg(feature = "svg")]
impl From<&SvgNode> for AnyNode {
    fn from(n: &SvgNode) -> Self {
        Self::Svg(n.clone())
    }
}
#[cfg(feature = "physics")]
impl From<&RigidBodyNode> for AnyNode {
    fn from(n: &RigidBodyNode) -> Self {
        Self::RigidBody(n.clone())
    }
}
#[cfg(feature = "physics")]
impl From<&StaticBodyNode> for AnyNode {
    fn from(n: &StaticBodyNode) -> Self {
        Self::StaticBody(n.clone())
    }
}
#[cfg(feature = "physics")]
impl From<&PhysicsNode> for AnyNode {
    fn from(n: &PhysicsNode) -> Self {
        Self::Physics(n.clone())
    }
}

impl From<&AnyNode> for AnyNode {
    fn from(n: &AnyNode) -> Self {
        n.clone()
    }
}

/// A static dispatch wrapper for any concrete implementation of `Animation`.
///
/// This enum avoids heap allocations (`Box<dyn Animation>`) for common operations
/// and control flows.
pub enum AnyAnimation {
    SignalTweenF32(SignalTween<f32>),
    SignalTweenVec2(SignalTween<Vec2>),
    SignalTweenColor(SignalTween<Color>),
    SignalTweenAffine(SignalTween<Affine>),
    SignalTweenString(SignalTween<String>),
    SignalTweenBool(SignalTween<bool>),
    SignalTweenVecVec2(SignalTween<Vec<Vec2>>),
    FollowPathVec2(FollowPath<Vec2>),
    FollowPathAffine(FollowPath<Affine>),
    All(crate::core::animation::flow::all::All),
    Any(crate::core::animation::flow::any::Any),
    Chain(crate::core::animation::flow::chain::Chain),
    Delay(crate::core::animation::flow::delay::Delay),
    Sequence(crate::core::animation::flow::sequence::Sequence),
    Loop(crate::core::animation::flow::loop_anim::LoopAnim),
    Wait(crate::core::animation::flow::wait::Wait),
    #[cfg(feature = "audio")]
    Audio(crate::elements::media::AudioAnimation),
    Custom(Box<dyn Animation>),
}

impl Animation for AnyAnimation {
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        match self {
            Self::SignalTweenF32(a) => a.update(dt),
            Self::SignalTweenVec2(a) => a.update(dt),
            Self::SignalTweenColor(a) => a.update(dt),
            Self::SignalTweenAffine(a) => a.update(dt),
            Self::SignalTweenString(a) => a.update(dt),
            Self::SignalTweenBool(a) => a.update(dt),
            Self::SignalTweenVecVec2(a) => a.update(dt),
            Self::FollowPathVec2(a) => a.update(dt),
            Self::FollowPathAffine(a) => a.update(dt),
            Self::All(a) => a.update(dt),
            Self::Any(a) => a.update(dt),
            Self::Chain(a) => a.update(dt),
            Self::Delay(a) => a.update(dt),
            Self::Sequence(a) => a.update(dt),
            Self::Loop(a) => a.update(dt),
            Self::Wait(a) => a.update(dt),
            #[cfg(feature = "audio")]
            Self::Audio(a) => a.update(dt),
            Self::Custom(a) => a.update(dt),
        }
    }

    fn duration(&self) -> Duration {
        match self {
            Self::SignalTweenF32(a) => a.duration(),
            Self::SignalTweenVec2(a) => a.duration(),
            Self::SignalTweenColor(a) => a.duration(),
            Self::SignalTweenAffine(a) => a.duration(),
            Self::SignalTweenString(a) => a.duration(),
            Self::SignalTweenBool(a) => a.duration(),
            Self::SignalTweenVecVec2(a) => a.duration(),
            Self::FollowPathVec2(a) => a.duration(),
            Self::FollowPathAffine(a) => a.duration(),
            Self::All(a) => a.duration(),
            Self::Any(a) => a.duration(),
            Self::Chain(a) => a.duration(),
            Self::Delay(a) => a.duration(),
            Self::Sequence(a) => a.duration(),
            Self::Loop(a) => a.duration(),
            Self::Wait(a) => a.duration(),
            #[cfg(feature = "audio")]
            Self::Audio(a) => a.duration(),
            Self::Custom(a) => a.duration(),
        }
    }

    fn set_easing(&mut self, easing: fn(f32) -> f32) {
        match self {
            Self::SignalTweenF32(a) => a.set_easing(easing),
            Self::SignalTweenVec2(a) => a.set_easing(easing),
            Self::SignalTweenColor(a) => a.set_easing(easing),
            Self::SignalTweenAffine(a) => a.set_easing(easing),
            Self::SignalTweenString(a) => a.set_easing(easing),
            Self::SignalTweenBool(a) => a.set_easing(easing),
            Self::SignalTweenVecVec2(a) => a.set_easing(easing),
            Self::FollowPathVec2(a) => a.set_easing(easing),
            Self::FollowPathAffine(a) => a.set_easing(easing),
            Self::All(a) => a.set_easing(easing),
            Self::Any(a) => a.set_easing(easing),
            Self::Chain(a) => a.set_easing(easing),
            Self::Delay(a) => a.set_easing(easing),
            Self::Sequence(a) => a.set_easing(easing),
            Self::Loop(a) => a.set_easing(easing),
            Self::Wait(a) => a.set_easing(easing),
            #[cfg(feature = "audio")]
            Self::Audio(a) => a.set_easing(easing),
            Self::Custom(a) => a.set_easing(easing),
        }
    }

    fn collect_audio_events(
        &mut self,
        current_time: Duration,
        events: &mut Vec<crate::core::animation::base::AudioEvent>,
    ) {
        match self {
            Self::SignalTweenF32(a) => a.collect_audio_events(current_time, events),
            Self::SignalTweenVec2(a) => a.collect_audio_events(current_time, events),
            Self::SignalTweenColor(a) => a.collect_audio_events(current_time, events),
            Self::SignalTweenAffine(a) => a.collect_audio_events(current_time, events),
            Self::SignalTweenString(a) => a.collect_audio_events(current_time, events),
            Self::SignalTweenBool(a) => a.collect_audio_events(current_time, events),
            Self::SignalTweenVecVec2(a) => a.collect_audio_events(current_time, events),
            Self::FollowPathVec2(a) => a.collect_audio_events(current_time, events),
            Self::FollowPathAffine(a) => a.collect_audio_events(current_time, events),
            Self::All(a) => a.collect_audio_events(current_time, events),
            Self::Any(a) => a.collect_audio_events(current_time, events),
            Self::Chain(a) => a.collect_audio_events(current_time, events),
            Self::Delay(a) => a.collect_audio_events(current_time, events),
            Self::Sequence(a) => a.collect_audio_events(current_time, events),
            Self::Loop(a) => a.collect_audio_events(current_time, events),
            Self::Wait(a) => a.collect_audio_events(current_time, events),
            #[cfg(feature = "audio")]
            Self::Audio(a) => a.collect_audio_events(current_time, events),
            Self::Custom(a) => a.collect_audio_events(current_time, events),
        }
    }

    fn reset(&mut self) {
        match self {
            Self::SignalTweenF32(a) => a.reset(),
            Self::SignalTweenVec2(a) => a.reset(),
            Self::SignalTweenColor(a) => a.reset(),
            Self::SignalTweenAffine(a) => a.reset(),
            Self::SignalTweenString(a) => a.reset(),
            Self::SignalTweenBool(a) => a.reset(),
            Self::SignalTweenVecVec2(a) => a.reset(),
            Self::FollowPathVec2(a) => a.reset(),
            Self::FollowPathAffine(a) => a.reset(),
            Self::All(a) => a.reset(),
            Self::Any(a) => a.reset(),
            Self::Chain(a) => a.reset(),
            Self::Delay(a) => a.reset(),
            Self::Sequence(a) => a.reset(),
            Self::Loop(a) => a.reset(),
            Self::Wait(a) => a.reset(),
            #[cfg(feature = "audio")]
            Self::Audio(a) => a.reset(),
            Self::Custom(a) => a.reset(),
        }
    }
}

impl<T: crate::core::animation::tween::Tweenable + 'static> From<SignalTween<T>> for AnyAnimation {
    fn from(tween: SignalTween<T>) -> Self {
        let boxed: Box<dyn std::any::Any> = Box::new(tween);
        let boxed = match boxed.downcast::<SignalTween<f32>>() {
            Ok(t) => return Self::SignalTweenF32(*t),
            Err(b) => b,
        };
        let boxed = match boxed.downcast::<SignalTween<Vec2>>() {
            Ok(t) => return Self::SignalTweenVec2(*t),
            Err(b) => b,
        };
        let boxed = match boxed.downcast::<SignalTween<Color>>() {
            Ok(t) => return Self::SignalTweenColor(*t),
            Err(b) => b,
        };
        let boxed = match boxed.downcast::<SignalTween<Affine>>() {
            Ok(t) => return Self::SignalTweenAffine(*t),
            Err(b) => b,
        };
        let boxed = match boxed.downcast::<SignalTween<String>>() {
            Ok(t) => return Self::SignalTweenString(*t),
            Err(b) => b,
        };
        let boxed = match boxed.downcast::<SignalTween<bool>>() {
            Ok(t) => return Self::SignalTweenBool(*t),
            Err(b) => b,
        };
        let boxed = match boxed.downcast::<SignalTween<Vec<Vec2>>>() {
            Ok(t) => return Self::SignalTweenVecVec2(*t),
            Err(b) => b,
        };

        let orig = *boxed.downcast::<SignalTween<T>>().unwrap();
        Self::Custom(Box::new(orig))
    }
}

impl<
        T: crate::core::animation::tween::Tweenable
            + crate::core::animation::tween::FromVec2
            + 'static,
    > From<FollowPath<T>> for AnyAnimation
{
    fn from(anim: FollowPath<T>) -> Self {
        let boxed: Box<dyn std::any::Any> = Box::new(anim);
        let boxed = match boxed.downcast::<FollowPath<Vec2>>() {
            Ok(t) => return Self::FollowPathVec2(*t),
            Err(b) => b,
        };
        let boxed = match boxed.downcast::<FollowPath<Affine>>() {
            Ok(t) => return Self::FollowPathAffine(*t),
            Err(b) => b,
        };

        let orig = *boxed.downcast::<FollowPath<T>>().unwrap();
        Self::Custom(Box::new(orig))
    }
}

impl<T: crate::core::animation::tween::Tweenable + 'static> From<Box<SignalTween<T>>>
    for AnyAnimation
{
    fn from(tween: Box<SignalTween<T>>) -> Self {
        Self::from(*tween)
    }
}

impl<
        T: crate::core::animation::tween::Tweenable
            + crate::core::animation::tween::FromVec2
            + 'static,
    > From<Box<FollowPath<T>>> for AnyAnimation
{
    fn from(anim: Box<FollowPath<T>>) -> Self {
        Self::from(*anim)
    }
}

impl From<crate::core::animation::flow::all::All> for AnyAnimation {
    fn from(a: crate::core::animation::flow::all::All) -> Self {
        Self::All(a)
    }
}
impl From<crate::core::animation::flow::any::Any> for AnyAnimation {
    fn from(a: crate::core::animation::flow::any::Any) -> Self {
        Self::Any(a)
    }
}
impl From<crate::core::animation::flow::chain::Chain> for AnyAnimation {
    fn from(a: crate::core::animation::flow::chain::Chain) -> Self {
        Self::Chain(a)
    }
}
impl From<crate::core::animation::flow::delay::Delay> for AnyAnimation {
    fn from(a: crate::core::animation::flow::delay::Delay) -> Self {
        Self::Delay(a)
    }
}
impl From<crate::core::animation::flow::sequence::Sequence> for AnyAnimation {
    fn from(a: crate::core::animation::flow::sequence::Sequence) -> Self {
        Self::Sequence(a)
    }
}
impl From<crate::core::animation::flow::loop_anim::LoopAnim> for AnyAnimation {
    fn from(a: crate::core::animation::flow::loop_anim::LoopAnim) -> Self {
        Self::Loop(a)
    }
}
impl From<crate::core::animation::flow::wait::Wait> for AnyAnimation {
    fn from(a: crate::core::animation::flow::wait::Wait) -> Self {
        Self::Wait(a)
    }
}

#[cfg(feature = "audio")]
impl From<crate::elements::media::AudioAnimation> for AnyAnimation {
    fn from(a: crate::elements::media::AudioAnimation) -> Self {
        Self::Audio(a)
    }
}

impl From<Box<dyn Animation>> for AnyAnimation {
    fn from(a: Box<dyn Animation>) -> Self {
        Self::Custom(a)
    }
}
