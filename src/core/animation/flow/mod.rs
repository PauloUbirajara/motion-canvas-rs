pub mod all;
pub mod any;
pub mod chain;
pub mod delay;
pub mod easing;
pub mod loop_anim;
pub mod sequence;
pub mod wait;

pub use all::all;
pub use any::any;
pub use chain::chain;
pub use delay::delay;
pub use easing::with_easing;
pub use loop_anim::loop_anim;
pub use sequence::sequence;
pub use wait::wait;

#[macro_export]
macro_rules! all {
    ($($anim:expr),* $(,)?) => {
        $crate::flows::all(vec![$(Box::new($anim) as Box<dyn $crate::core::animation::base::Animation>),*])
    };
}

#[macro_export]
macro_rules! any {
    ($($anim:expr),* $(,)?) => {
        $crate::flows::any(vec![$(Box::new($anim) as Box<dyn $crate::core::animation::base::Animation>),*])
    };
}

#[macro_export]
macro_rules! chain {
    ($($anim:expr),* $(,)?) => {
        $crate::flows::chain(vec![$(Box::new($anim) as Box<dyn $crate::core::animation::base::Animation>),*])
    };
}

#[macro_export]
macro_rules! delay {
    ($d:expr, $anim:expr $(,)?) => {
        $crate::flows::delay(
            $d,
            Box::new($anim) as Box<dyn $crate::core::animation::base::Animation>,
        )
    };
}

#[macro_export]
macro_rules! sequence {
    ($stagger:expr, $($anim:expr),* $(,)?) => {
        $crate::flows::sequence($stagger, vec![$(Box::new($anim) as Box<dyn $crate::core::animation::base::Animation>),*])
    };
}

#[macro_export]
macro_rules! loop_anim {
    ($factory:expr, $iters:expr $(,)?) => {
        $crate::flows::loop_anim(
            Box::new(move || {
                Box::new($factory) as Box<dyn $crate::core::animation::base::Animation>
            }),
            $iters,
        )
    };
}

#[macro_export]
macro_rules! with_easing {
    ($easing:expr, [$($anim:expr),* $(,)?] $(,)?) => {
        $crate::flows::with_easing($easing, vec![$(Box::new($anim) as Box<dyn $crate::core::animation::base::Animation>),*])
    };
}

#[cfg(feature = "audio")]
#[macro_export]
macro_rules! play {
    ($node:expr) => {
        Box::new($crate::elements::media::AudioAnimation::new($node))
            as Box<dyn $crate::core::animation::base::Animation>
    };
}

#[cfg(feature = "audio")]
#[macro_export]
macro_rules! audio_wait {
    ($d:expr) => {
        $crate::flows::wait(std::time::Duration::from_secs_f32($d as f32))
    };
}
