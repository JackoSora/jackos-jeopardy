// Utility functions for color manipulation and common operations
use eframe::egui;

// Re-add with_alpha since it's used
pub fn with_alpha(color: egui::Color32, alpha: u8) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}

// Re-add create_gradient_colors since it's used
pub fn create_gradient_colors(base_color: egui::Color32, steps: usize) -> Vec<egui::Color32> {
    let mut colors = Vec::with_capacity(steps);
    let bright_color = adjust_brightness(base_color, 1.3);
    let dim_color = adjust_brightness(base_color, 0.7);

    for i in 0..steps {
        let t = i as f32 / (steps - 1) as f32;
        let wave_t = (t * std::f32::consts::PI).sin() * 0.5 + 0.5;
        colors.push(lerp_color(dim_color, bright_color, wave_t));
    }
    colors
}

/// Linear interpolation between two colors
pub fn lerp_color(color1: egui::Color32, color2: egui::Color32, t: f32) -> egui::Color32 {
    let t = t.clamp(0.0, 1.0);
    let r = (color1.r() as f32 * (1.0 - t) + color2.r() as f32 * t) as u8;
    let g = (color1.g() as f32 * (1.0 - t) + color2.g() as f32 * t) as u8;
    let b = (color1.b() as f32 * (1.0 - t) + color2.b() as f32 * t) as u8;
    let a = (color1.a() as f32 * (1.0 - t) + color2.a() as f32 * t) as u8;
    egui::Color32::from_rgba_unmultiplied(r, g, b, a)
}

/// Adjust the brightness of a color by a factor
pub fn adjust_brightness(color: egui::Color32, factor: f32) -> egui::Color32 {
    let factor = factor.max(0.0);
    let r = ((color.r() as f32 * factor).min(255.0)) as u8;
    let g = ((color.g() as f32 * factor).min(255.0)) as u8;
    let b = ((color.b() as f32 * factor).min(255.0)) as u8;
    egui::Color32::from_rgba_unmultiplied(r, g, b, color.a())
}

/// Calculate appropriate text color for a given background
pub fn contrasting_text_color(background: egui::Color32) -> egui::Color32 {
    // Calculate luminance
    let r = background.r() as f32 / 255.0;
    let g = background.g() as f32 / 255.0;
    let b = background.b() as f32 / 255.0;

    let luminance = 0.299 * r + 0.587 * g + 0.114 * b;

    if luminance > 0.5 {
        egui::Color32::BLACK
    } else {
        egui::Color32::WHITE
    }
}
