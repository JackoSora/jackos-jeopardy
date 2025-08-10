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
    
    // Background gradient colors (used in theme)
    pub const PANEL_GRADIENT_START: egui::Color32 = egui::Color32::from_rgb(25, 25, 45);
    
    // Glow colors (used in buttons)
    pub const GLOW_CYAN_INNER: egui::Color32 = egui::Color32::from_rgb(100, 255, 200);
    pub const GLOW_CYAN_OUTER: egui::Color32 = egui::Color32::TRANSPARENT;
    pub const GLOW_MAGENTA_INNER: egui::Color32 = egui::Color32::from_rgb(255, 100, 200);
    pub const GLOW_BLUE_INNER: egui::Color32 = egui::Color32::from_rgb(100, 200, 255);
}

