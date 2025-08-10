// Game board rendering components
use eframe::egui;
use crate::theme::{
    colors::Palette,
    effects::{GlowConfig, paint_glow_rect, paint_gradient_rect, paint_completion_particles},
    utils::{adjust_brightness, with_alpha, lerp_color},
    animations::ease_in_out,
};

/// Enhanced game board cell rendering with animation support
pub fn paint_enhanced_clue_cell(
    painter: &egui::Painter,
    rect: egui::Rect,
    points: u32,
    is_solved: bool,
    is_hovered: bool,
) {
    paint_enhanced_clue_cell_with_animation(painter, rect, points, is_solved, is_hovered, 1.0)
}

/// Enhanced game board cell rendering with animation support
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

/// Enhanced category header rendering
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