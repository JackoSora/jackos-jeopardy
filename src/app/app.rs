use eframe::egui;

use crate::app::config_ui;
use crate::app::game_ui;
use crate::core::storage::{self, Snapshot};
use crate::core::{Board, ConfigState};
use crate::game::GameEngine;
use crate::theme::{self, Palette};

#[derive(Debug)]
pub enum AppMode {
    Config(ConfigState),
    Game(GameEngine),
}

pub struct PartyJeopardyApp {
    mode: AppMode,
    // UI state
    show_save_dialog: bool,
    show_load_dialog: bool,
    save_name: String,
}

impl PartyJeopardyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        theme::apply_global_style(&_cc.egui_ctx);
        let default_board = Board::default_with_dimensions(6, 5);
        let config = ConfigState {
            board: default_board,
        };
        Self {
            mode: AppMode::Config(config),
            show_save_dialog: false,
            show_load_dialog: false,
            save_name: String::new(),
        }
    }
}

impl eframe::App for PartyJeopardyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar")
            .frame(
                egui::Frame::none()
                    .fill(Palette::BG_DARK)
                    .inner_margin(egui::Margin::symmetric(12.0, 8.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Jacko's Jeopardy!")
                            .color(Palette::CYAN)
                            .size(22.0)
                            .strong(),
                    );
                    ui.add_space(8.0);
                    ui.colored_label(Palette::MAGENTA, "::");
                    ui.add_space(8.0);
                    if theme::accent_button(ui, "Save").clicked() {
                        self.show_save_dialog = true;
                    }
                    let in_config = matches!(self.mode, AppMode::Config(_));
                    if in_config {
                        if theme::secondary_button(ui, "Load").clicked() {
                            self.show_load_dialog = true;
                        }
                    } else {
                        let resp = theme::secondary_button(ui, "Load");
                        resp.widget_info(|| {
                            egui::WidgetInfo::labeled(egui::WidgetType::Button, "Load (disabled)")
                        });
                        // visually dim by overlay
                        if resp.hovered() { /* ignore */ }
                    }
                });
            });

        // Save dialog window
        if self.show_save_dialog {
            let mut open = true;
            egui::Window::new("Save Snapshot")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .frame(theme::window_frame())
                .show(ctx, |ui| {
                    ui.set_min_width(320.0);
                    ui.label(
                        egui::RichText::new("Enter a name for the save file").color(Palette::CYAN),
                    );
                    ui.text_edit_singleline(&mut self.save_name);
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        if theme::accent_button(ui, "Save").clicked() {
                            let snapshot = match &self.mode {
                                AppMode::Config(cfg) => Snapshot {
                                    board: cfg.board.clone(),
                                    game: None,
                                },
                                AppMode::Game(game_engine) => Snapshot {
                                    board: game_engine.get_state().board.clone(),
                                    game: Some(game_engine.get_state().clone()),
                                },
                            };
                            if let Ok(path) =
                                storage::save_snapshot_named(&self.save_name, &snapshot)
                            {
                                self.show_save_dialog = false;
                                self.save_name.clear();
                                ui.output_mut(|o| {
                                    o.copied_text = format!("Saved: {}", path.display())
                                });
                            }
                        }
                        if theme::secondary_button(ui, "Cancel").clicked() {
                            self.show_save_dialog = false;
                        }
                    });
                });
            self.show_save_dialog = open && self.show_save_dialog; // respect close button
        }

        // Load dialog window
        if self.show_load_dialog {
            let mut open = true;
            egui::Window::new("Load Snapshot")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .frame(theme::window_frame())
                .show(ctx, |ui| {
                    ui.set_min_width(340.0);
                    match storage::list_saves() {
                        Ok(files) => {
                            if files.is_empty() {
                                ui.label(
                                    egui::RichText::new("No saves found.").color(Palette::MAGENTA),
                                );
                            } else {
                                ui.label(
                                    egui::RichText::new("Select a save to load:")
                                        .color(Palette::CYAN),
                                );
                            }
                            for path in files {
                                let label =
                                    path.file_stem().and_then(|s| s.to_str()).unwrap_or("?");
                                if theme::secondary_button(ui, label).clicked() {
                                    if let Ok(snapshot) = storage::load_snapshot_from_path(&path) {
                                        match snapshot.game {
                                            Some(game_state) => {
                                                let mut game_engine =
                                                    GameEngine::new(game_state.board.clone());
                                                *game_engine.get_state_mut() = game_state;
                                                self.mode = AppMode::Game(game_engine);
                                            }
                                            None => {
                                                self.mode = AppMode::Config(ConfigState {
                                                    board: snapshot.board,
                                                })
                                            }
                                        }
                                        self.show_load_dialog = false;
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            ui.colored_label(
                                egui::Color32::RED,
                                format!("Error listing saves: {}", err),
                            );
                        }
                    }
                    if theme::accent_button(ui, "Close").clicked() {
                        self.show_load_dialog = false;
                    }
                });
            self.show_load_dialog = open && self.show_load_dialog;
        }

        match &mut self.mode {
            AppMode::Config(config_state) => {
                if let Some(new_game_engine) = config_ui::show(ctx, config_state) {
                    self.mode = AppMode::Game(new_game_engine);
                }
            }
            AppMode::Game(game_engine) => {
                if let Some(next_mode) = game_ui::show(ctx, game_engine) {
                    self.mode = next_mode;
                }
            }
        }
    }
}
