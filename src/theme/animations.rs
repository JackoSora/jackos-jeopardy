// Animation system with easing functions and state management
use std::time::{Duration, Instant};

/// Types of easing functions available
#[derive(Clone, Copy, Debug)]
pub enum EasingType {
    Linear,
    EaseInOut,
    EaseOutBounce,
    EaseInElastic,
}

/// Animation state for managing time-based animations
#[derive(Clone, Debug)]
pub struct AnimationState {
    pub start_time: Instant,
    pub duration: Duration,
    pub easing_function: EasingType,
    pub current_value: f32,
}

impl AnimationState {
    /// Create a new animation state
    pub fn new(duration: Duration, easing_function: EasingType) -> Self {
        Self {
            start_time: Instant::now(),
            duration,
            easing_function,
            current_value: 0.0,
        }
    }

    /// Update the animation and return the current progress (0.0 to 1.0)
    pub fn update(&mut self) -> f32 {
        let elapsed = self.start_time.elapsed();
        let t = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);

        self.current_value = match self.easing_function {
            EasingType::Linear => t,
            EasingType::EaseInOut => ease_in_out(t),
            EasingType::EaseOutBounce => ease_out_bounce(t),
            EasingType::EaseInElastic => ease_in_elastic(t),
        };

        self.current_value
    }

    /// Check if the animation is finished
    pub fn is_finished(&self) -> bool {
        self.start_time.elapsed() >= self.duration
    }

    /// Restart the animation from the beginning
    pub fn restart(&mut self) {
        self.start_time = Instant::now();
        self.current_value = 0.0;
    }
}

/// Ease-in-out function for smooth animations
pub fn ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

/// Bounce easing function for playful animations
pub fn ease_out_bounce(t: f32) -> f32 {
    if t < 1.0 / 2.75 {
        7.5625 * t * t
    } else if t < 2.0 / 2.75 {
        let t = t - 1.5 / 2.75;
        7.5625 * t * t + 0.75
    } else if t < 2.5 / 2.75 {
        let t = t - 2.25 / 2.75;
        7.5625 * t * t + 0.9375
    } else {
        let t = t - 2.625 / 2.75;
        7.5625 * t * t + 0.984375
    }
}

/// Elastic easing function for spring-like animations
pub fn ease_in_elastic(t: f32) -> f32 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        let c4 = (2.0 * std::f32::consts::PI) / 3.0;
        -(2.0_f32.powf(10.0 * t - 10.0)) * ((t * 10.0 - 10.75) * c4).sin()
    }
}

/// Animation controller for managing multiple animations
#[derive(Default)]
pub struct AnimationController {
    animations: std::collections::HashMap<String, AnimationState>,
}

impl AnimationController {
    /// Create a new animation controller
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a new animation with the given name
    pub fn start_animation(&mut self, name: String, duration: Duration, easing: EasingType) {
        self.animations
            .insert(name, AnimationState::new(duration, easing));
    }

    /// Update an animation and return its current progress
    pub fn update_animation(&mut self, name: &str) -> Option<f32> {
        if let Some(animation) = self.animations.get_mut(name) {
            Some(animation.update())
        } else {
            None
        }
    }

    /// Check if an animation is finished
    pub fn is_animation_finished(&self, name: &str) -> bool {
        self.animations.get(name).map_or(true, |a| a.is_finished())
    }

    /// Remove all finished animations to free memory
    pub fn remove_finished_animations(&mut self) {
        self.animations
            .retain(|_, animation| !animation.is_finished());
    }

    /// Restart an existing animation
    pub fn restart_animation(&mut self, name: &str) {
        if let Some(animation) = self.animations.get_mut(name) {
            animation.restart();
        }
    }
}
