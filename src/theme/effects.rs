// Visual effects like glows, gradients, and particles
use crate::theme::{
    colors::Palette,
    utils::{adjust_brightness, lerp_color, with_alpha},
};
use eframe::egui;

/// Configuration for glow effects
#[derive(Clone, Copy)]
pub struct GlowConfig {
    pub inner_color: egui::Color32,
    pub outer_color: egui::Color32,
    pub intensity: f32,
    pub radius: f32,
    pub layers: u8,
}

impl GlowConfig {
    /// Create a new glow configuration
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

    /// Create a cyan glow configuration
    pub fn cyan_glow(intensity: f32, radius: f32) -> Self {
        Self {
            inner_color: Palette::GLOW_CYAN_INNER,
            outer_color: Palette::GLOW_CYAN_OUTER,
            intensity: intensity.clamp(0.0, 1.0),
            radius: radius.max(0.0),
            layers: 4,
        }
    }
}

/// Paint a rectangle with glow effect
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

        let final_color = with_alpha(layer_color, (layer_color.a() as f32 * alpha_factor) as u8);

        let expanded_rect = rect.expand(expansion);
        painter.rect_filled(expanded_rect, rounding + expansion * 0.5, final_color);
    }
}

/// Paint a gradient rectangle
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

/// Paint particle effects for completion animations
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
        let progress = crate::theme::animations::ease_out_bounce(animation_progress);
        let radius = progress * max_radius;

        let particle_pos = center + egui::Vec2::angled(angle) * radius;
        let particle_size = (1.0 - progress) * 4.0 + 1.0;
        let particle_alpha = ((1.0 - progress) * 255.0) as u8;
        let particle_color = with_alpha(Palette::CYAN, particle_alpha);

        painter.circle_filled(particle_pos, particle_size, particle_color);
    }
}
