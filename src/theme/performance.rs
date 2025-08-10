// Performance monitoring and optimization for visual effects
use std::time::Instant;

/// Visual quality levels for performance scaling
#[derive(Clone, Copy, Debug)]
pub enum VisualQuality {
    Low,
    Medium,
    High,
    Ultra,
}

/// Performance settings for controlling visual effects
#[derive(Clone, Debug)]
pub struct PerformanceSettings {
    pub visual_quality: VisualQuality,
    pub enable_glow_effects: bool,
    pub enable_gradients: bool,
    pub enable_animations: bool,
    pub enable_particles: bool,
    pub max_glow_layers: u8,
    pub gradient_steps: usize,
}

impl Default for PerformanceSettings {
    fn default() -> Self {
        Self {
            visual_quality: VisualQuality::High,
            enable_glow_effects: true,
            enable_gradients: true,
            enable_animations: true,
            enable_particles: true,
            max_glow_layers: 4,
            gradient_steps: 32,
        }
    }
}

impl PerformanceSettings {
    /// Create low performance settings
    pub fn low_performance() -> Self {
        Self {
            visual_quality: VisualQuality::Low,
            enable_glow_effects: false,
            enable_gradients: false,
            enable_animations: false,
            enable_particles: false,
            max_glow_layers: 1,
            gradient_steps: 8,
        }
    }

    /// Create medium performance settings
    pub fn medium_performance() -> Self {
        Self {
            visual_quality: VisualQuality::Medium,
            enable_glow_effects: true,
            enable_gradients: true,
            enable_animations: false,
            enable_particles: false,
            max_glow_layers: 2,
            gradient_steps: 16,
        }
    }

    /// Create high performance settings (default)
    pub fn high_performance() -> Self {
        Self::default()
    }

    /// Create ultra performance settings
    pub fn ultra_performance() -> Self {
        Self {
            visual_quality: VisualQuality::Ultra,
            enable_glow_effects: true,
            enable_gradients: true,
            enable_animations: true,
            enable_particles: true,
            max_glow_layers: 6,
            gradient_steps: 64,
        }
    }
}

/// Performance monitor for tracking FPS and adjusting quality
#[derive(Default)]
pub struct PerformanceMonitor {
    frame_times: Vec<f32>,
    last_update: Option<Instant>,
    current_fps: f32,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the performance monitor with current frame timing
    pub fn update(&mut self) {
        let now = Instant::now();
        if let Some(last) = self.last_update {
            let frame_time = now.duration_since(last).as_secs_f32();
            self.frame_times.push(frame_time);

            // Keep only last 60 frames
            if self.frame_times.len() > 60 {
                self.frame_times.remove(0);
            }

            // Calculate average FPS
            let avg_frame_time =
                self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32;
            self.current_fps = if avg_frame_time > 0.0 {
                1.0 / avg_frame_time
            } else {
                0.0
            };
        }
        self.last_update = Some(now);
    }

    /// Get the current FPS
    pub fn get_fps(&self) -> f32 {
        self.current_fps
    }

    /// Check if quality should be reduced due to poor performance
    pub fn should_reduce_quality(&self) -> bool {
        self.current_fps < 30.0 && self.frame_times.len() >= 30
    }

    /// Suggest appropriate quality level based on current performance
    pub fn suggest_quality(&self) -> VisualQuality {
        if self.current_fps >= 60.0 {
            VisualQuality::Ultra
        } else if self.current_fps >= 45.0 {
            VisualQuality::High
        } else if self.current_fps >= 30.0 {
            VisualQuality::Medium
        } else {
            VisualQuality::Low
        }
    }
}
