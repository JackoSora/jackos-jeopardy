// Utility functions for color manipulation and common operations
use eframe::egui;

// Keep only the utilities actually used by the codebase.

/// Linear interpolation between two colors
pub fn lerp_color(color1: egui::Color32, color2: egui::Color32, t: f32) -> egui::Color32 {
    let t = t.clamp(0.0, 1.0);
    let r = (color1.r() as f32 * (1.0 - t) + color2.r() as f32 * t) as u8;
    let g = (color1.g() as f32 * (1.0 - t) + color2.g() as f32 * t) as u8;
    let b = (color1.b() as f32 * (1.0 - t) + color2.b() as f32 * t) as u8;
    let a = (color1.a() as f32 * (1.0 - t) + color2.a() as f32 * t) as u8;
    egui::Color32::from_rgba_unmultiplied(r, g, b, a)
}

/// Replace alpha channel of a color
pub fn with_alpha(color: egui::Color32, alpha: u8) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}

/// Adjust the brightness of a color by a factor
pub fn adjust_brightness(color: egui::Color32, factor: f32) -> egui::Color32 {
    let factor = factor.max(0.0);
    let r = ((color.r() as f32 * factor).min(255.0)) as u8;
    let g = ((color.g() as f32 * factor).min(255.0)) as u8;
    let b = ((color.b() as f32 * factor).min(255.0)) as u8;
    egui::Color32::from_rgba_unmultiplied(r, g, b, color.a())
}
