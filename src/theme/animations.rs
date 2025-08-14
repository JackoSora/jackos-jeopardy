// Enhanced animation system with easing functions and state management
use std::time::{Duration, Instant};

pub type EasingFunction = fn(f32) -> f32;

#[derive(Clone, Debug)]
pub struct AnimationState {
    pub progress: f32,
    pub start_time: Instant,
    pub duration: Duration,
    pub easing: EasingFunction,
    pub status: AnimationStatus,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AnimationStatus {
    Pending,
    Running,
    Completed,
    Cancelled,
}

impl AnimationState {
    pub fn new(duration: Duration, easing: EasingFunction) -> Self {
        Self {
            progress: 0.0,
            start_time: Instant::now(),
            duration,
            easing,
            status: AnimationStatus::Pending,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Instant::now();
        self.status = AnimationStatus::Running;
        self.progress = 0.0;
    }

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

    pub fn is_complete(&self) -> bool {
        self.status == AnimationStatus::Completed
    }

    pub fn cancel(&mut self) {
        self.status = AnimationStatus::Cancelled;
    }
}

pub trait AnimationController {
    type State;

    fn animate_in(&mut self, duration: Duration) -> &mut AnimationState;
    fn animate_out(&mut self, duration: Duration) -> &mut AnimationState;
    fn animate_to(&mut self, target_state: Self::State, duration: Duration) -> &mut AnimationState;
    fn is_animating(&self) -> bool;
    fn update_animations(&mut self);
}

// Enhanced easing functions for smooth transitions

pub fn ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

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

// Removed several unused easing helpers to silence warnings: smooth_step, smoother_step,
// ease_out_elastic, ease_in_cubic, ease_out_cubic, ease_in_out_cubic, linear.