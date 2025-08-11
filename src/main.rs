mod app;
mod core;
mod game;
mod theme;
mod ui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([900.0, 600.0])
            .with_title("Jacko's Jeopardy"),
        ..Default::default()
    };
    eframe::run_native(
        "LNS with Jay",
        options,
        Box::new(|cc| Box::new(app::PartyJeopardyApp::new(cc))),
    )
}
