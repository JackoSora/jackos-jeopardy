use eframe::egui;

pub struct Palette;
impl Palette {
    pub const CYAN: egui::Color32 = egui::Color32::from_rgb(0, 255, 170);
    pub const MAGENTA: egui::Color32 = egui::Color32::from_rgb(255, 0, 150);
    pub const BG_DARK: egui::Color32 = egui::Color32::from_rgb(10, 10, 18);
    pub const BG_PANEL: egui::Color32 = egui::Color32::from_rgb(20, 20, 36);
    pub const BG_ACTIVE: egui::Color32 = egui::Color32::from_rgb(36, 0, 58);
    pub const TEXT: egui::Color32 = egui::Color32::from_rgb(208, 255, 247);
}

pub fn apply_global_style(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.override_text_color = Some(Palette::TEXT);
    visuals.window_rounding = 8.0.into();
    visuals.widgets.noninteractive.bg_fill = Palette::BG_DARK;
    visuals.widgets.inactive.bg_fill = Palette::BG_PANEL;
    visuals.widgets.active.bg_fill = Palette::BG_ACTIVE;
    visuals.widgets.hovered.bg_fill = Palette::BG_ACTIVE.linear_multiply(1.15);
    visuals.selection.bg_fill = Palette::CYAN;
    visuals.extreme_bg_color = Palette::BG_DARK;
    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    style.spacing.button_padding = egui::vec2(14.0, 10.0);
    ctx.set_style(style);
}

pub fn accent_button(ui: &mut egui::Ui, text: impl Into<String>) -> egui::Response {
    let button = egui::Button::new(
        egui::RichText::new(text)
            .strong()
            .color(egui::Color32::BLACK),
    )
    .fill(Palette::CYAN)
    .min_size(egui::vec2(90.0, 32.0));
    ui.add(button)
}

pub fn secondary_button(ui: &mut egui::Ui, text: impl Into<String>) -> egui::Response {
    let button = egui::Button::new(egui::RichText::new(text).strong().color(Palette::TEXT))
        .fill(Palette::BG_PANEL)
        .stroke(egui::Stroke::new(1.5, Palette::CYAN))
        .min_size(egui::vec2(90.0, 32.0));
    ui.add(button)
}

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

pub fn panel_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Palette::BG_PANEL)
        .stroke(egui::Stroke::new(1.0, Palette::CYAN))
        .rounding(6.0)
        .inner_margin(egui::Margin::symmetric(10.0, 10.0))
}

pub fn window_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Palette::BG_ACTIVE)
        .stroke(egui::Stroke::new(2.0, Palette::MAGENTA))
        .rounding(10.0)
        .inner_margin(egui::Margin::symmetric(14.0, 14.0))
}

// Clean board background matching panels/top bar
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
