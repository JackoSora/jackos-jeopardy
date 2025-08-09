mod app;
mod config_ui;
mod domain;
mod game;
mod game_ui;
mod storage;
mod theme;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([900.0, 600.0])
            .with_title("Party Jeopardy!"),
        ..Default::default()
    };
    eframe::run_native(
        "Party Jeopardy!",
        options,
        Box::new(|cc| Box::new(app::PartyJeopardyApp::new(cc))),
    )
}
