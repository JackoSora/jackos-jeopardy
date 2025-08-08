use eframe::egui;

use crate::app::AppMode;
use crate::domain::Board;
use crate::game::{GameState, PlayPhase};

use rand::seq::SliceRandom;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
enum AnswerFlash {
    Correct,
    Incorrect,
}

pub fn show(ctx: &egui::Context, gs: &mut GameState) -> Option<AppMode> {
    egui::SidePanel::left("teams").show(ctx, |ui| {
        ui.heading("Teams");
        let in_lobby = matches!(gs.phase, PlayPhase::Lobby);
        for team in &mut gs.teams {
            ui.horizontal(|ui| {
                if in_lobby {
                    ui.add(egui::TextEdit::singleline(&mut team.name));
                    ui.label(format!(" — {}", team.score));
                } else {
                    ui.label(format!("{} — {}", team.name, team.score));
                }
            });
        }
        if ui.button("Add Team").clicked() {
            gs.add_team(format!("Team {}", gs.teams.len() + 1));
        }
    });

    let mut next_mode: Option<AppMode> = None;
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Game Board");
        let mut requested_phase: Option<PlayPhase> = None;
        // transient flash state stored in memory per frame (simple effect)
        // We track it via a temporary Id and memory
        let flash_id = ui.id().with("answer_flash");
        let mut flash: Option<(AnswerFlash, Instant)> =
            ui.memory_mut(|m| m.data.get_temp(flash_id)).unwrap_or(None);
        match gs.phase {
            PlayPhase::Lobby => {
                ui.label("Lobby: Add teams and press Start");
                if ui.button("Start").clicked() {
                    if let Some(first) = gs.teams.first() {
                        requested_phase = Some(PlayPhase::Selecting { team_id: first.id });
                        gs.active_team = first.id; // safe, not touching phase
                    }
                }
            }
            PlayPhase::Selecting { team_id } => {
                ui.label(format!("Selecting — Active Team: {}", team_id));
                let cols = gs.board.categories.len().max(1);
                let rows = gs
                    .board
                    .categories
                    .get(0)
                    .map(|c| c.clues.len())
                    .unwrap_or(0);

                let available = ui.available_size();
                let spacing_x = ui.spacing().item_spacing.x;
                let total_spacing = spacing_x * (cols.saturating_sub(1)) as f32;
                let cell_w = ((available.x - total_spacing) / cols as f32).max(60.0);
                let header_h = 40.0;
                let cell_h = 70.0;

                ui.horizontal(|ui| {
                    ui.set_width(available.x);
                    for cat in &gs.board.categories {
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(cell_w, header_h),
                            egui::Sense::hover(),
                        );
                        let painter = ui.painter_at(rect);
                        painter.rect_filled(rect, 6.0, egui::Color32::from_rgb(15, 15, 30));
                        painter.text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            &cat.name,
                            egui::FontId::proportional(18.0),
                            egui::Color32::from_rgb(0, 255, 170),
                        );
                        painter.line_segment(
                            [rect.left_bottom(), rect.right_bottom()],
                            egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 0, 150)),
                        );
                    }
                });

                for r in 0..rows {
                    ui.horizontal(|ui| {
                        ui.set_width(available.x);
                        for (ci, cat) in gs.board.categories.iter().enumerate() {
                            let clue = &cat.clues[r];
                            let (rect, response) = ui.allocate_exact_size(
                                egui::vec2(cell_w, cell_h),
                                egui::Sense::click(),
                            );
                            let painter = ui.painter_at(rect);
                            let base = if clue.solved {
                                egui::Color32::from_rgb(25, 25, 40)
                            } else {
                                egui::Color32::from_rgb(36, 0, 58)
                            };
                            painter.rect_filled(rect, 6.0, base);
                            painter.rect_stroke(
                                rect.expand(1.0),
                                6.0,
                                egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 255, 170)),
                            );
                            painter.text(
                                rect.center(),
                                egui::Align2::CENTER_CENTER,
                                format!("{}", clue.points),
                                egui::FontId::proportional(20.0),
                                egui::Color32::from_rgb(208, 255, 247),
                            );
                            if !clue.solved && response.clicked() {
                                requested_phase = Some(PlayPhase::Showing {
                                    clue: (ci, r),
                                    owner_team_id: team_id,
                                });
                            }
                        }
                    });
                }
            }
            PlayPhase::Showing {
                clue,
                owner_team_id,
            } => {
                // Full-window overlay for the active question
                let screen = ctx.screen_rect();
                egui::Area::new("question_full_overlay".into())
                    .order(egui::Order::Foreground)
                    .movable(false)
                    .interactable(true)
                    .fixed_pos(screen.min)
                    .show(ctx, |ui| {
                        let rect = screen;
                        let painter = ui.painter_at(rect);
                        painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(10, 10, 18));

                        let (question, points) = if let Some(cat) = gs.board.categories.get(clue.0)
                        {
                            if let Some(c) = cat.clues.get(clue.1) {
                                (c.question.clone(), c.points)
                            } else {
                                (String::new(), 0)
                            }
                        } else {
                            (String::new(), 0)
                        };

                        // Top section content
                        ui.allocate_ui_with_layout(
                            rect.size(),
                            egui::Layout::top_down(egui::Align::Center),
                            |ui| {
                                ui.add_space(32.0);
                                ui.heading(
                                    egui::RichText::new(format!("{} pts", points))
                                        .color(egui::Color32::from_rgb(0, 255, 170)),
                                );
                                ui.add_space(16.0);
                                let wrap_width = rect.width() * 0.9;
                                let label =
                                    egui::Label::new(egui::RichText::new(question).size(28.0))
                                        .wrap(true)
                                        .truncate(false);
                                ui.add_sized([wrap_width, 0.0], label);
                            },
                        );

                        // Bottom-centered action bar
                        let bottom_h = 90.0;
                        let bottom_rect = egui::Rect::from_min_size(
                            egui::pos2(rect.left(), rect.bottom() - bottom_h - 8.0),
                            egui::vec2(rect.width(), bottom_h),
                        );
                        ui.allocate_ui_at_rect(bottom_rect, |ui| {
                            ui.with_layout(
                                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                                |ui| {
                                    ui.set_width(bottom_rect.width());
                                    ui.horizontal(|ui| {
                                        let correct_btn = egui::Button::new(
                                            egui::RichText::new("Correct")
                                                .strong()
                                                .color(egui::Color32::BLACK),
                                        )
                                        .fill(egui::Color32::from_rgb(0, 255, 170))
                                        .min_size(egui::vec2(160.0, 44.0));
                                        if ui.add(correct_btn).clicked() {
                                            if let Some(c) = gs
                                                .board
                                                .categories
                                                .get_mut(clue.0)
                                                .and_then(|cat| cat.clues.get_mut(clue.1))
                                            {
                                                c.revealed = true;
                                                c.solved = true;
                                                if let Some(team) = gs
                                                    .teams
                                                    .iter_mut()
                                                    .find(|t| t.id == owner_team_id)
                                                {
                                                    team.score += c.points as i32;
                                                }
                                            }
                                            flash = Some((AnswerFlash::Correct, Instant::now()));
                                            requested_phase = Some(PlayPhase::Resolved {
                                                clue,
                                                next_team_id: owner_team_id,
                                            });
                                        }

                                        ui.add_space(24.0);

                                        let incorrect_btn = egui::Button::new(
                                            egui::RichText::new("Incorrect")
                                                .strong()
                                                .color(egui::Color32::WHITE),
                                        )
                                        .fill(egui::Color32::from_rgb(255, 0, 150))
                                        .min_size(egui::vec2(160.0, 44.0));
                                        if ui.add(incorrect_btn).clicked() {
                                            let mut others: Vec<u32> = gs
                                                .teams
                                                .iter()
                                                .filter(|t| t.id != owner_team_id)
                                                .map(|t| t.id)
                                                .collect();
                                            let mut rng = rand::thread_rng();
                                            others.as_mut_slice().shuffle(&mut rng);
                                            let mut queue =
                                                std::collections::VecDeque::from(others);
                                            let current =
                                                queue.pop_front().unwrap_or(owner_team_id);
                                            flash = Some((AnswerFlash::Incorrect, Instant::now()));
                                            requested_phase = Some(PlayPhase::Steal {
                                                clue,
                                                queue,
                                                current,
                                                owner_team_id,
                                            });
                                        }
                                    });
                                },
                            );
                        });
                    });
            }
            PlayPhase::Steal {
                clue,
                ref mut queue,
                ref mut current,
                owner_team_id,
            } => {
                // Full-window overlay for steal attempt (question remains visible)
                let screen = ctx.screen_rect();
                egui::Area::new("steal_full_overlay".into())
                    .order(egui::Order::Foreground)
                    .movable(false)
                    .interactable(true)
                    .fixed_pos(screen.min)
                    .show(ctx, |ui| {
                        let rect = screen;
                        let painter = ui.painter_at(rect);
                        painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(12, 12, 20));
                        // Top content: points + question remain visible
                        let (question, points) = if let Some(cat) = gs.board.categories.get(clue.0)
                        {
                            if let Some(c) = cat.clues.get(clue.1) {
                                (c.question.clone(), c.points)
                            } else {
                                (String::new(), 0)
                            }
                        } else {
                            (String::new(), 0)
                        };

                        ui.allocate_ui_with_layout(
                            rect.size(),
                            egui::Layout::top_down(egui::Align::Center),
                            |ui| {
                                ui.add_space(24.0);
                                ui.heading(
                                    egui::RichText::new(format!("{} pts", points))
                                        .color(egui::Color32::from_rgb(0, 255, 170)),
                                );
                                ui.add_space(12.0);
                                let wrap_width = rect.width() * 0.9;
                                let label =
                                    egui::Label::new(egui::RichText::new(question).size(26.0))
                                        .wrap(true)
                                        .truncate(false);
                                ui.add_sized([wrap_width, 0.0], label);
                                ui.add_space(8.0);
                                let team_name = gs
                                    .teams
                                    .iter()
                                    .find(|t| t.id == *current)
                                    .map(|t| t.name.clone())
                                    .unwrap_or_else(|| format!("#{}", current));
                                ui.label(
                                    egui::RichText::new(format!("Steal: {}", team_name)).size(20.0),
                                );
                            },
                        );

                        // Bottom-centered action bar with styled buttons (same as Showing)
                        let bottom_h = 90.0;
                        let bottom_rect = egui::Rect::from_min_size(
                            egui::pos2(rect.left(), rect.bottom() - bottom_h - 8.0),
                            egui::vec2(rect.width(), bottom_h),
                        );
                        ui.allocate_ui_at_rect(bottom_rect, |ui| {
                            ui.with_layout(
                                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                                |ui| {
                                    ui.set_width(bottom_rect.width());
                                    ui.horizontal(|ui| {
                                        let correct_btn = egui::Button::new(
                                            egui::RichText::new("Correct")
                                                .strong()
                                                .color(egui::Color32::BLACK),
                                        )
                                        .fill(egui::Color32::from_rgb(0, 255, 170))
                                        .min_size(egui::vec2(160.0, 44.0));
                                        if ui.add(correct_btn).clicked() {
                                            if let Some(c) = gs
                                                .board
                                                .categories
                                                .get_mut(clue.0)
                                                .and_then(|cat| cat.clues.get_mut(clue.1))
                                            {
                                                c.revealed = true;
                                                c.solved = true;
                                                if let Some(team) =
                                                    gs.teams.iter_mut().find(|t| t.id == *current)
                                                {
                                                    team.score += c.points as i32;
                                                }
                                            }
                                            flash = Some((AnswerFlash::Correct, Instant::now()));
                                            requested_phase = Some(PlayPhase::Resolved {
                                                clue,
                                                next_team_id: *current,
                                            });
                                        }

                                        ui.add_space(24.0);

                                        let incorrect_btn = egui::Button::new(
                                            egui::RichText::new("Incorrect")
                                                .strong()
                                                .color(egui::Color32::WHITE),
                                        )
                                        .fill(egui::Color32::from_rgb(255, 0, 150))
                                        .min_size(egui::vec2(160.0, 44.0));
                                        if ui.add(incorrect_btn).clicked() {
                                            if let Some(next) = queue.pop_front() {
                                                *current = next;
                                            } else {
                                                if let Some(c) = gs
                                                    .board
                                                    .categories
                                                    .get_mut(clue.0)
                                                    .and_then(|cat| cat.clues.get_mut(clue.1))
                                                {
                                                    c.solved = true;
                                                }
                                                flash =
                                                    Some((AnswerFlash::Incorrect, Instant::now()));
                                                requested_phase = Some(PlayPhase::Resolved {
                                                    clue,
                                                    next_team_id: owner_team_id,
                                                });
                                            }
                                        }
                                    });
                                },
                            );
                        });
                    });
            }
            PlayPhase::Resolved { clue, next_team_id } => {
                // Full-window overlay showing the correct answer and a Close button
                let screen = ctx.screen_rect();
                egui::Area::new("resolved_full_overlay".into())
                    .order(egui::Order::Foreground)
                    .movable(false)
                    .interactable(true)
                    .fixed_pos(screen.min)
                    .show(ctx, |ui| {
                        let rect = screen;
                        let painter = ui.painter_at(rect);
                        painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(10, 10, 18));

                        let (question, answer, points) =
                            if let Some(cat) = gs.board.categories.get(clue.0) {
                                if let Some(c) = cat.clues.get(clue.1) {
                                    (c.question.clone(), c.answer.clone(), c.points)
                                } else {
                                    (String::new(), String::new(), 0)
                                }
                            } else {
                                (String::new(), String::new(), 0)
                            };

                        // Top content
                        ui.allocate_ui_with_layout(
                            rect.size(),
                            egui::Layout::top_down(egui::Align::Center),
                            |ui| {
                                ui.add_space(32.0);
                                ui.heading(
                                    egui::RichText::new(format!("{} pts", points))
                                        .color(egui::Color32::from_rgb(0, 255, 170)),
                                );
                                ui.add_space(12.0);
                                let wrap_width = rect.width() * 0.9;
                                let q_label =
                                    egui::Label::new(egui::RichText::new(question).size(24.0))
                                        .wrap(true)
                                        .truncate(false);
                                ui.add_sized([wrap_width, 0.0], q_label);
                                ui.add_space(8.0);
                                let a_label = egui::Label::new(
                                    egui::RichText::new(format!("Answer: {}", answer))
                                        .color(egui::Color32::from_rgb(255, 0, 150))
                                        .size(22.0),
                                )
                                .wrap(true)
                                .truncate(false);
                                ui.add_sized([wrap_width, 0.0], a_label);
                            },
                        );

                        // Bottom-centered Close button
                        let bottom_h = 90.0;
                        let bottom_rect = egui::Rect::from_min_size(
                            egui::pos2(rect.left(), rect.bottom() - bottom_h - 8.0),
                            egui::vec2(rect.width(), bottom_h),
                        );
                        ui.allocate_ui_at_rect(bottom_rect, |ui| {
                            ui.with_layout(
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    ui.set_width(bottom_rect.width());
                                    ui.horizontal_centered(|ui| {
                                        let close_btn = egui::Button::new(
                                            egui::RichText::new("Close")
                                                .strong()
                                                .color(egui::Color32::BLACK),
                                        )
                                        .fill(egui::Color32::from_rgb(0, 255, 170))
                                        .min_size(egui::vec2(200.0, 48.0));
                                        if ui.add(close_btn).clicked() {
                                            requested_phase = Some(PlayPhase::Selecting {
                                                team_id: next_team_id,
                                            });
                                            ui.ctx().request_repaint();
                                        }
                                    });
                                },
                            );
                        });
                    });
            }
            PlayPhase::Intermission => {
                ui.label("Intermission");
            }
            PlayPhase::Finished => {
                ui.label("Finished");
                if ui.button("Back to Config").clicked() {
                    next_mode = Some(AppMode::Config(crate::domain::ConfigState {
                        board: Board::default(),
                    }));
                }
            }
        }
        if let Some(p) = requested_phase {
            gs.phase = p;
            // ensure overlay state is cleared when closing
            ui.memory_mut(|m| m.data.remove::<Option<(AnswerFlash, Instant)>>(flash_id));
        }

        // Draw transient full-screen flash overlay
        if let Some((kind, start)) = flash {
            let elapsed = start.elapsed();
            let duration = Duration::from_millis(700);
            if elapsed < duration {
                let t = (elapsed.as_secs_f32() / duration.as_secs_f32()).clamp(0.0, 1.0);
                let alpha = (220.0 * (1.0 - t)) as u8;
                let color = match kind {
                    AnswerFlash::Correct => {
                        egui::Color32::from_rgba_unmultiplied(0, 255, 170, alpha)
                    }
                    AnswerFlash::Incorrect => {
                        egui::Color32::from_rgba_unmultiplied(255, 0, 150, alpha)
                    }
                };
                let rect = ui.max_rect();
                // vignette-like flash: outer border plus inner fill
                ui.painter().rect_filled(rect, 0.0, color);
                let inset = 20.0 * (1.0 + 0.2 * (1.0 - t));
                ui.painter()
                    .rect_stroke(rect.shrink(inset), 8.0, egui::Stroke::new(6.0, color));
                ui.ctx().request_repaint();
                ui.memory_mut(|m| m.data.insert_temp(flash_id, Some((kind, start))));
            } else {
                ui.memory_mut(|m| m.data.remove::<Option<(AnswerFlash, Instant)>>(flash_id));
            }
        } else {
            // clear stored flash if any
            ui.memory_mut(|m| m.data.remove::<Option<(AnswerFlash, Instant)>>(flash_id));
        }
    });

    next_mode
}
