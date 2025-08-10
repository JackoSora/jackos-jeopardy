use eframe::egui;
use std::time::{Duration, Instant};

pub struct Palette;
impl Palette {
    // Existing colors (maintained for compatibility)
    pub const CYAN: egui::Color32 = egui::Color32::from_rgb(0, 255, 170);
    pub const MAGENTA: egui::Color32 = egui::Color32::from_rgb(255, 0, 150);
    pub const BG_DARK: egui::Color32 = egui::Color32::from_rgb(10, 10, 18);
    pub const BG_PANEL: egui::Color32 = egui::Color32::from_rgb(20, 20, 36);
    pub const BG_ACTIVE: egui::Color32 = egui::Color32::from_rgb(36, 0, 58);
    pub const TEXT: egui::Color32 = egui::Color32::from_rgb(208, 255, 247);
    
    // New cyberpunk colors
    pub const NEON_BLUE: egui::Color32 = egui::Color32::from_rgb(0, 150, 255);
    pub const ELECTRIC_PURPLE: egui::Color32 = egui::Color32::from_rgb(150, 0, 255);
    pub const CYBER_ORANGE: egui::Color32 = egui::Color32::from_rgb(255, 100, 0);
    pub const NEON_GREEN: egui::Color32 = egui::Color32::from_rgb(57, 255, 20);
    pub const ELECTRIC_PINK: egui::Color32 = egui::Color32::from_rgb(255, 20, 147);
    
    // Gradient support colors
    pub const GLOW_CYAN_INNER: egui::Color32 = egui::Color32::from_rgb(100, 255, 200);
    pub const GLOW_CYAN_OUTER: egui::Color32 = egui::Color32::TRANSPARENT;
    pub const GLOW_MAGENTA_INNER: egui::Color32 = egui::Color32::from_rgb(255, 100, 200);
    pub const GLOW_MAGENTA_OUTER: egui::Color32 = egui::Color32::TRANSPARENT;
    pub const GLOW_BLUE_INNER: egui::Color32 = egui::Color32::from_rgb(100, 200, 255);
    pub const GLOW_BLUE_OUTER: egui::Color32 = egui::Color32::TRANSPARENT;
    
    // Enhanced background colors
    pub const BG_GRADIENT_START: egui::Color32 = egui::Color32::from_rgb(15, 15, 25);
    pub const BG_GRADIENT_END: egui::Color32 = egui::Color32::from_rgb(25, 10, 35);
    pub const PANEL_GRADIENT_START: egui::Color32 = egui::Color32::from_rgb(25, 25, 45);
    pub const PANEL_GRADIENT_END: egui::Color32 = egui::Color32::from_rgb(35, 15, 55);
}

pub fn apply_global_style(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    
    // Enhanced text styling
    visuals.override_text_color = Some(adjust_brightness(Palette::TEXT, 1.05));
    
    // Enhanced window and panel styling
    visuals.window_rounding = 12.0.into();
    visuals.panel_fill = Palette::PANEL_GRADIENT_START;
    visuals.window_fill = adjust_brightness(Palette::BG_ACTIVE, 1.1);
    
    // Enhanced widget styling
    visuals.widgets.noninteractive.bg_fill = Palette::BG_DARK;
    visuals.widgets.noninteractive.fg_stroke.color = adjust_brightness(Palette::TEXT, 0.9);
    
    visuals.widgets.inactive.bg_fill = adjust_brightness(Palette::BG_PANEL, 1.05);
    visuals.widgets.inactive.fg_stroke.color = Palette::TEXT;
    visuals.widgets.inactive.bg_stroke.color = adjust_brightness(Palette::CYAN, 0.7);
    visuals.widgets.inactive.bg_stroke.width = 1.0;
    
    visuals.widgets.active.bg_fill = adjust_brightness(Palette::BG_ACTIVE, 1.2);
    visuals.widgets.active.fg_stroke.color = adjust_brightness(Palette::TEXT, 1.1);
    visuals.widgets.active.bg_stroke.color = adjust_brightness(Palette::CYAN, 1.2);
    visuals.widgets.active.bg_stroke.width = 2.0;
    
    visuals.widgets.hovered.bg_fill = adjust_brightness(Palette::BG_ACTIVE, 1.3);
    visuals.widgets.hovered.fg_stroke.color = adjust_brightness(Palette::TEXT, 1.15);
    visuals.widgets.hovered.bg_stroke.color = adjust_brightness(Palette::CYAN, 1.3);
    visuals.widgets.hovered.bg_stroke.width = 2.5;
    
    // Enhanced selection and focus indicators
    visuals.selection.bg_fill = adjust_brightness(Palette::CYAN, 1.1);
    visuals.selection.stroke.color = adjust_brightness(Palette::CYAN, 1.4);
    visuals.selection.stroke.width = 2.0;
    
    // Enhanced background colors
    visuals.extreme_bg_color = Palette::BG_DARK;
    visuals.faint_bg_color = adjust_brightness(Palette::BG_PANEL, 0.8);
    
    // Enhanced hyperlink styling
    visuals.hyperlink_color = adjust_brightness(Palette::NEON_BLUE, 1.2);
    
    ctx.set_visuals(visuals);

    // Enhanced spacing and sizing
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(12.0, 12.0);
    style.spacing.button_padding = egui::vec2(16.0, 12.0);
    style.spacing.menu_margin = egui::Margin::symmetric(8.0, 8.0);
    style.spacing.indent = 20.0;
    style.spacing.slider_width = 120.0;
    style.spacing.combo_width = 120.0;
    
    // Enhanced interaction settings
    style.interaction.resize_grab_radius_side = 6.0;
    style.interaction.resize_grab_radius_corner = 8.0;
    style.interaction.show_tooltips_only_when_still = false;
    
    // Enhanced text styles
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::proportional(28.0),
    );
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::proportional(16.0),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::proportional(16.0),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::proportional(12.0),
    );
    
    ctx.set_style(style);
}

// Performance optimization and quality settings
#[derive(Clone, Copy, Debug)]
pub enum VisualQuality {
    Low,
    Medium,
    High,
    Ultra,
}

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
    
    pub fn high_performance() -> Self {
        Self::default()
    }
    
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

// Performance-aware rendering functions
pub fn paint_gradient_rect_optimized(
    painter: &egui::Painter,
    rect: egui::Rect,
    color1: egui::Color32,
    color2: egui::Color32,
    vertical: bool,
    rounding: f32,
    settings: &PerformanceSettings,
) {
    if !settings.enable_gradients {
        // Fallback to solid color
        painter.rect_filled(rect, rounding, lerp_color(color1, color2, 0.5));
        return;
    }
    
    let steps = match settings.visual_quality {
        VisualQuality::Low => 8,
        VisualQuality::Medium => 16,
        VisualQuality::High => 32,
        VisualQuality::Ultra => 64,
    };
    
    paint_gradient_rect_with_steps(painter, rect, color1, color2, vertical, rounding, steps);
}

pub fn paint_gradient_rect_with_steps(
    painter: &egui::Painter,
    rect: egui::Rect,
    color1: egui::Color32,
    color2: egui::Color32,
    vertical: bool,
    rounding: f32,
    steps: usize,
) {
    if vertical {
        let step_height = rect.height() / steps as f32;
        for i in 0..steps {
            let t = i as f32 / (steps - 1) as f32;
            let color = lerp_color(color1, color2, t);
            let y = rect.top() + i as f32 * step_height;
            let step_rect = egui::Rect::from_min_size(
                egui::pos2(rect.left(), y),
                egui::vec2(rect.width(), step_height + 1.0),
            );
            painter.rect_filled(step_rect, rounding, color);
        }
    } else {
        let step_width = rect.width() / steps as f32;
        for i in 0..steps {
            let t = i as f32 / (steps - 1) as f32;
            let color = lerp_color(color1, color2, t);
            let x = rect.left() + i as f32 * step_width;
            let step_rect = egui::Rect::from_min_size(
                egui::pos2(x, rect.top()),
                egui::vec2(step_width + 1.0, rect.height()),
            );
            painter.rect_filled(step_rect, rounding, color);
        }
    }
}

pub fn paint_glow_rect_optimized(
    painter: &egui::Painter,
    rect: egui::Rect,
    rounding: f32,
    glow_config: GlowConfig,
    settings: &PerformanceSettings,
) {
    if !settings.enable_glow_effects || glow_config.intensity <= 0.0 || glow_config.radius <= 0.0 {
        return;
    }
    
    let layers = glow_config.layers.min(settings.max_glow_layers).max(1);
    let step_size = glow_config.radius / layers as f32;
    
    for i in 0..layers {
        let layer_progress = i as f32 / (layers - 1) as f32;
        let expansion = step_size * (i + 1) as f32;
        let alpha_factor = (1.0 - layer_progress) * glow_config.intensity;
        
        let layer_color = lerp_color(
            glow_config.inner_color,
            glow_config.outer_color,
            layer_progress,
        );
        
        let final_color = with_alpha(
            layer_color,
            (layer_color.a() as f32 * alpha_factor) as u8,
        );
        
        let expanded_rect = rect.expand(expansion);
        painter.rect_filled(expanded_rect, rounding + expansion * 0.5, final_color);
    }
}

// Performance monitoring
#[derive(Default)]
pub struct PerformanceMonitor {
    frame_times: Vec<f32>,
    last_update: Option<Instant>,
    current_fps: f32,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self::default()
    }
    
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
            let avg_frame_time = self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32;
            self.current_fps = if avg_frame_time > 0.0 { 1.0 / avg_frame_time } else { 0.0 };
        }
        self.last_update = Some(now);
    }
    
    pub fn get_fps(&self) -> f32 {
        self.current_fps
    }
    
    pub fn should_reduce_quality(&self) -> bool {
        self.current_fps < 30.0 && self.frame_times.len() >= 30
    }
    
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

// Enhanced visual indicators for game state
pub fn paint_active_team_indicator(
    painter: &egui::Painter,
    rect: egui::Rect,
    team_name: &str,
    is_active: bool,
) {
    let rounding = 8.0;
    
    if is_active {
        // Enhanced active team styling
        let glow_config = GlowConfig::cyan_glow(0.7, 10.0);
        paint_glow_rect(painter, rect, rounding, glow_config);
        
        // Animated gradient background
        let bg_start = adjust_brightness(Palette::CYAN, 1.2);
        let bg_end = adjust_brightness(Palette::CYAN, 0.8);
        paint_gradient_rect(painter, rect, bg_start, bg_end, true, rounding);
        
        // Enhanced border
        painter.rect_stroke(rect, rounding, egui::Stroke::new(3.0, adjust_brightness(Palette::CYAN, 1.4)));
        
        // Text with enhanced styling
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            format!("▶ {} ◀", team_name),
            egui::FontId::proportional(18.0),
            egui::Color32::BLACK,
        );
    } else {
        // Inactive team styling
        let bg_color = adjust_brightness(Palette::BG_PANEL, 1.1);
        painter.rect_filled(rect, rounding, bg_color);
        painter.rect_stroke(rect, rounding, egui::Stroke::new(1.0, adjust_brightness(Palette::CYAN, 0.6)));
        
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            team_name,
            egui::FontId::proportional(16.0),
            adjust_brightness(Palette::TEXT, 0.8),
        );
    }
}

pub fn paint_game_phase_indicator(
    painter: &egui::Painter,
    rect: egui::Rect,
    phase_text: &str,
    phase_color: egui::Color32,
) {
    let rounding = 10.0;
    
    // Enhanced phase indicator with glow
    let glow_config = GlowConfig::new(phase_color, 0.5, 8.0);
    paint_glow_rect(painter, rect, rounding, glow_config);
    
    // Gradient background
    let bg_start = adjust_brightness(phase_color, 1.1);
    let bg_end = adjust_brightness(phase_color, 0.9);
    paint_gradient_rect(painter, rect, bg_start, bg_end, false, rounding);
    
    // Enhanced border
    painter.rect_stroke(rect, rounding, egui::Stroke::new(2.5, adjust_brightness(phase_color, 1.3)));
    
    // Phase text with enhanced styling
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        phase_text,
        egui::FontId::proportional(20.0),
        egui::Color32::WHITE,
    );
}

// Enhanced transition effects between game phases
pub fn paint_phase_transition_effect(
    painter: &egui::Painter,
    rect: egui::Rect,
    transition_progress: f32,
    from_color: egui::Color32,
    to_color: egui::Color32,
) {
    let center = rect.center();
    let max_radius = rect.width().min(rect.height()) * 0.8;
    
    // Expanding circle transition
    let radius = ease_in_out(transition_progress) * max_radius;
    let transition_color = lerp_color(from_color, to_color, transition_progress);
    let alpha = ((1.0 - transition_progress) * 150.0) as u8;
    let circle_color = with_alpha(transition_color, alpha);
    
    painter.circle_filled(center, radius, circle_color);
    
    // Ripple effects
    for i in 0..3 {
        let ripple_t = (transition_progress * 2.0 - i as f32 * 0.3).clamp(0.0, 1.0);
        if ripple_t > 0.0 {
            let ripple_radius = ripple_t * max_radius * 1.2;
            let ripple_alpha = ((1.0 - ripple_t) * 80.0) as u8;
            let ripple_color = with_alpha(transition_color, ripple_alpha);
            painter.circle_stroke(center, ripple_radius, egui::Stroke::new(2.0, ripple_color));
        }
    }
}

pub fn accent_button(ui: &mut egui::Ui, text: impl Into<String>) -> egui::Response {
    let desired_size = egui::vec2(90.0, 32.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        
        // Enhanced styling with glow effect
        let base_color = if response.hovered() {
            adjust_brightness(Palette::CYAN, 1.2)
        } else {
            Palette::CYAN
        };
        
        // Add subtle glow effect
        if response.hovered() {
            let glow_config = GlowConfig::cyan_glow(0.6, 8.0);
            paint_glow_rect(painter, rect, 6.0, glow_config);
        }
        
        // Paint gradient background
        let gradient_start = adjust_brightness(base_color, 1.1);
        let gradient_end = adjust_brightness(base_color, 0.9);
        paint_gradient_rect(painter, rect, gradient_start, gradient_end, true, 6.0);
        
        // Border
        let border_color = if response.hovered() {
            adjust_brightness(Palette::CYAN, 1.3)
        } else {
            Palette::CYAN
        };
        painter.rect_stroke(rect, 6.0, egui::Stroke::new(1.5, border_color));
        
        // Text
        let text_color = egui::Color32::BLACK;
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text.into(),
            egui::FontId::proportional(14.0),
            text_color,
        );
    }
    
    response
}

pub fn secondary_button(ui: &mut egui::Ui, text: impl Into<String>) -> egui::Response {
    let desired_size = egui::vec2(90.0, 32.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        
        // Animated neon outline effect
        let border_intensity = if response.hovered() { 1.0 } else { 0.7 };
        let border_color = adjust_brightness(Palette::CYAN, border_intensity);
        
        // Subtle glow on hover
        if response.hovered() {
            let glow_config = GlowConfig::cyan_glow(0.3, 4.0);
            paint_glow_rect(painter, rect, 6.0, glow_config);
        }
        
        // Background with subtle gradient
        let bg_start = if response.hovered() {
            adjust_brightness(Palette::BG_PANEL, 1.2)
        } else {
            Palette::BG_PANEL
        };
        let bg_end = adjust_brightness(bg_start, 0.9);
        paint_gradient_rect(painter, rect, bg_start, bg_end, true, 6.0);
        
        // Animated border
        let border_width = if response.hovered() { 2.0 } else { 1.5 };
        painter.rect_stroke(rect, 6.0, egui::Stroke::new(border_width, border_color));
        
        // Text
        let text_color = if response.hovered() {
            adjust_brightness(Palette::TEXT, 1.1)
        } else {
            Palette::TEXT
        };
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text.into(),
            egui::FontId::proportional(14.0),
            text_color,
        );
    }
    
    response
}

pub fn danger_button(ui: &mut egui::Ui, text: impl Into<String>) -> egui::Response {
    let button = egui::Button::new(
        egui::RichText::new(text)
            .strong()
            .color(egui::Color32::WHITE),
    )
    .fill(Palette::MAGENTA)
    .min_size(egui::vec2(90.0, 32.0));
    ui.add(button)
}

// New cyberpunk button variants
pub fn cyberpunk_button(ui: &mut egui::Ui, text: impl Into<String>) -> egui::Response {
    let text_string = text.into();
    let desired_size = egui::vec2(120.0, 40.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        
        // Advanced glow effect
        let glow_intensity = if response.hovered() { 0.8 } else { 0.4 };
        let glow_config = GlowConfig::cyan_glow(glow_intensity, 12.0);
        paint_glow_rect(painter, rect, 8.0, glow_config);
        
        // Multi-layer gradient background
        let base_color = if response.is_pointer_button_down_on() {
            adjust_brightness(Palette::CYAN, 0.8)
        } else if response.hovered() {
            adjust_brightness(Palette::CYAN, 1.1)
        } else {
            Palette::CYAN
        };
        
        // Create a more complex gradient
        let gradient_colors = create_gradient_colors(base_color, 5);
        for (i, color) in gradient_colors.iter().enumerate() {
            let t = i as f32 / (gradient_colors.len() - 1) as f32;
            let y = rect.top() + t * rect.height();
            let step_rect = egui::Rect::from_min_size(
                egui::pos2(rect.left(), y),
                egui::vec2(rect.width(), rect.height() / gradient_colors.len() as f32 + 1.0),
            );
            painter.rect_filled(step_rect, 8.0, *color);
        }
        
        // Enhanced border with double-line effect
        painter.rect_stroke(rect, 8.0, egui::Stroke::new(2.0, adjust_brightness(base_color, 1.4)));
        painter.rect_stroke(rect.shrink(2.0), 6.0, egui::Stroke::new(1.0, adjust_brightness(base_color, 0.6)));
        
        // Text with shadow effect
        let text_color = egui::Color32::BLACK;
        let shadow_offset = egui::vec2(1.0, 1.0);
        painter.text(
            rect.center() + shadow_offset,
            egui::Align2::CENTER_CENTER,
            &text_string,
            egui::FontId::proportional(16.0),
            with_alpha(text_color, 100),
        );
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            &text_string,
            egui::FontId::proportional(16.0),
            text_color,
        );
    }
    
    response
}

pub fn neon_outline_button(ui: &mut egui::Ui, text: impl Into<String>) -> egui::Response {
    let desired_size = egui::vec2(100.0, 36.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        
        // Pulsing border animation (simulated with hover intensity)
        let pulse_intensity = if response.hovered() { 1.2 } else { 0.8 };
        let border_color = adjust_brightness(Palette::NEON_BLUE, pulse_intensity);
        
        // Subtle glow effect
        if response.hovered() {
            let glow_config = GlowConfig::blue_glow(0.5, 6.0);
            paint_glow_rect(painter, rect, 6.0, glow_config);
        }
        
        // Transparent background with subtle gradient
        let bg_alpha = if response.hovered() { 40 } else { 20 };
        let bg_start = with_alpha(Palette::NEON_BLUE, bg_alpha);
        let bg_end = with_alpha(Palette::NEON_BLUE, bg_alpha / 2);
        paint_gradient_rect(painter, rect, bg_start, bg_end, true, 6.0);
        
        // Animated border thickness
        let border_width = if response.hovered() { 2.5 } else { 2.0 };
        painter.rect_stroke(rect, 6.0, egui::Stroke::new(border_width, border_color));
        
        // Text
        let text_color = if response.hovered() {
            adjust_brightness(Palette::NEON_BLUE, 1.2)
        } else {
            Palette::NEON_BLUE
        };
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text.into(),
            egui::FontId::proportional(14.0),
            text_color,
        );
    }
    
    response
}

pub fn holographic_button(ui: &mut egui::Ui, text: impl Into<String>) -> egui::Response {
    let desired_size = egui::vec2(110.0, 38.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        
        // Shifting gradient effect (simulated with different colors based on state)
        let (color1, color2) = if response.hovered() {
            (Palette::ELECTRIC_PURPLE, Palette::CYAN)
        } else {
            (Palette::CYAN, Palette::ELECTRIC_PURPLE)
        };
        
        // Holographic glow
        if response.hovered() {
            let glow_config = GlowConfig::new(color1, 0.6, 8.0);
            paint_glow_rect(painter, rect, 7.0, glow_config);
        }
        
        // Diagonal gradient for holographic effect
        paint_gradient_rect(painter, rect, color1, color2, false, 7.0);
        
        // Overlay with transparency for holographic look
        let overlay_color = with_alpha(egui::Color32::WHITE, 30);
        painter.rect_filled(rect, 7.0, overlay_color);
        
        // Shimmering border
        let border_color = lerp_color(color1, color2, 0.5);
        painter.rect_stroke(rect, 7.0, egui::Stroke::new(1.5, border_color));
        
        // Text with holographic effect
        let text_color = egui::Color32::WHITE;
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text.into(),
            egui::FontId::proportional(15.0),
            text_color,
        );
    }
    
    response
}

pub fn danger_pulse_button(ui: &mut egui::Ui, text: impl Into<String>) -> egui::Response {
    let desired_size = egui::vec2(100.0, 36.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        
        // Enhanced warning visuals
        let base_color = Palette::MAGENTA;
        let warning_intensity = if response.hovered() { 1.3 } else { 1.0 };
        let warning_color = adjust_brightness(base_color, warning_intensity);
        
        // Pulsing glow effect
        let glow_intensity = if response.hovered() { 0.9 } else { 0.5 };
        let glow_config = GlowConfig::magenta_glow(glow_intensity, 10.0);
        paint_glow_rect(painter, rect, 6.0, glow_config);
        
        // Warning gradient background
        let gradient_start = adjust_brightness(warning_color, 1.1);
        let gradient_end = adjust_brightness(warning_color, 0.8);
        paint_gradient_rect(painter, rect, gradient_start, gradient_end, true, 6.0);
        
        // Warning border with enhanced thickness
        let border_width = if response.hovered() { 3.0 } else { 2.0 };
        painter.rect_stroke(rect, 6.0, egui::Stroke::new(border_width, adjust_brightness(warning_color, 1.2)));
        
        // Warning text
        let text_color = egui::Color32::WHITE;
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text.into(),
            egui::FontId::proportional(14.0),
            text_color,
        );
    }
    
    response
}

pub fn panel_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Palette::PANEL_GRADIENT_START)
        .stroke(egui::Stroke::new(1.5, adjust_brightness(Palette::CYAN, 1.1)))
        .rounding(8.0)
        .inner_margin(egui::Margin::symmetric(12.0, 12.0))
}

pub fn window_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Palette::BG_ACTIVE)
        .stroke(egui::Stroke::new(2.5, adjust_brightness(Palette::MAGENTA, 1.2)))
        .rounding(12.0)
        .inner_margin(egui::Margin::symmetric(16.0, 16.0))
}

// New enhanced frame variants
pub fn cyberpunk_panel_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Palette::PANEL_GRADIENT_START)
        .stroke(egui::Stroke::new(2.0, Palette::NEON_BLUE))
        .rounding(10.0)
        .inner_margin(egui::Margin::symmetric(14.0, 14.0))
}

pub fn glow_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(adjust_brightness(Palette::BG_PANEL, 1.1))
        .stroke(egui::Stroke::new(1.8, Palette::ELECTRIC_PURPLE))
        .rounding(9.0)
        .inner_margin(egui::Margin::symmetric(13.0, 13.0))
}

// Clean board background matching panels/top bar
pub fn paint_board_background(ui: &egui::Ui) {
    let rect = ui.max_rect();
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 6.0, Palette::BG_PANEL);
    painter.rect_stroke(
        rect.shrink(1.0),
        6.0,
        egui::Stroke::new(1.0, Palette::CYAN.linear_multiply(0.5)),
    );
}

// Enhanced game board cell rendering with animation support
pub fn paint_enhanced_clue_cell(
    painter: &egui::Painter,
    rect: egui::Rect,
    points: u32,
    is_solved: bool,
    is_hovered: bool,
) {
    paint_enhanced_clue_cell_with_animation(painter, rect, points, is_solved, is_hovered, 1.0)
}

pub fn paint_enhanced_clue_cell_with_animation(
    painter: &egui::Painter,
    rect: egui::Rect,
    points: u32,
    is_solved: bool,
    is_hovered: bool,
    animation_progress: f32, // 0.0 to 1.0 for transition animations
) {
    let rounding = 8.0;
    let animation_t = ease_in_out(animation_progress);
    
    // Determine cell state colors with animation support
    let (bg_start, bg_end, border_color, text_color, glow_intensity) = if is_solved {
        let solved_bg_start = adjust_brightness(Palette::BG_PANEL, 0.8);
        let solved_bg_end = adjust_brightness(Palette::BG_PANEL, 0.6);
        let solved_border = adjust_brightness(Palette::CYAN, 0.5);
        let solved_text = adjust_brightness(Palette::TEXT, 0.6);
        
        if animation_progress < 1.0 {
            // Animate transition to solved state
            let active_bg_start = adjust_brightness(Palette::BG_ACTIVE, 1.1);
            let active_bg_end = adjust_brightness(Palette::BG_ACTIVE, 0.9);
            let active_border = Palette::CYAN;
            let active_text = Palette::TEXT;
            
            (
                lerp_color(active_bg_start, solved_bg_start, animation_t),
                lerp_color(active_bg_end, solved_bg_end, animation_t),
                lerp_color(active_border, solved_border, animation_t),
                lerp_color(active_text, solved_text, animation_t),
                (1.0 - animation_t) * 0.8, // Fade out glow as it becomes solved
            )
        } else {
            (solved_bg_start, solved_bg_end, solved_border, solved_text, 0.0)
        }
    } else if is_hovered {
        let hover_intensity = 1.0 + (animation_t * 0.3); // Smooth hover animation
        (
            adjust_brightness(Palette::BG_ACTIVE, 1.3 * hover_intensity),
            adjust_brightness(Palette::BG_ACTIVE, 1.1 * hover_intensity),
            adjust_brightness(Palette::CYAN, 1.4 * hover_intensity),
            adjust_brightness(Palette::TEXT, 1.2 * hover_intensity),
            0.6 * hover_intensity,
        )
    } else {
        (
            adjust_brightness(Palette::BG_ACTIVE, 1.1),
            adjust_brightness(Palette::BG_ACTIVE, 0.9),
            Palette::CYAN,
            Palette::TEXT,
            0.2,
        )
    };
    
    // Add glow effect for interactive cells
    if !is_solved && glow_intensity > 0.0 {
        let glow_config = GlowConfig::cyan_glow(glow_intensity, 6.0);
        paint_glow_rect(painter, rect, rounding, glow_config);
    }
    
    // Paint gradient background
    paint_gradient_rect(painter, rect, bg_start, bg_end, true, rounding);
    
    // Enhanced border with different thickness based on state
    let border_width = if is_hovered && !is_solved { 3.0 } else { 2.0 };
    painter.rect_stroke(rect, rounding, egui::Stroke::new(border_width, border_color));
    
    // Add inner highlight for depth
    if !is_solved {
        let inner_rect = rect.shrink(3.0);
        let highlight_color = with_alpha(adjust_brightness(border_color, 1.5), 60);
        painter.rect_stroke(inner_rect, rounding - 2.0, egui::Stroke::new(1.0, highlight_color));
    }
    
    // Enhanced text rendering with subtle shadow
    let font_size = if is_hovered && !is_solved { 22.0 } else { 20.0 };
    let shadow_offset = egui::vec2(1.0, 1.0);
    let shadow_color = with_alpha(egui::Color32::BLACK, 100);
    
    // Draw text shadow
    painter.text(
        rect.center() + shadow_offset,
        egui::Align2::CENTER_CENTER,
        format!("{}", points),
        egui::FontId::proportional(font_size),
        shadow_color,
    );
    
    // Draw main text
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        format!("{}", points),
        egui::FontId::proportional(font_size),
        text_color,
    );
    
    // Add particle effects for completion animation
    if is_solved && animation_progress < 1.0 {
        paint_completion_particles(painter, rect, animation_progress);
    }
}

// Particle effects for successful clue completion
pub fn paint_completion_particles(
    painter: &egui::Painter,
    rect: egui::Rect,
    animation_progress: f32,
) {
    let center = rect.center();
    let particle_count = 8;
    let max_radius = rect.width().min(rect.height()) * 0.6;
    
    for i in 0..particle_count {
        let angle = (i as f32 / particle_count as f32) * 2.0 * std::f32::consts::PI;
        let progress = ease_out_bounce(animation_progress);
        let radius = progress * max_radius;
        
        let particle_pos = center + egui::Vec2::angled(angle) * radius;
        let particle_size = (1.0 - progress) * 4.0 + 1.0;
        let particle_alpha = ((1.0 - progress) * 255.0) as u8;
        let particle_color = with_alpha(Palette::CYAN, particle_alpha);
        
        painter.circle_filled(particle_pos, particle_size, particle_color);
    }
}

// Enhanced modal overlay styling
pub fn paint_enhanced_modal_background(
    painter: &egui::Painter,
    rect: egui::Rect,
) {
    // Enhanced background with subtle gradient and glow
    let bg_start = adjust_brightness(Palette::BG_DARK, 1.1);
    let bg_end = adjust_brightness(Palette::BG_DARK, 0.9);
    paint_gradient_rect(painter, rect, bg_start, bg_end, true, 0.0);
    
    // Add subtle border glow
    let border_glow = GlowConfig::cyan_glow(0.3, 20.0);
    paint_glow_rect(painter, rect.shrink(50.0), 15.0, border_glow);
    
    // Add corner accents
    let corner_size = 40.0;
    let corner_color = adjust_brightness(Palette::CYAN, 1.2);
    
    // Top-left corner
    let tl_corner = egui::Rect::from_min_size(rect.min, egui::vec2(corner_size, corner_size));
    painter.rect_stroke(tl_corner, 0.0, egui::Stroke::new(3.0, corner_color));
    
    // Top-right corner
    let tr_corner = egui::Rect::from_min_size(
        egui::pos2(rect.right() - corner_size, rect.top()),
        egui::vec2(corner_size, corner_size),
    );
    painter.rect_stroke(tr_corner, 0.0, egui::Stroke::new(3.0, corner_color));
    
    // Bottom-left corner
    let bl_corner = egui::Rect::from_min_size(
        egui::pos2(rect.left(), rect.bottom() - corner_size),
        egui::vec2(corner_size, corner_size),
    );
    painter.rect_stroke(bl_corner, 0.0, egui::Stroke::new(3.0, corner_color));
    
    // Bottom-right corner
    let br_corner = egui::Rect::from_min_size(
        egui::pos2(rect.right() - corner_size, rect.bottom() - corner_size),
        egui::vec2(corner_size, corner_size),
    );
    painter.rect_stroke(br_corner, 0.0, egui::Stroke::new(3.0, corner_color));
}

// Enhanced modal button with cyberpunk styling
pub fn enhanced_modal_button(
    ui: &mut egui::Ui,
    text: impl Into<String>,
    button_type: ModalButtonType,
) -> egui::Response {
    let text_string = text.into();
    let desired_size = egui::vec2(180.0, 50.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        
        let (bg_color, text_color, glow_color) = match button_type {
            ModalButtonType::Correct => (Palette::CYAN, egui::Color32::BLACK, Palette::GLOW_CYAN_INNER),
            ModalButtonType::Incorrect => (Palette::MAGENTA, egui::Color32::WHITE, Palette::GLOW_MAGENTA_INNER),
            ModalButtonType::Close => (Palette::NEON_BLUE, egui::Color32::WHITE, Palette::GLOW_BLUE_INNER),
        };
        
        // Enhanced glow effect
        let glow_intensity = if response.hovered() { 0.8 } else { 0.4 };
        let glow_config = GlowConfig::new(glow_color, glow_intensity, 12.0);
        paint_glow_rect(painter, rect, 10.0, glow_config);
        
        // Gradient background
        let bg_start = if response.hovered() {
            adjust_brightness(bg_color, 1.2)
        } else {
            bg_color
        };
        let bg_end = adjust_brightness(bg_start, 0.8);
        paint_gradient_rect(painter, rect, bg_start, bg_end, true, 10.0);
        
        // Enhanced border
        let border_color = adjust_brightness(bg_color, 1.3);
        painter.rect_stroke(rect, 10.0, egui::Stroke::new(2.5, border_color));
        
        // Text with enhanced styling
        let font_size = if response.hovered() { 18.0 } else { 16.0 };
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            &text_string,
            egui::FontId::proportional(font_size),
            text_color,
        );
    }
    
    response
}

#[derive(Clone, Copy)]
pub enum ModalButtonType {
    Correct,
    Incorrect,
    Close,
}

// Enhanced category header rendering
pub fn paint_enhanced_category_header(
    painter: &egui::Painter,
    rect: egui::Rect,
    category_name: &str,
) {
    let rounding = 8.0;
    
    // Gradient background for header
    let bg_start = adjust_brightness(Palette::BG_ACTIVE, 1.2);
    let bg_end = adjust_brightness(Palette::BG_ACTIVE, 0.9);
    paint_gradient_rect(painter, rect, bg_start, bg_end, true, rounding);
    
    // Subtle glow effect
    let glow_config = GlowConfig::cyan_glow(0.3, 4.0);
    paint_glow_rect(painter, rect, rounding, glow_config);
    
    // Enhanced border
    painter.rect_stroke(rect, rounding, egui::Stroke::new(2.0, adjust_brightness(Palette::CYAN, 1.1)));
    
    // Category text with enhanced styling
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        category_name,
        egui::FontId::proportional(18.0),
        adjust_brightness(Palette::CYAN, 1.2),
    );
    
    // Animated underline effect
    let underline_y = rect.bottom() - 2.0;
    let underline_start = egui::pos2(rect.left() + 4.0, underline_y);
    let underline_end = egui::pos2(rect.right() - 4.0, underline_y);
    painter.line_segment(
        [underline_start, underline_end],
        egui::Stroke::new(3.0, adjust_brightness(Palette::MAGENTA, 1.2)),
    );
}

// Gradient and color utilities
pub fn lerp_color(color1: egui::Color32, color2: egui::Color32, t: f32) -> egui::Color32 {
    let t = t.clamp(0.0, 1.0);
    let r = (color1.r() as f32 * (1.0 - t) + color2.r() as f32 * t) as u8;
    let g = (color1.g() as f32 * (1.0 - t) + color2.g() as f32 * t) as u8;
    let b = (color1.b() as f32 * (1.0 - t) + color2.b() as f32 * t) as u8;
    let a = (color1.a() as f32 * (1.0 - t) + color2.a() as f32 * t) as u8;
    egui::Color32::from_rgba_unmultiplied(r, g, b, a)
}

pub fn adjust_brightness(color: egui::Color32, factor: f32) -> egui::Color32 {
    let factor = factor.max(0.0);
    let r = ((color.r() as f32 * factor).min(255.0)) as u8;
    let g = ((color.g() as f32 * factor).min(255.0)) as u8;
    let b = ((color.b() as f32 * factor).min(255.0)) as u8;
    egui::Color32::from_rgba_unmultiplied(r, g, b, color.a())
}

pub fn with_alpha(color: egui::Color32, alpha: u8) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}

pub fn paint_gradient_rect(
    painter: &egui::Painter,
    rect: egui::Rect,
    color1: egui::Color32,
    color2: egui::Color32,
    vertical: bool,
    rounding: f32,
) {
    let steps = 32; // Number of gradient steps for smooth transition
    
    if vertical {
        let step_height = rect.height() / steps as f32;
        for i in 0..steps {
            let t = i as f32 / (steps - 1) as f32;
            let color = lerp_color(color1, color2, t);
            let y = rect.top() + i as f32 * step_height;
            let step_rect = egui::Rect::from_min_size(
                egui::pos2(rect.left(), y),
                egui::vec2(rect.width(), step_height + 1.0), // +1 to avoid gaps
            );
            painter.rect_filled(step_rect, rounding, color);
        }
    } else {
        let step_width = rect.width() / steps as f32;
        for i in 0..steps {
            let t = i as f32 / (steps - 1) as f32;
            let color = lerp_color(color1, color2, t);
            let x = rect.left() + i as f32 * step_width;
            let step_rect = egui::Rect::from_min_size(
                egui::pos2(x, rect.top()),
                egui::vec2(step_width + 1.0, rect.height()), // +1 to avoid gaps
            );
            painter.rect_filled(step_rect, rounding, color);
        }
    }
}

pub fn create_gradient_colors(base_color: egui::Color32, steps: usize) -> Vec<egui::Color32> {
    let mut colors = Vec::with_capacity(steps);
    let bright_color = adjust_brightness(base_color, 1.3);
    let dim_color = adjust_brightness(base_color, 0.7);
    
    for i in 0..steps {
        let t = i as f32 / (steps - 1) as f32;
        // Create a wave pattern for more dynamic gradients
        let wave_t = (t * std::f32::consts::PI).sin() * 0.5 + 0.5;
        colors.push(lerp_color(dim_color, bright_color, wave_t));
    }
    colors
}

// Glow rendering system
#[derive(Clone, Copy)]
pub struct GlowConfig {
    pub inner_color: egui::Color32,
    pub outer_color: egui::Color32,
    pub intensity: f32,
    pub radius: f32,
    pub layers: u8,
}

impl GlowConfig {
    pub fn new(base_color: egui::Color32, intensity: f32, radius: f32) -> Self {
        let inner_color = adjust_brightness(base_color, 1.2);
        let outer_color = with_alpha(base_color, 0);
        Self {
            inner_color,
            outer_color,
            intensity: intensity.clamp(0.0, 1.0),
            radius: radius.max(0.0),
            layers: 4,
        }
    }
    
    pub fn cyan_glow(intensity: f32, radius: f32) -> Self {
        Self {
            inner_color: Palette::GLOW_CYAN_INNER,
            outer_color: Palette::GLOW_CYAN_OUTER,
            intensity: intensity.clamp(0.0, 1.0),
            radius: radius.max(0.0),
            layers: 4,
        }
    }
    
    pub fn magenta_glow(intensity: f32, radius: f32) -> Self {
        Self {
            inner_color: Palette::GLOW_MAGENTA_INNER,
            outer_color: Palette::GLOW_MAGENTA_OUTER,
            intensity: intensity.clamp(0.0, 1.0),
            radius: radius.max(0.0),
            layers: 4,
        }
    }
    
    pub fn blue_glow(intensity: f32, radius: f32) -> Self {
        Self {
            inner_color: Palette::GLOW_BLUE_INNER,
            outer_color: Palette::GLOW_BLUE_OUTER,
            intensity: intensity.clamp(0.0, 1.0),
            radius: radius.max(0.0),
            layers: 4,
        }
    }
}

pub fn paint_glow_rect(
    painter: &egui::Painter,
    rect: egui::Rect,
    rounding: f32,
    glow_config: GlowConfig,
) {
    if glow_config.intensity <= 0.0 || glow_config.radius <= 0.0 {
        return;
    }
    
    let layers = glow_config.layers.max(1);
    let step_size = glow_config.radius / layers as f32;
    
    for i in 0..layers {
        let layer_progress = i as f32 / (layers - 1) as f32;
        let expansion = step_size * (i + 1) as f32;
        let alpha_factor = (1.0 - layer_progress) * glow_config.intensity;
        
        let layer_color = lerp_color(
            glow_config.inner_color,
            glow_config.outer_color,
            layer_progress,
        );
        
        let final_color = with_alpha(
            layer_color,
            (layer_color.a() as f32 * alpha_factor) as u8,
        );
        
        let expanded_rect = rect.expand(expansion);
        painter.rect_filled(expanded_rect, rounding + expansion * 0.5, final_color);
    }
}

pub fn paint_glow_circle(
    painter: &egui::Painter,
    center: egui::Pos2,
    radius: f32,
    glow_config: GlowConfig,
) {
    if glow_config.intensity <= 0.0 || glow_config.radius <= 0.0 {
        return;
    }
    
    let layers = glow_config.layers.max(1);
    let step_size = glow_config.radius / layers as f32;
    
    for i in 0..layers {
        let layer_progress = i as f32 / (layers - 1) as f32;
        let layer_radius = radius + step_size * (i + 1) as f32;
        let alpha_factor = (1.0 - layer_progress) * glow_config.intensity;
        
        let layer_color = lerp_color(
            glow_config.inner_color,
            glow_config.outer_color,
            layer_progress,
        );
        
        let final_color = with_alpha(
            layer_color,
            (layer_color.a() as f32 * alpha_factor) as u8,
        );
        
        painter.circle_filled(center, layer_radius, final_color);
    }
}

pub fn calculate_glow_radius_for_rect(rect: egui::Rect, base_radius: f32) -> f32 {
    let size_factor = (rect.width().min(rect.height()) / 100.0).clamp(0.5, 2.0);
    base_radius * size_factor
}

// Animation state management system
#[derive(Clone, Copy, Debug)]
pub enum EasingType {
    Linear,
    EaseInOut,
    EaseOutBounce,
    EaseInElastic,
}

#[derive(Clone, Debug)]
pub struct AnimationState {
    pub start_time: Instant,
    pub duration: Duration,
    pub easing_function: EasingType,
    pub current_value: f32,
}

impl AnimationState {
    pub fn new(duration: Duration, easing_function: EasingType) -> Self {
        Self {
            start_time: Instant::now(),
            duration,
            easing_function,
            current_value: 0.0,
        }
    }
    
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
    
    pub fn is_finished(&self) -> bool {
        self.start_time.elapsed() >= self.duration
    }
    
    pub fn restart(&mut self) {
        self.start_time = Instant::now();
        self.current_value = 0.0;
    }
}

// Easing functions
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

// Animation controller for managing multiple animations
#[derive(Default)]
pub struct AnimationController {
    animations: std::collections::HashMap<String, AnimationState>,
}

impl AnimationController {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn start_animation(&mut self, name: String, duration: Duration, easing: EasingType) {
        self.animations.insert(name, AnimationState::new(duration, easing));
    }
    
    pub fn update_animation(&mut self, name: &str) -> Option<f32> {
        if let Some(animation) = self.animations.get_mut(name) {
            Some(animation.update())
        } else {
            None
        }
    }
    
    pub fn is_animation_finished(&self, name: &str) -> bool {
        self.animations.get(name).map_or(true, |a| a.is_finished())
    }
    
    pub fn remove_finished_animations(&mut self) {
        self.animations.retain(|_, animation| !animation.is_finished());
    }
    
    pub fn restart_animation(&mut self, name: &str) {
        if let Some(animation) = self.animations.get_mut(name) {
            animation.restart();
        }
    }
}
