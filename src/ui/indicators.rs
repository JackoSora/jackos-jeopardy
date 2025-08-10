// Game state indicators and visual feedback components
use eframe::egui;
use crate::theme::{
    colors::Palette,
    effects::{GlowConfig, paint_glow_rect, paint_gradient_rect},
    utils::{adjust_brightness, with_alpha, lerp_color},
    animations::ease_in_out,
};

/// Enhanced visual indicators for active team
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

/// Enhanced game phase indicator
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

/// Enhanced transition effects between game phases
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