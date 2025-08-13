// Transition coordination system for managing multiple animations
use crate::theme::animations::{AnimationState, AnimationStatus, EasingFunction};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

/// Unique identifier for animations
pub type AnimationId = u64;

/// Handle for tracking and controlling animations
#[derive(Clone, Copy, Debug)]
pub struct AnimationHandle {
    pub id: AnimationId,
    pub created_at: Instant,
}

impl AnimationHandle {
    pub fn new(id: AnimationId) -> Self {
        Self {
            id,
            created_at: Instant::now(),
        }
    }
}

/// Types of transitions that can be coordinated
#[derive(Clone, Debug)]
pub enum TransitionType {
    ModeSwitch { from: String, to: String },
    LayoutChange { component: String },
    StateChange { element: String },
    PhaseTransition { from: String, to: String },
}

/// A pending transition waiting to be executed
#[derive(Clone, Debug)]
pub struct PendingTransition {
    pub id: AnimationId,
    pub transition_type: TransitionType,
    pub duration: Duration,
    pub easing: EasingFunction,
    pub delay: Duration,
    pub priority: u8,
}

/// An active transition currently being executed
#[derive(Clone, Debug)]
pub struct ActiveTransition {
    pub id: AnimationId,
    pub transition_type: TransitionType,
    pub animation_state: AnimationState,
    pub priority: u8,
}

/// Coordinates multiple animations and manages transitions
#[derive(Clone)]
pub struct TransitionController {
    next_id: AnimationId,
    active_animations: HashMap<AnimationId, ActiveTransition>,
    animation_queue: VecDeque<PendingTransition>,
    max_concurrent_animations: usize,
    performance_monitor: PerformanceMonitor,
    accessibility_settings: AccessibilitySettings,
    complexity_scaler: ComplexityScaler,
}

impl TransitionController {
    /// Create a new transition controller
    pub fn new() -> Self {
        Self {
            next_id: 1,
            active_animations: HashMap::new(),
            animation_queue: VecDeque::new(),
            max_concurrent_animations: 8,
            performance_monitor: PerformanceMonitor::new(),
            accessibility_settings: AccessibilitySettings::default(),
            complexity_scaler: ComplexityScaler::new(),
        }
    }

    /// Update accessibility settings (e.g., from system preferences)
    pub fn set_accessibility_settings(&mut self, settings: AccessibilitySettings) {
        // If reduced motion is enabled, cancel non-essential animations
        if settings.reduce_motion {
            self.cancel_non_essential_animations();
        }

        self.accessibility_settings = settings;
    }

    /// Cancel animations that are not essential for functionality
    fn cancel_non_essential_animations(&mut self) {
        let essential_types = [std::mem::discriminant(&TransitionType::ModeSwitch {
            from: String::new(),
            to: String::new(),
        })];

        self.active_animations.retain(|_, transition| {
            let is_essential =
                essential_types.contains(&std::mem::discriminant(&transition.transition_type));
            if !is_essential {
                transition.animation_state.cancel();
            }
            is_essential
        });

        self.animation_queue.retain(|pending| {
            essential_types.contains(&std::mem::discriminant(&pending.transition_type))
        });
    }

    /// Queue a new transition
    pub fn queue_transition(
        &mut self,
        transition_type: TransitionType,
        duration: Duration,
        easing: EasingFunction,
        delay: Duration,
        priority: u8,
    ) -> AnimationHandle {
        let id = self.next_id;
        self.next_id += 1;

        let pending = PendingTransition {
            id,
            transition_type,
            duration,
            easing,
            delay,
            priority,
        };

        // Insert based on priority (higher priority first)
        let mut inserted = false;
        for (i, existing) in self.animation_queue.iter().enumerate() {
            if priority > existing.priority {
                self.animation_queue.insert(i, pending.clone());
                inserted = true;
                break;
            }
        }
        if !inserted {
            self.animation_queue.push_back(pending);
        }

        AnimationHandle::new(id)
    }

    /// Start a transition immediately
    pub fn start_transition(
        &mut self,
        transition_type: TransitionType,
        duration: Duration,
        easing: EasingFunction,
    ) -> AnimationHandle {
        let id = self.next_id;
        self.next_id += 1;

        let mut animation_state = AnimationState::new(duration, easing);
        animation_state.start();

        let active = ActiveTransition {
            id,
            transition_type,
            animation_state,
            priority: 5, // Default priority
        };

        self.active_animations.insert(id, active);
        AnimationHandle::new(id)
    }

    /// Update all active animations and process queue
    pub fn update(&mut self) {
        self.performance_monitor.frame_start();

        // Update complexity scaler based on performance
        self.complexity_scaler
            .update(&self.performance_monitor.metrics);

        // Update max concurrent animations based on complexity
        self.max_concurrent_animations = self.complexity_scaler.max_concurrent_animations();

        // Update active animations
        let mut completed = Vec::new();
        for (id, transition) in &mut self.active_animations {
            transition.animation_state.update();
            if transition.animation_state.is_complete() {
                completed.push(*id);
            }
        }

        // Remove completed animations
        for id in completed {
            self.active_animations.remove(&id);
        }

        // Process queue if we have capacity
        while self.active_animations.len() < self.max_concurrent_animations
            && !self.animation_queue.is_empty()
        {
            if let Some(pending) = self.animation_queue.pop_front() {
                // Check if delay has passed
                if pending.delay == Duration::ZERO {
                    // Apply accessibility and complexity adjustments
                    let adjusted_duration = self
                        .accessibility_settings
                        .adjust_duration(self.complexity_scaler.scale_duration(pending.duration));

                    let mut animation_state =
                        AnimationState::new(adjusted_duration, pending.easing);
                    animation_state.start();

                    let active = ActiveTransition {
                        id: pending.id,
                        transition_type: pending.transition_type,
                        animation_state,
                        priority: pending.priority,
                    };

                    self.active_animations.insert(pending.id, active);
                } else {
                    // Re-queue with reduced delay
                    let mut delayed = pending;
                    delayed.delay = delayed.delay.saturating_sub(Duration::from_millis(16)); // Assume 60fps
                    self.animation_queue.push_front(delayed);
                    break;
                }
            }
        }

        self.performance_monitor.frame_end();
    }

    /// Get the current progress of an animation
    pub fn get_progress(&self, handle: AnimationHandle) -> Option<f32> {
        self.active_animations
            .get(&handle.id)
            .map(|t| t.animation_state.progress)
    }

    /// Get the eased progress of an animation
    pub fn get_eased_progress(&self, handle: AnimationHandle) -> Option<f32> {
        self.active_animations
            .get(&handle.id)
            .map(|t| (t.animation_state.easing)(t.animation_state.progress))
    }

    /// Check if an animation is still running
    pub fn is_running(&self, handle: AnimationHandle) -> bool {
        self.active_animations
            .get(&handle.id)
            .map(|t| t.animation_state.status == AnimationStatus::Running)
            .unwrap_or(false)
    }

    /// Cancel an animation
    pub fn cancel_animation(&mut self, handle: AnimationHandle) {
        if let Some(transition) = self.active_animations.get_mut(&handle.id) {
            transition.animation_state.cancel();
        }
        // Also remove from queue if it's there
        self.animation_queue.retain(|p| p.id != handle.id);
    }

    /// Cancel all animations of a specific type
    pub fn cancel_transitions_of_type(&mut self, transition_type: &TransitionType) {
        // Cancel active animations
        for transition in self.active_animations.values_mut() {
            if std::mem::discriminant(&transition.transition_type)
                == std::mem::discriminant(transition_type)
            {
                transition.animation_state.cancel();
            }
        }

        // Remove from queue
        self.animation_queue.retain(|p| {
            std::mem::discriminant(&p.transition_type) != std::mem::discriminant(transition_type)
        });
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> &PerformanceMetrics {
        &self.performance_monitor.metrics
    }

    /// Check if the system is under performance pressure
    pub fn is_performance_stressed(&self) -> bool {
        self.performance_monitor.is_stressed()
    }

    /// Get count of active animations
    pub fn active_count(&self) -> usize {
        self.active_animations.len()
    }

    /// Get count of queued animations
    pub fn queued_count(&self) -> usize {
        self.animation_queue.len()
    }

    /// Get current complexity scale
    pub fn get_complexity_scale(&self) -> f32 {
        self.complexity_scaler.get_scale()
    }

    /// Check if complex effects should be disabled
    pub fn should_disable_complex_effects(&self) -> bool {
        self.complexity_scaler.should_disable_complex_effects()
            || self.accessibility_settings.reduce_motion
    }

    /// Get current accessibility settings
    pub fn get_accessibility_settings(&self) -> &AccessibilitySettings {
        &self.accessibility_settings
    }
}

impl Default for TransitionController {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance monitoring for animations
#[derive(Clone)]
pub struct PerformanceMonitor {
    pub metrics: PerformanceMetrics,
    frame_start_time: Option<Instant>,
    frame_times: VecDeque<Duration>,
    max_frame_history: usize,
}

/// Performance metrics for animation system
#[derive(Clone, Debug)]
pub struct PerformanceMetrics {
    pub average_frame_time: Duration,
    pub current_fps: f32,
    pub dropped_frames: u32,
    pub stress_level: f32, // 0.0 to 1.0
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics: PerformanceMetrics {
                average_frame_time: Duration::from_millis(16),
                current_fps: 60.0,
                dropped_frames: 0,
                stress_level: 0.0,
            },
            frame_start_time: None,
            frame_times: VecDeque::new(),
            max_frame_history: 60, // Track last 60 frames
        }
    }

    pub fn frame_start(&mut self) {
        self.frame_start_time = Some(Instant::now());
    }

    pub fn frame_end(&mut self) {
        if let Some(start_time) = self.frame_start_time.take() {
            let frame_time = start_time.elapsed();

            // Track frame times
            self.frame_times.push_back(frame_time);
            if self.frame_times.len() > self.max_frame_history {
                self.frame_times.pop_front();
            }

            // Update metrics
            self.update_metrics();
        }
    }

    fn update_metrics(&mut self) {
        if self.frame_times.is_empty() {
            return;
        }

        // Calculate average frame time
        let total_time: Duration = self.frame_times.iter().sum();
        self.metrics.average_frame_time = total_time / self.frame_times.len() as u32;

        // Calculate FPS
        self.metrics.current_fps = 1.0 / self.metrics.average_frame_time.as_secs_f32();

        // Count dropped frames (frames over 16.67ms for 60fps)
        let target_frame_time = Duration::from_millis(16);
        self.metrics.dropped_frames = self
            .frame_times
            .iter()
            .filter(|&&time| time > target_frame_time)
            .count() as u32;

        // Calculate stress level
        let stress_ratio = self.metrics.dropped_frames as f32 / self.frame_times.len() as f32;
        self.metrics.stress_level = stress_ratio.clamp(0.0, 1.0);
    }

    pub fn is_stressed(&self) -> bool {
        self.metrics.stress_level > 0.3 || self.metrics.current_fps < 45.0
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Accessibility settings for animation system
#[derive(Clone, Debug)]
pub struct AccessibilitySettings {
    pub reduce_motion: bool,
    pub high_contrast: bool,
    pub animation_duration_multiplier: f32, // 0.0 = instant, 1.0 = normal, 2.0 = slow
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self {
            reduce_motion: false,
            high_contrast: false,
            animation_duration_multiplier: 1.0,
        }
    }
}

impl AccessibilitySettings {
    /// Create settings for reduced motion
    pub fn reduced_motion() -> Self {
        Self {
            reduce_motion: true,
            high_contrast: false,
            animation_duration_multiplier: 0.5,
        }
    }

    /// Create settings for no animations
    pub fn no_animations() -> Self {
        Self {
            reduce_motion: true,
            high_contrast: false,
            animation_duration_multiplier: 0.0,
        }
    }

    /// Adjust duration based on accessibility settings
    pub fn adjust_duration(&self, duration: Duration) -> Duration {
        if self.reduce_motion && self.animation_duration_multiplier == 0.0 {
            Duration::ZERO
        } else {
            duration.mul_f32(self.animation_duration_multiplier)
        }
    }
}

/// Scales animation complexity based on performance
#[derive(Clone)]
pub struct ComplexityScaler {
    current_scale: f32,
    target_scale: f32,
    adaptation_rate: f32,
}

impl ComplexityScaler {
    pub fn new() -> Self {
        Self {
            current_scale: 1.0,
            target_scale: 1.0,
            adaptation_rate: 0.1,
        }
    }

    /// Update complexity scale based on performance metrics
    pub fn update(&mut self, metrics: &PerformanceMetrics) {
        // Determine target scale based on performance
        self.target_scale = if metrics.stress_level > 0.7 {
            0.3 // Very low complexity
        } else if metrics.stress_level > 0.5 {
            0.6 // Reduced complexity
        } else if metrics.stress_level > 0.3 {
            0.8 // Slightly reduced
        } else {
            1.0 // Full complexity
        };

        // Smoothly adapt to target scale
        let diff = self.target_scale - self.current_scale;
        self.current_scale += diff * self.adaptation_rate;
        self.current_scale = self.current_scale.clamp(0.1, 1.0);
    }

    /// Get current complexity scale (0.1 to 1.0)
    pub fn get_scale(&self) -> f32 {
        self.current_scale
    }

    /// Scale a duration based on current complexity
    pub fn scale_duration(&self, duration: Duration) -> Duration {
        if self.current_scale < 0.5 {
            // At low complexity, make animations faster
            duration.mul_f32(0.5)
        } else {
            duration
        }
    }

    /// Determine if complex effects should be disabled
    pub fn should_disable_complex_effects(&self) -> bool {
        self.current_scale < 0.7
    }

    /// Get maximum concurrent animations based on complexity
    pub fn max_concurrent_animations(&self) -> usize {
        if self.current_scale < 0.4 {
            2
        } else if self.current_scale < 0.7 {
            4
        } else {
            8
        }
    }
}

impl Default for ComplexityScaler {
    fn default() -> Self {
        Self::new()
    }
}
