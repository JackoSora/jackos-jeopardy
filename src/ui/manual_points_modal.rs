use crate::core::Team;
use eframe::egui;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct ManualPointsModal {
    pub visible: bool,
    pub team_inputs: HashMap<u32, String>,
    pub validation_errors: HashMap<u32, String>,
    pub pending_changes: Vec<(u32, i32)>,
}

impl ManualPointsModal {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
        self.clear_state();
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn initialize_inputs(&mut self, teams: &[Team]) {
        self.team_inputs.clear();
        self.validation_errors.clear();
        self.pending_changes.clear();

        for team in teams {
            self.team_inputs.insert(team.id, team.score.to_string());
        }
    }

    fn clear_state(&mut self) {
        self.team_inputs.clear();
        self.validation_errors.clear();
        self.pending_changes.clear();
    }

    fn validate_input(&mut self, team_id: u32, input: &str) -> bool {
        self.validation_errors.remove(&team_id);

        if input.trim().is_empty() {
            self.validation_errors
                .insert(team_id, "Points cannot be empty".to_string());
            return false;
        }

        match input.trim().parse::<i32>() {
            Ok(points) => {
                // Range validation - allow reasonable point values
                if points < -999999 || points > 999999 {
                    self.validation_errors.insert(
                        team_id,
                        "Points must be between -999,999 and 999,999".to_string(),
                    );
                    false
                } else {
                    true
                }
            }
            Err(_) => {
                self.validation_errors
                    .insert(team_id, "Please enter a valid number".to_string());
                false
            }
        }
    }

    fn prepare_changes(&mut self, teams: &[Team]) -> bool {
        self.pending_changes.clear();
        let mut all_valid = true;

        for team in teams {
            if let Some(input) = self.team_inputs.get(&team.id).cloned() {
                if self.validate_input(team.id, &input) {
                    if let Ok(new_points) = input.trim().parse::<i32>() {
                        if new_points != team.score {
                            self.pending_changes.push((team.id, new_points));
                        }
                    }
                } else {
                    all_valid = false;
                }
            }
        }

        all_valid
    }
}

/// Show the manual points adjustment modal and return pending changes if confirmed
pub fn show_manual_points_modal(
    ctx: &egui::Context,
    modal: &mut ManualPointsModal,
    teams: &[Team],
) -> Option<Vec<(u32, i32)>> {
    if !modal.is_visible() {
        return None;
    }

    let mut result = None;
    let screen_rect = ctx.screen_rect();

    egui::Area::new("manual_points_modal".into())
        .order(egui::Order::Foreground)
        .movable(false)
        .interactable(true)
        .fixed_pos(screen_rect.min)
        .show(ctx, |ui| {
            let rect = screen_rect;
            let painter = ui.painter_at(rect);

            // Modal background
            crate::ui::paint_subtle_modal_background(&painter, rect);

            // Modal content area
            let modal_width = 500.0;
            let modal_height = 400.0;
            let modal_rect =
                egui::Rect::from_center_size(rect.center(), egui::vec2(modal_width, modal_height));

            ui.allocate_ui_at_rect(modal_rect, |ui| {
                egui::Frame::none()
                    .fill(crate::theme::Palette::BG_DARK)
                    .stroke(egui::Stroke::new(2.0, crate::theme::Palette::CYAN))
                    .rounding(8.0)
                    .inner_margin(20.0)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            // Title
                            ui.heading(
                                egui::RichText::new("Adjust Team Points")
                                    .color(crate::theme::Palette::CYAN)
                                    .size(24.0),
                            );
                            ui.add_space(20.0);

                            // Team inputs in a scrollable area
                            egui::ScrollArea::vertical()
                                .max_height(250.0)
                                .show(ui, |ui| {
                                    for team in teams {
                                        ui.horizontal(|ui| {
                                            // Team name
                                            ui.label(
                                                egui::RichText::new(&team.name)
                                                    .color(crate::theme::Palette::CYBER_YELLOW)
                                                    .size(16.0),
                                            );
                                            ui.add_space(10.0);

                                            // Points input
                                            let input = modal
                                                .team_inputs
                                                .entry(team.id)
                                                .or_insert_with(|| team.score.to_string());
                                            let _input_value = input.clone();
                                            let response = ui.add(
                                                egui::TextEdit::singleline(input)
                                                    .desired_width(100.0)
                                                    .hint_text("Points"),
                                            );

                                            // Real-time validation
                                            if response.changed() {
                                                let current_input = modal
                                                    .team_inputs
                                                    .get(&team.id)
                                                    .cloned()
                                                    .unwrap_or_default();
                                                modal.validate_input(team.id, &current_input);
                                            }
                                        });

                                        // Show validation error if any
                                        if let Some(error) = modal.validation_errors.get(&team.id) {
                                            ui.label(
                                                egui::RichText::new(error)
                                                    .color(egui::Color32::from_rgb(255, 100, 100))
                                                    .size(12.0),
                                            );
                                        }
                                        ui.add_space(8.0);
                                    }
                                });

                            ui.add_space(20.0);

                            // Buttons
                            ui.horizontal(|ui| {
                                // Cancel button
                                if crate::theme::enhanced_modal_button(
                                    ui,
                                    "Cancel",
                                    crate::theme::ModalButtonType::Close,
                                )
                                .clicked()
                                {
                                    modal.hide();
                                }

                                ui.add_space(20.0);

                                // Apply button
                                let apply_enabled = modal.prepare_changes(teams);
                                let apply_button = if apply_enabled {
                                    crate::theme::enhanced_modal_button(
                                        ui,
                                        "Apply Changes",
                                        crate::theme::ModalButtonType::Correct,
                                    )
                                } else {
                                    ui.add_enabled(
                                        false,
                                        egui::Button::new("Apply Changes")
                                            .min_size(egui::vec2(120.0, 40.0)),
                                    )
                                };

                                if apply_button.clicked() && apply_enabled {
                                    result = Some(modal.pending_changes.clone());
                                    modal.hide();
                                }
                            });
                        });
                    });
            });
        });

    // Handle escape key to close modal
    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        modal.hide();
    }

    result
}
