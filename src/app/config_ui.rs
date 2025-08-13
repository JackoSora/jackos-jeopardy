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
    // Editing dialog state for config mode
    editing_cell: Option<(usize, usize)>, // (col, row)
    edit_question: String,
    edit_answer: String,
}

impl Default for EnhancedConfigUIState {
    fn default() -> Self {
        Self {
            cell_manager: CellManager::new(),
            transition_system: BoardEditorTransitionSystem::new(),
            transition_controller: TransitionController::new(),
            editing_cell: None,
            edit_question: String::new(),
            edit_answer: String::new(),
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

            ui.separator();
            // Board layout controls
            ui.label(egui::RichText::new("Layout").color(Palette::MAGENTA));
            if theme::accent_button(ui, "Add Category").clicked() {
                let cols = state.board.categories.len();
                if cols >= 10 {
                    ui.label(
                        egui::RichText::new("Max 10 categories").color(egui::Color32::YELLOW),
                    );
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

                    ui_state
                        .transition_system
                        .transition_to(ConfigLayoutState::EditorView);
                }
            }

            if theme::secondary_button(ui, "Add Row").clicked() {
                // Add a new row of clues across all categories, max 8 rows
                let rows = state
                    .board
                    .categories
                    .get(0)
                    .map(|c| c.clues.len())
                    .unwrap_or(0);
                if rows >= 8 {
                    ui.label(
                        egui::RichText::new("Max 8 rows").color(egui::Color32::YELLOW),
                    );
                } else {
                    let mut next_id: u32 = state
                        .board
                        .categories
                        .iter()
                        .flat_map(|cat| cat.clues.iter())
                        .map(|clue| clue.id)
                        .max()
                        .unwrap_or(0)
                        + 1;
                    let new_points: u32 = ((rows as u32) + 1) * 100;
                    for category in &mut state.board.categories {
                        category.clues.push(crate::core::domain::Clue {
                            id: next_id,
                            points: new_points,
                            question: String::new(),
                            answer: String::new(),
                            revealed: false,
                            solved: false,
                        });
                        next_id += 1;
                    }
                    ui_state
                        .transition_system
                        .transition_to(ConfigLayoutState::EditorView);
                }
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
        let spacing_y = ui.spacing().item_spacing.y;
        // Match game mode sizing behavior
        let total_spacing_x = spacing_x * (cols.saturating_sub(1)) as f32;
        let col_w = ((available.x - total_spacing_x) / cols as f32).max(60.0);
        // Header uses 15% height, clamped 40..60 like game mode
        let header_h = (available.y * 0.15).max(40.0).min(60.0);
        let total_spacing_y = spacing_y * rows as f32; // spacing between header and rows
        let remaining_height = (available.y - header_h - total_spacing_y).max(0.0);
        let cell_h = if rows > 0 {
            (remaining_height / rows as f32).max(50.0)
        } else {
            70.0
        };

        // Enhanced category headers with smooth transitions
        ui.horizontal(|ui| {
            ui.set_width(available.x);
            for (ci, category) in state.board.categories.iter_mut().enumerate() {
                let (rect, _response) =
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

        // Cells: adopt game mode visual layout and click to edit dialog
        let mut clicked: Option<(usize, usize)> = None;
        for row_idx in 0..rows {
            ui.horizontal(|ui| {
                ui.set_width(available.x);
                for (col_idx, category) in state.board.categories.iter().enumerate() {
                    let clue = &category.clues[row_idx];
                    let (rect, response) = ui.allocate_exact_size(
                        egui::vec2(col_w, cell_h),
                        egui::Sense::click(),
                    );
                    let painter = ui.painter_at(rect);
                    crate::ui::paint_enhanced_clue_cell(
                        &painter,
                        rect,
                        clue.points,
                        false, // not solved in config mode
                        response.hovered(),
                    );
                    if response.clicked() {
                        clicked = Some((col_idx, row_idx));
                    }
                }
            });
        }

        if let Some((c, r)) = clicked {
            ui_state.editing_cell = Some((c, r));
            if let Some(cat) = state.board.categories.get(c) {
                if let Some(clue) = cat.clues.get(r) {
                    ui_state.edit_question = clue.question.clone();
                    ui_state.edit_answer = clue.answer.clone();
                }
            }
        }

        // Editing modal for question/answer
        if let Some((c, r)) = ui_state.editing_cell {
            let screen = ui.ctx().screen_rect();
            egui::Area::new("config_edit_cell_modal".into())
                .order(egui::Order::Foreground)
                .movable(false)
                .interactable(true)
                .fixed_pos(screen.min)
                .show(ui.ctx(), |ui| {
                    let rect = screen;
                    let painter = ui.painter_at(rect);
                    crate::ui::paint_subtle_modal_background(&painter, rect);

                    // Centered window
                    let modal_w = (rect.width() * 0.7).clamp(400.0, 900.0);
                    let modal_h = (rect.height() * 0.6).clamp(260.0, 700.0);
                    let modal_rect = egui::Rect::from_center_size(
                        rect.center(),
                        egui::vec2(modal_w, modal_h),
                    );
                    let mut inner = ui.child_ui(modal_rect, egui::Layout::top_down(egui::Align::LEFT));
                    inner.scope(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(8.0, 10.0);
                        ui.add_space(6.0);
                        ui.heading(
                            egui::RichText::new("Edit Clue").color(Palette::CYAN).size(20.0),
                        );
                        ui.add_space(6.0);
                        ui.label(egui::RichText::new(format!("Category {} · Row {}", c + 1, r + 1)).color(Palette::MAGENTA));
                        ui.separator();

                        ui.label("Question");
                        ui.add(
                            egui::TextEdit::multiline(&mut ui_state.edit_question)
                                .desired_rows(5)
                                .hint_text("Enter question..."),
                        );
                        ui.add_space(4.0);
                        ui.label("Answer");
                        ui.add(
                            egui::TextEdit::multiline(&mut ui_state.edit_answer)
                                .desired_rows(3)
                                .hint_text("Enter answer..."),
                        );
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if crate::theme::accent_button(ui, "Save").clicked() {
                                if let Some(cat) = state.board.categories.get_mut(c) {
                                    if let Some(clue) = cat.clues.get_mut(r) {
                                        clue.question = ui_state.edit_question.clone();
                                        clue.answer = ui_state.edit_answer.clone();
                                    }
                                }
                                ui_state.editing_cell = None;
                            }
                            if crate::theme::secondary_button(ui, "Cancel").clicked() {
                                ui_state.editing_cell = None;
                            }
                        });
                    });
                });
        }

        // Controls moved to left panel; keep layout clean here.
    });

    // Store enhanced UI state back to memory
    ctx.memory_mut(|m| {
        m.data.insert_temp(ui_state_id, ui_state);
    });

    start_game
}
