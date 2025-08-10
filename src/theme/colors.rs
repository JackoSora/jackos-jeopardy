// Color definitions and palette for the cyberpunk theme
use eframe::egui;

/// Cyberpunk color palette with all theme colors
pub struct Palette;

impl Palette {
    // Core cyberpunk colors (maintained for compatibility)
    pub const CYAN: egui::Color32 = egui::Color32::from_rgb(0, 255, 170);
    pub const MAGENTA: egui::Color32 = egui::Color32::from_rgb(255, 0, 150);
    pub const BG_DARK: egui::Color32 = egui::Color32::from_rgb(10, 10, 18);
    pub const BG_PANEL: egui::Color32 = egui::Color32::from_rgb(20, 20, 36);
    pub const BG_ACTIVE: egui::Color32 = egui::Color32::from_rgb(36, 0, 58);
    pub const TEXT: egui::Color32 = egui::Color32::from_rgb(208, 255, 247);

    // Extended cyberpunk colors
    pub const NEON_BLUE: egui::Color32 = egui::Color32::from_rgb(0, 150, 255);
    pub const ELECTRIC_PURPLE: egui::Color32 = egui::Color32::from_rgb(150, 0, 255);
    pub const CYBER_ORANGE: egui::Color32 = egui::Color32::from_rgb(255, 100, 0);
    pub const NEON_GREEN: egui::Color32 = egui::Color32::from_rgb(57, 255, 20);
    pub const ELECTRIC_PINK: egui::Color32 = egui::Color32::from_rgb(255, 20, 147);

    // Glow effect colors
    pub const GLOW_CYAN_INNER: egui::Color32 = egui::Color32::from_rgb(100, 255, 200);
    pub const GLOW_CYAN_OUTER: egui::Color32 = egui::Color32::TRANSPARENT;
    pub const GLOW_MAGENTA_INNER: egui::Color32 = egui::Color32::from_rgb(255, 100, 200);
    pub const GLOW_MAGENTA_OUTER: egui::Color32 = egui::Color32::TRANSPARENT;
    pub const GLOW_BLUE_INNER: egui::Color32 = egui::Color32::from_rgb(100, 200, 255);
    pub const GLOW_BLUE_OUTER: egui::Color32 = egui::Color32::TRANSPARENT;

    // Background gradient colors
    pub const BG_GRADIENT_START: egui::Color32 = egui::Color32::from_rgb(15, 15, 25);
    pub const BG_GRADIENT_END: egui::Color32 = egui::Color32::from_rgb(25, 10, 35);
    pub const PANEL_GRADIENT_START: egui::Color32 = egui::Color32::from_rgb(25, 25, 45);
    pub const PANEL_GRADIENT_END: egui::Color32 = egui::Color32::from_rgb(35, 15, 55);
}

/// Predefined color schemes for different UI contexts
pub enum ColorScheme {
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
}

impl ColorScheme {
    /// Get the main color for this scheme
    pub fn main_color(&self) -> egui::Color32 {
        match self {
            ColorScheme::Primary => Palette::CYAN,
            ColorScheme::Secondary => Palette::NEON_BLUE,
            ColorScheme::Success => Palette::NEON_GREEN,
            ColorScheme::Warning => Palette::CYBER_ORANGE,
            ColorScheme::Danger => Palette::MAGENTA,
        }
    }

    /// Get the accent color for this scheme
    pub fn accent_color(&self) -> egui::Color32 {
        match self {
            ColorScheme::Primary => Palette::ELECTRIC_PURPLE,
            ColorScheme::Secondary => Palette::ELECTRIC_PURPLE,
            ColorScheme::Success => Palette::CYAN,
            ColorScheme::Warning => Palette::ELECTRIC_PINK,
            ColorScheme::Danger => Palette::ELECTRIC_PINK,
        }
    }

    /// Get the glow colors for this scheme
    pub fn glow_colors(&self) -> (egui::Color32, egui::Color32) {
        match self {
            ColorScheme::Primary => (Palette::GLOW_CYAN_INNER, Palette::GLOW_CYAN_OUTER),
            ColorScheme::Secondary => (Palette::GLOW_BLUE_INNER, Palette::GLOW_BLUE_OUTER),
            ColorScheme::Success => (Palette::GLOW_CYAN_INNER, Palette::GLOW_CYAN_OUTER),
            ColorScheme::Warning => (Palette::CYBER_ORANGE, egui::Color32::TRANSPARENT),
            ColorScheme::Danger => (Palette::GLOW_MAGENTA_INNER, Palette::GLOW_MAGENTA_OUTER),
        }
    }
}
