use eframe::egui;

use crate::theme::{self, Palette};

use crate::domain::{Board, Category, ConfigState};
use crate::game::GameEngine;

pub fn show(ctx: &egui::Context, state: &mut ConfigState) -> Option<GameEngine> {
    let mut start_game: Option<GameEngine> = None;

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
        // layered background
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
        let header_h = 28.0;
        let cell_h = 64.0;

        // Headers (editable category titles)
        ui.horizontal(|ui| {
            ui.set_width(available.x);
            for (ci, category) in state.board.categories.iter_mut().enumerate() {
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(col_w, header_h), egui::Sense::hover());
                let painter = ui.painter_at(rect);
                painter.rect_filled(rect, 6.0, Palette::BG_ACTIVE);
                let mut title = category.name.clone();
                let galley = ui.painter().layout_no_wrap(
                    format!("Category {}:", ci + 1),
                    egui::FontId::proportional(13.0),
                    Palette::CYAN,
                );
                painter.galley(
                    rect.left_top() + egui::vec2(6.0, 6.0),
                    galley,
                    egui::Color32::TRANSPARENT,
                );
                // Inline editor overlay
                let edit_rect = egui::Rect::from_min_size(
                    rect.left_top() + egui::vec2(6.0, 24.0),
                    egui::vec2(col_w - 12.0, header_h - 26.0),
                );
                let resp = ui.put(
                    edit_rect,
                    egui::TextEdit::singleline(&mut title).hint_text("Name"),
                );
                if resp.changed() {
                    category.name = title;
                }
            }
        });

        // Rows of clues (question/answer)
        for row_idx in 0..rows {
            ui.horizontal(|ui| {
                ui.set_width(available.x);
                for category in state.board.categories.iter_mut() {
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(col_w, cell_h), egui::Sense::hover());
                    let painter = ui.painter_at(rect);
                    painter.rect_filled(rect, 6.0, Palette::BG_PANEL);
                    painter.rect_stroke(
                        rect.expand(1.0),
                        6.0,
                        egui::Stroke::new(1.0, Palette::CYAN),
                    );

                    // Inset fields
                    let inner = rect.shrink2(egui::vec2(6.0, 8.0));
                    let left = egui::Rect::from_min_max(
                        inner.min,
                        egui::pos2(inner.min.x + 70.0, inner.max.y),
                    );
                    let right = egui::Rect::from_min_max(
                        egui::pos2(left.max.x + 6.0, inner.min.y),
                        inner.max,
                    );
                    ui.put(
                        left,
                        egui::Label::new(
                            egui::RichText::new(format!(
                                "{:>3} pts",
                                category.clues[row_idx].points
                            ))
                            .color(Palette::MAGENTA),
                        )
                        .wrap(false),
                    );
                    ui.put(
                        right.split_top_bottom_at_y(right.min.y + 24.0).0,
                        egui::TextEdit::singleline(&mut category.clues[row_idx].question)
                            .hint_text("Question"),
                    );
                    ui.put(
                        right.split_top_bottom_at_y(right.min.y + 24.0).1,
                        egui::TextEdit::singleline(&mut category.clues[row_idx].answer)
                            .hint_text("Answer"),
                    );
                }
            });
        }

        ui.separator();
        ui.horizontal(|ui| {
            if theme::accent_button(ui, "Add Category").clicked() {
                if state.board.categories.len() >= 10 {
                    // soft limit: show toast-like label
                    ui.label(egui::RichText::new("Max 10 categories").color(egui::Color32::YELLOW));
                } else {
                    let rows = state
                        .board
                        .categories
                        .get(0)
                        .map(|c| c.clues.len())
                        .unwrap_or(5);
                    state.board.categories.push(Category {
                        name: "New".into(),
                        clues: Board::default_with_dimensions(1, rows)
                            .categories
                            .remove(0)
                            .clues,
                    });
                }
            }
            if cols > 0 && theme::danger_button(ui, "Remove Last").clicked() {
                state.board.categories.pop();
            }
        });
    });

    start_game
}
