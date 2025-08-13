// Button components with cyberpunk styling
use crate::theme::{
    colors::Palette,
    effects::{GlowConfig, paint_glow_rect, paint_gradient_rect},
    utils::adjust_brightness,
};
use eframe::egui;

/// Enhanced accent button with cyberpunk styling
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

/// Enhanced secondary button with neon outline
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

/// Enhanced danger button with warning effects
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

/// Button type for modal dialogs
#[derive(Clone, Copy)]
pub enum ModalButtonType {
    Correct,
    Incorrect,
    Close,
}

/// Enhanced modal button with cyberpunk styling
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
            ModalButtonType::Correct => (
                Palette::CYAN,
                egui::Color32::BLACK,
                Palette::GLOW_CYAN_INNER,
            ),
            ModalButtonType::Incorrect => (
                Palette::MAGENTA,
                egui::Color32::WHITE,
                Palette::GLOW_MAGENTA_INNER,
            ),
            ModalButtonType::Close => (
                Palette::NEON_BLUE,
                egui::Color32::WHITE,
                Palette::GLOW_BLUE_INNER,
            ),
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
