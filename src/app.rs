use eframe::egui;

use crate::config_ui;
use crate::domain::{Board, ConfigState};
use crate::game::GameState;
use crate::game_ui;

#[derive(Debug)]
pub enum AppMode {
    Config(ConfigState),
    Game(GameState),
}

pub struct PartyJeopardyApp {
    mode: AppMode,
}

impl PartyJeopardyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Cyberpunk-ish theme
        let mut visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(egui::Color32::from_rgb(0xD0, 0xFF, 0xF7));
        visuals.window_rounding = 8.0.into();
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(10, 10, 18);
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(20, 20, 36);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(36, 0, 58);
        visuals.selection.bg_fill = egui::Color32::from_rgb(0, 255, 170);
        _cc.egui_ctx.set_visuals(visuals);

        let mut style = (*_cc.egui_ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(10.0, 10.0);
        style.spacing.button_padding = egui::vec2(10.0, 8.0);
        _cc.egui_ctx.set_style(style);
        let default_board = Board::default_with_dimensions(6, 5);
        let config = ConfigState {
            board: default_board,
        };
        Self {
            mode: AppMode::Config(config),
        }
    }
}

impl eframe::App for PartyJeopardyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.label("Party Jeopardy!");
        });

        match &mut self.mode {
            AppMode::Config(config_state) => {
                if let Some(new_game) = config_ui::show(ctx, config_state) {
                    self.mode = AppMode::Game(new_game);
                }
            }
            AppMode::Game(game_state) => {
                if let Some(next_mode) = game_ui::show(ctx, game_state) {
                    self.mode = next_mode;
                }
            }
        }
    }
}
