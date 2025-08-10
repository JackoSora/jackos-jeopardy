// Theme module - Main entry point for all theming functionality
pub mod animations;
pub mod buttons;
pub mod colors;
pub mod effects;
pub mod frames;
pub mod performance;
pub mod utils;

// Re-export commonly used items for convenience
pub use animations::{AnimationController, AnimationState, EasingType};
pub use buttons::{
    ModalButtonType, accent_button, danger_button, enhanced_modal_button, secondary_button,
};
pub use colors::Palette;
pub use effects::{GlowConfig, paint_glow_rect, paint_gradient_rect};
pub use frames::{panel_frame, window_frame};
pub use performance::{PerformanceMonitor, PerformanceSettings, VisualQuality};
pub use utils::{adjust_brightness, lerp_color, with_alpha};

use eframe::egui;

/// Apply the global cyberpunk theme to the egui context
pub fn apply_global_style(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    // Enhanced text styling
    visuals.override_text_color = Some(utils::adjust_brightness(Palette::TEXT, 1.05));

    // Enhanced window and panel styling
    visuals.window_rounding = 12.0.into();
    visuals.panel_fill = Palette::PANEL_GRADIENT_START;
    visuals.window_fill = utils::adjust_brightness(Palette::BG_ACTIVE, 1.1);

    // Enhanced widget styling
    visuals.widgets.noninteractive.bg_fill = Palette::BG_DARK;
    visuals.widgets.noninteractive.fg_stroke.color = utils::adjust_brightness(Palette::TEXT, 0.9);

    visuals.widgets.inactive.bg_fill = utils::adjust_brightness(Palette::BG_PANEL, 1.05);
    visuals.widgets.inactive.fg_stroke.color = Palette::TEXT;
    visuals.widgets.inactive.bg_stroke.color = utils::adjust_brightness(Palette::CYAN, 0.7);
    visuals.widgets.inactive.bg_stroke.width = 1.0;

    visuals.widgets.active.bg_fill = utils::adjust_brightness(Palette::BG_ACTIVE, 1.2);
    visuals.widgets.active.fg_stroke.color = utils::adjust_brightness(Palette::TEXT, 1.1);
    visuals.widgets.active.bg_stroke.color = utils::adjust_brightness(Palette::CYAN, 1.2);
    visuals.widgets.active.bg_stroke.width = 2.0;

    visuals.widgets.hovered.bg_fill = utils::adjust_brightness(Palette::BG_ACTIVE, 1.3);
    visuals.widgets.hovered.fg_stroke.color = utils::adjust_brightness(Palette::TEXT, 1.15);
    visuals.widgets.hovered.bg_stroke.color = utils::adjust_brightness(Palette::CYAN, 1.3);
    visuals.widgets.hovered.bg_stroke.width = 2.5;

    // Enhanced selection and focus indicators
    visuals.selection.bg_fill = utils::adjust_brightness(Palette::CYAN, 1.1);
    visuals.selection.stroke.color = utils::adjust_brightness(Palette::CYAN, 1.4);
    visuals.selection.stroke.width = 2.0;

    // Enhanced background colors
    visuals.extreme_bg_color = Palette::BG_DARK;
    visuals.faint_bg_color = utils::adjust_brightness(Palette::BG_PANEL, 0.8);

    // Enhanced hyperlink styling
    visuals.hyperlink_color = utils::adjust_brightness(Palette::NEON_BLUE, 1.2);

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
    style
        .text_styles
        .insert(egui::TextStyle::Heading, egui::FontId::proportional(28.0));
    style
        .text_styles
        .insert(egui::TextStyle::Body, egui::FontId::proportional(16.0));
    style
        .text_styles
        .insert(egui::TextStyle::Button, egui::FontId::proportional(16.0));
    style
        .text_styles
        .insert(egui::TextStyle::Small, egui::FontId::proportional(12.0));

    ctx.set_style(style);
}

/// Paint the board background with cyberpunk styling
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
