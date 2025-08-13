use eframe::egui;

use crate::core::{Board, Category, ConfigState};
use crate::game::GameEngine;
use crate::theme::{self, Palette, TransitionController};
use crate::ui::{BoardEditorTransitionSystem, CellId, CellManager, ConfigLayoutState};

// Enhanced config UI state (stored in egui memory)
#[derive(Clone)]
struct EnhancedConfigUIState {
    cell_manager: CellManager,
    transition_system: BoardEditorTransitionSystem,
    transition_controller: TransitionController,
}

impl Default for EnhancedConfigUIState {
    fn default() -> Self {
        Self {
            cell_manager: CellManager::new(),
            transition_system: BoardEditorTransitionSystem::new(),
            transition_controller: TransitionController::new(),
        }
    }
}

pub fn show(ctx: &egui::Context, state: &mut ConfigState) -> Option<GameEngine> {
    let mut start_game: Option<GameEngine> = None;

    // Get or create enhanced UI state
    let ui_state_id = egui::Id::new("enhanced_config_ui_state");
    let mut ui_state: EnhancedConfigUIState =
        ctx.memory_mut(|m| m.data.get_temp(ui_state_id).unwrap_or_default());

    egui::SidePanel::left("config_left")
        .frame(theme::panel_frame())
        .show(ctx, |ui| {
            ui.heading(egui::RichText::new("Board Editor").color(Palette::CYAN));
            if theme::secondary_button(ui, "New Board").clicked() {
                state.board = Board::default();
            }
            if theme::accent_button(ui, "Start Game").clicked() {
                start_game = Some(GameEngine::new(state.board.clone()));
            }
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        // Update animations and check if repaint is needed
        let needs_repaint =
            ui_state.cell_manager.update_animations() || ui_state.transition_system.update();

        if needs_repaint {
            ctx.request_repaint();
        }

        // Enhanced background with smooth transitions
        crate::theme::paint_board_background(ui);
        ui.heading(egui::RichText::new("Board Layout").color(Palette::CYAN));

        let cols = state.board.categories.len().max(1);
        let rows = state
            .board
            .categories
            .get(0)
            .map(|c| c.clues.len())
            .unwrap_or(0);

        let available = ui.available_size();
        let spacing_x = ui.spacing().item_spacing.x;
        let total_spacing = spacing_x * (cols.saturating_sub(1)) as f32;
        let col_w = ((available.x - total_spacing) / cols as f32).max(140.0);
        let header_h = 40.0; // Increased for better visual hierarchy
        let cell_h = 80.0; // Increased for enhanced cells

        // Enhanced category headers with smooth transitions
        ui.horizontal(|ui| {
            ui.set_width(available.x);
            for (ci, category) in state.board.categories.iter_mut().enumerate() {
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(col_w, header_h), egui::Sense::hover());

                // Use enhanced category header rendering
                crate::ui::paint_enhanced_category_header(
                    &ui.painter_at(rect),
                    rect,
                    &format!("Category {}", ci + 1),
                );

                // Enhanced title editing with better visual feedback
                let mut title = category.name.clone();
                let edit_rect = egui::Rect::from_min_size(
                    rect.center() - egui::vec2(col_w * 0.4, 8.0),
                    egui::vec2(col_w * 0.8, 16.0),
                );

                let title_response = ui.put(
                    edit_rect,
                    egui::TextEdit::singleline(&mut title)
                        .hint_text("Category Name")
                        .font(egui::FontId::proportional(14.0)),
                );

                if title_response.changed() {
                    category.name = title;
                }
            }
        });

        // Enhanced cells with proper visual boundaries and content separation
        for row_idx in 0..rows {
            ui.horizontal(|ui| {
                ui.set_width(available.x);
                for (col_idx, category) in state.board.categories.iter_mut().enumerate() {
                    let cell_id: CellId = (col_idx, row_idx);
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(col_w, cell_h), egui::Sense::hover());

                    // Update cell state based on content
                    ui_state.cell_manager.update_cell_state(
                        cell_id,
                        &category.clues[row_idx].question,
                        &category.clues[row_idx].answer,
                    );

                    // Render enhanced cell with proper content separation
                    let mut question = category.clues[row_idx].question.clone();
                    let mut answer = category.clues[row_idx].answer.clone();

                    let cell_response = {
                        let cell = ui_state.cell_manager.get_or_create_cell(cell_id);
                        cell.render(
                            ui,
                            rect,
                            category.clues[row_idx].points,
                            &mut question,
                            &mut answer,
                        )
                    };

                    // Update category data if cell content changed
                    if cell_response.question_changed {
                        category.clues[row_idx].question = question;
                    }
                    if cell_response.answer_changed {
                        category.clues[row_idx].answer = answer;
                    }

                    // Handle cell interactions
                    ui_state
                        .cell_manager
                        .handle_cell_response(cell_id, cell_response);
                }
            });
        }

        // Enhanced control buttons with smooth transitions
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            if theme::accent_button(ui, "Add Category").clicked() {
                if state.board.categories.len() >= 10 {
                    ui.label(egui::RichText::new("Max 10 categories").color(egui::Color32::YELLOW));
                } else {
                    let rows = state
                        .board
                        .categories
                        .get(0)
                        .map(|c| c.clues.len())
                        .unwrap_or(5);
                    state.board.categories.push(Category {
                        name: "New Category".into(),
                        clues: Board::default_with_dimensions(1, rows)
                            .categories
                            .remove(0)
                            .clues,
                    });

                    // Trigger layout transition for new category
                    ui_state
                        .transition_system
                        .transition_to(ConfigLayoutState::EditorView);
                }
            }

            ui.add_space(8.0);

            if cols > 0 && theme::danger_button(ui, "Remove Last").clicked() {
                state.board.categories.pop();

                // Clean up unused cells
                let valid_ids: Vec<CellId> = (0..state.board.categories.len())
                    .flat_map(|col| (0..rows).map(move |row| (col, row)))
                    .collect();
                ui_state.cell_manager.cleanup_unused_cells(&valid_ids);

                // Trigger layout transition
                ui_state
                    .transition_system
                    .transition_to(ConfigLayoutState::BoardView);
            }
        });
    });

    // Store enhanced UI state back to memory
    ctx.memory_mut(|m| {
        m.data.insert_temp(ui_state_id, ui_state);
    });

    start_game
}
