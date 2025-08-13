// Enhanced animation system with easing functions and state management
use std::time::{Duration, Instant};

/// Easing function type for smooth animations
pub type EasingFunction = fn(f32) -> f32;

/// Animation state tracking for smooth transitions
#[derive(Clone, Debug)]
pub struct AnimationState {
    pub progress: f32,
    pub start_time: Instant,
    pub duration: Duration,
    pub easing: EasingFunction,
    pub status: AnimationStatus,
}

/// Status of an animation
#[derive(Clone, Debug, PartialEq)]
pub enum AnimationStatus {
    Pending,
    Running,
    Completed,
    Cancelled,
}

impl AnimationState {
    /// Create a new animation state
    pub fn new(duration: Duration, easing: EasingFunction) -> Self {
        Self {
            progress: 0.0,
            start_time: Instant::now(),
            duration,
            easing,
            status: AnimationStatus::Pending,
        }
    }

    /// Start the animation
    pub fn start(&mut self) {
        self.start_time = Instant::now();
        self.status = AnimationStatus::Running;
        self.progress = 0.0;
    }

    /// Update animation progress and return eased value
    pub fn update(&mut self) -> f32 {
        if self.status != AnimationStatus::Running {
            return if self.status == AnimationStatus::Completed {
                1.0
            } else {
                0.0
            };
        }

        let elapsed = self.start_time.elapsed();
        if elapsed >= self.duration {
            self.progress = 1.0;
            self.status = AnimationStatus::Completed;
        } else {
            self.progress = elapsed.as_secs_f32() / self.duration.as_secs_f32();
        }

        (self.easing)(self.progress.clamp(0.0, 1.0))
    }

    /// Check if animation is complete
    pub fn is_complete(&self) -> bool {
        self.status == AnimationStatus::Completed
    }

    /// Cancel the animation
    pub fn cancel(&mut self) {
        self.status = AnimationStatus::Cancelled;
    }
}

/// Trait for objects that can be animated
pub trait AnimationController {
    type State;

    fn animate_in(&mut self, duration: Duration) -> &mut AnimationState;
    fn animate_out(&mut self, duration: Duration) -> &mut AnimationState;
    fn animate_to(&mut self, target_state: Self::State, duration: Duration) -> &mut AnimationState;
    fn is_animating(&self) -> bool;
    fn update_animations(&mut self);
}

// Enhanced easing functions for smooth transitions

/// Smooth ease in-out transition
pub fn ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

/// Smooth step function for very smooth transitions
pub fn smooth_step(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// Smoother step function for ultra-smooth transitions
pub fn smoother_step(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Bounce easing out for playful effects
pub fn ease_out_bounce(t: f32) -> f32 {
    if t < 1.0 / 2.75 {
        7.5625 * t * t
    } else if t < 2.0 / 2.75 {
        7.5625 * (t - 1.5 / 2.75) * (t - 1.5 / 2.75) + 0.75
    } else if t < 2.5 / 2.75 {
        7.5625 * (t - 2.25 / 2.75) * (t - 2.25 / 2.75) + 0.9375
    } else {
        7.5625 * (t - 2.625 / 2.75) * (t - 2.625 / 2.75) + 0.984375
    }
}

/// Elastic easing out for spring-like effects
pub fn ease_out_elastic(t: f32) -> f32 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        let p = 0.3;
        let s = p / 4.0;
        2.0_f32.powf(-10.0 * t) * ((t - s) * (2.0 * std::f32::consts::PI) / p).sin() + 1.0
    }
}

/// Ease in cubic for smooth acceleration
pub fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
}

/// Ease out cubic for smooth deceleration
pub fn ease_out_cubic(t: f32) -> f32 {
    let t = t - 1.0;
    t * t * t + 1.0
}

/// Ease in-out cubic for balanced acceleration/deceleration
pub fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let t = 2.0 * t - 2.0;
        1.0 + t * t * t / 2.0
    }
}

/// Linear easing (no easing)
pub fn linear(t: f32) -> f32 {
    t
}
