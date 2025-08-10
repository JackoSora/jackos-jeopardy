// Frame and panel components with cyberpunk styling
use eframe::egui;
use crate::theme::{colors::Palette, utils::adjust_brightness};

/// Enhanced panel frame with cyberpunk styling
pub fn panel_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Palette::PANEL_GRADIENT_START)
        .stroke(egui::Stroke::new(1.5, adjust_brightness(Palette::CYAN, 1.1)))
        .rounding(8.0)
        .inner_margin(egui::Margin::symmetric(12.0, 12.0))
}

/// Enhanced window frame with cyberpunk styling
pub fn window_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Palette::BG_ACTIVE)
        .stroke(egui::Stroke::new(2.5, adjust_brightness(Palette::MAGENTA, 1.2)))
        .rounding(12.0)
        .inner_margin(egui::Margin::symmetric(16.0, 16.0))
}

/// Advanced cyberpunk panel frame
pub fn cyberpunk_panel_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Palette::PANEL_GRADIENT_START)
        .stroke(egui::Stroke::new(2.0, Palette::NEON_BLUE))
        .rounding(10.0)
        .inner_margin(egui::Margin::symmetric(14.0, 14.0))
}

/// Glowing frame variant
pub fn glow_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(adjust_brightness(Palette::BG_PANEL, 1.1))
        .stroke(egui::Stroke::new(1.8, Palette::ELECTRIC_PURPLE))
        .rounding(9.0)
        .inner_margin(egui::Margin::symmetric(13.0, 13.0))
}