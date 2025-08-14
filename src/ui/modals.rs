// Modal dialog components
use crate::theme::{
    colors::Palette,
    effects::{GlowConfig, paint_glow_rect, paint_gradient_rect},
    utils::adjust_brightness,
};
use eframe::egui;

pub fn paint_enhanced_modal_background(painter: &egui::Painter, rect: egui::Rect) {
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

pub fn paint_subtle_modal_background(painter: &egui::Painter, rect: egui::Rect) {
    // Muted background with subtle teal/dark gradient
    let bg_start = adjust_brightness(Palette::BG_DARK, 1.05);
    let bg_end = adjust_brightness(Palette::SUBTLE_TEAL, 0.3);
    paint_gradient_rect(painter, rect, bg_start, bg_end, true, 0.0);

    // Add very subtle border glow with reduced intensity
    let border_glow = GlowConfig::new(Palette::SUBTLE_TEAL, 0.12, 15.0);
    paint_glow_rect(painter, rect.shrink(50.0), 12.0, border_glow);
}

// Removed paint_subtle_modal_background as it was unused.