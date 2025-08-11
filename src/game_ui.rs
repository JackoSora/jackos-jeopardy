use eframe::egui;

use crate::app::AppMode;
use crate::domain::Board;
use crate::game::events::{EventAnimationController, EventAnimationType, GameEvent};
use crate::game::{GameAction, GameActionResult, GameEngine, PlayPhase};
use crate::theme::Palette;
use crate::theme::{ModalButtonType, adjust_brightness, enhanced_modal_button};
use crate::ui::{
    paint_enhanced_category_header, paint_enhanced_clue_cell, paint_subtle_modal_background,
};

use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
enum AnswerFlash {
    Correct,
    Incorrect,
}

#[derive(Clone, Copy, PartialEq)]
enum StealOutcome {
    Correct,
    Incorrect,
}

pub fn show(ctx: &egui::Context, game_engine: &mut GameEngine) -> Option<AppMode> {
    egui::SidePanel::left("teams")
        .frame(crate::theme::panel_frame())
        .show(ctx, |ui| {
            ui.heading(egui::RichText::new("Teams").color(Palette::CYAN));
            let in_lobby = matches!(game_engine.get_phase(), PlayPhase::Lobby);
            for team in &mut game_engine.get_state_mut().teams {
                ui.horizontal(|ui| {
                    if in_lobby {
                        ui.add(egui::TextEdit::singleline(&mut team.name));
                        ui.label(format!(" — {}", team.score));
                    } else {
                        ui.label(format!("{} — {}", team.name, team.score));
                    }
                });
            }
            if crate::theme::accent_button(ui, "Add Team").clicked() {
                let action = GameAction::AddTeam {
                    name: format!("Team {}", game_engine.team_count() + 1),
                };
                let _ = game_engine.handle_action(action);
            }
        });

    let mut next_mode: Option<AppMode> = None;
    egui::CentralPanel::default().show(ctx, |ui| {
        crate::theme::paint_board_background(ui);
        ui.heading(egui::RichText::new("Game Board").color(Palette::CYAN));
        let mut requested_phase: Option<PlayPhase> = None;
        let flash_id = ui.id().with("answer_flash");
        let pending_answer_id = ui.id().with("pending_answer");
        let pending_steal_id = ui.id().with("pending_steal");
        let event_animation_id = ui.id().with("event_animation");
        let mut flash: Option<(AnswerFlash, Instant)> =
            ui.memory_mut(|m| m.data.get_temp(flash_id)).unwrap_or(None);
        let mut pending_answer: Option<(AnswerFlash, (usize, usize), u32)> = ui
            .memory_mut(|m| m.data.get_temp(pending_answer_id))
            .unwrap_or(None);
        let mut pending_steal: Option<(StealOutcome, (usize, usize), u32)> = ui
            .memory_mut(|m| m.data.get_temp(pending_steal_id))
            .unwrap_or(None);
        let mut event_animation: Option<EventAnimationController> = ui
            .memory_mut(|m| m.data.get_temp(event_animation_id))
            .unwrap_or(None);

        match game_engine.get_phase() {
            PlayPhase::Lobby => {
                ui.label("Lobby: Add teams and press Start");
                if crate::theme::accent_button(ui, "Start").clicked() {
                    let action = GameAction::StartGame;
                    if let Ok(result) = game_engine.handle_action(action) {
                        match result {
                            GameActionResult::Success { new_phase } => {
                                requested_phase = Some(new_phase)
                            }
                            GameActionResult::StateChanged { new_phase, .. } => {
                                requested_phase = Some(new_phase)
                            }
                            _ => {}
                        }
                    }
                }
            }
            PlayPhase::Selecting { team_id } => {
                ui.label(
                    egui::RichText::new(format!("Selecting — Active Team: {}", team_id))
                        .color(Palette::MAGENTA),
                );
                let cols = game_engine.get_state().board.categories.len().max(1);
                let rows = game_engine
                    .get_state()
                    .board
                    .categories
                    .get(0)
                    .map(|c| c.clues.len())
                    .unwrap_or(0);
                let available = ui.available_size();
                let spacing_x = ui.spacing().item_spacing.x;
                let spacing_y = ui.spacing().item_spacing.y;

                // Calculate cell dimensions based on available space
                let total_spacing_x = spacing_x * (cols.saturating_sub(1)) as f32;
                let total_spacing_y = spacing_y * rows as f32; // spacing between header and rows
                let cell_w = ((available.x - total_spacing_x) / cols as f32).max(60.0);

                // Reserve space for header and distribute remaining height among rows
                let header_h = (available.y * 0.15).max(40.0).min(60.0); // 15% of height, min 40px, max 60px
                let remaining_height = available.y - header_h - total_spacing_y;
                let cell_h = if rows > 0 {
                    (remaining_height / rows as f32).max(50.0)
                } else {
                    70.0
                };
                ui.horizontal(|ui| {
                    ui.set_width(available.x);
                    for cat in &game_engine.get_state().board.categories {
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(cell_w, header_h),
                            egui::Sense::hover(),
                        );
                        let painter = ui.painter_at(rect);
                        paint_enhanced_category_header(&painter, rect, &cat.name);
                    }
                });
                let mut clicked_clue: Option<(usize, usize)> = None;
                for r in 0..rows {
                    ui.horizontal(|ui| {
                        ui.set_width(available.x);
                        for (ci, cat) in game_engine.get_state().board.categories.iter().enumerate()
                        {
                            let clue = &cat.clues[r];
                            let (rect, response) = ui.allocate_exact_size(
                                egui::vec2(cell_w, cell_h),
                                egui::Sense::click(),
                            );
                            let painter = ui.painter_at(rect);
                            paint_enhanced_clue_cell(
                                &painter,
                                rect,
                                clue.points,
                                clue.solved,
                                response.hovered(),
                            );
                            if !clue.solved && response.clicked() {
                                clicked_clue = Some((ci, r));
                            }
                        }
                    });
                }

                // Handle clue selection outside the iteration
                if let Some(clue) = clicked_clue {
                    // Check if there's a queued event that should play animation first
                    if game_engine.get_state().event_state.has_queued_event()
                        && !game_engine.get_state().event_state.is_animation_playing()
                    {
                        // Start the queued event animation instead of selecting the clue
                        if let Some(queued_event) =
                            game_engine.get_state().event_state.queued_event.clone()
                        {
                            let action = GameAction::PlayEventAnimation {
                                event: queued_event,
                            };
                            let _ = game_engine.handle_action(action);
                        }
                    } else if !game_engine.get_state().event_state.is_animation_playing() {
                        // Only allow clue selection if no animation is playing
                        let action = GameAction::SelectClue {
                            clue,
                            team_id: *team_id,
                        };
                        if let Ok(result) = game_engine.handle_action(action) {
                            match result {
                                GameActionResult::Success { new_phase } => {
                                    requested_phase = Some(new_phase)
                                }
                                GameActionResult::StateChanged { new_phase, .. } => {
                                    requested_phase = Some(new_phase)
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            PlayPhase::Showing {
                clue,
                owner_team_id,
            } => {
                draw_showing_overlay(
                    ctx,
                    game_engine,
                    *clue,
                    *owner_team_id,
                    &mut flash,
                    &mut requested_phase,
                    &mut pending_answer,
                );
            }
            PlayPhase::Steal {
                clue,
                queue: _,
                current,
                owner_team_id: _,
            } => {
                let current_team_id = *current;
                let has_more =
                    if let PlayPhase::Steal { queue, .. } = &game_engine.get_state().phase {
                        !queue.is_empty()
                    } else {
                        false
                    };
                // Precompute immutable data needed for overlay
                let (question, points) = game_engine
                    .get_state()
                    .board
                    .categories
                    .get(clue.0)
                    .and_then(|cat| cat.clues.get(clue.1))
                    .map(|c| (c.question.clone(), c.points))
                    .unwrap_or_default();
                let team_name = game_engine
                    .get_state()
                    .teams
                    .iter()
                    .find(|t| t.id == current_team_id)
                    .map(|t| t.name.clone())
                    .unwrap_or_else(|| format!("#{}", current_team_id));
                if let Some(outcome) = draw_steal_overlay(
                    ctx,
                    &question,
                    points,
                    &team_name,
                    has_more,
                    &mut flash,
                    &mut pending_steal,
                ) {
                    // Store pending steal action to be executed after animation completes
                    if pending_steal.is_none() {
                        pending_steal = Some((outcome, *clue, current_team_id));
                    }
                }
            }
            PlayPhase::Resolved { clue, next_team_id } => {
                draw_resolved_overlay(
                    ctx,
                    game_engine,
                    *clue,
                    *next_team_id,
                    &mut requested_phase,
                    &flash,
                );
            }
            PlayPhase::Intermission => {
                ui.label("Intermission");
            }
            PlayPhase::Finished => {
                ui.label("Finished");
                if crate::theme::secondary_button(ui, "Back to Config").clicked() {
                    next_mode = Some(AppMode::Config(crate::domain::ConfigState {
                        board: Board::default(),
                    }));
                }
            }
        }

        if let Some(p) = requested_phase {
            game_engine.get_state_mut().phase = p;
            ui.memory_mut(|m| {
                // Preserve active flash animation so it can finish after phase switch (e.g. Steal -> Resolved)
                if let Some(active) = flash {
                    m.data.insert_temp(flash_id, Some(active));
                } else {
                    m.data.remove::<Option<(AnswerFlash, Instant)>>(flash_id);
                }
                // Pending answer and steal only used in their respective phases
                m.data
                    .remove::<Option<(AnswerFlash, (usize, usize), u32)>>(pending_answer_id);
                m.data
                    .remove::<Option<(StealOutcome, (usize, usize), u32)>>(pending_steal_id);
            });
        }

        if let Some((kind, start)) = flash {
            let elapsed = start.elapsed();
            let duration = Duration::from_millis(1200); // Extended duration for more expressive animation
            if elapsed < duration {
                let t = (elapsed.as_secs_f32() / duration.as_secs_f32()).clamp(0.0, 1.0);
                let ctx = ui.ctx();
                let rect = ctx.screen_rect();
                egui::Area::new("answer_flash_overlay".into())
                    .order(egui::Order::Foreground)
                    .movable(false)
                    .interactable(false)
                    .fixed_pos(rect.min)
                    .show(ctx, |ui| {
                        let painter = ui.painter_at(rect);
                        match kind {
                            AnswerFlash::Correct => {
                                // Success burst animation with multiple layers
                                draw_success_animation(&painter, rect, t);
                            }
                            AnswerFlash::Incorrect => {
                                // Use the same burst animation style but red variant
                                draw_failure_animation(&painter, rect, t);
                            }
                        }
                    });

                ctx.request_repaint();
                ui.memory_mut(|m| m.data.insert_temp(flash_id, Some((kind, start))));
            } else {
                // Animation finished -> if we have a pending answer or steal, now apply the game action
                if let Some((pending_kind, clue, owner_team_id)) = pending_answer.take() {
                    let action = match pending_kind {
                        AnswerFlash::Correct => GameAction::AnswerCorrect {
                            clue,
                            team_id: owner_team_id,
                        },
                        AnswerFlash::Incorrect => GameAction::AnswerIncorrect {
                            clue,
                            team_id: owner_team_id,
                        },
                    };
                    if let Ok(result) = game_engine.handle_action(action) {
                        match result {
                            GameActionResult::Success { new_phase } => {
                                requested_phase = Some(new_phase)
                            }
                            GameActionResult::StateChanged {
                                new_phase, effects, ..
                            } => {
                                requested_phase = Some(new_phase);
                                // Effects already represented visually by animation; nothing extra for now
                                let _ = effects; // suppress unused warning if any
                            }
                            _ => {}
                        }
                    }
                }

                // Handle pending steal actions after animation completes
                if let Some((pending_outcome, clue, team_id)) = pending_steal.take() {
                    let correct = matches!(pending_outcome, StealOutcome::Correct);
                    let action = GameAction::StealAttempt {
                        clue,
                        team_id,
                        correct,
                    };
                    if let Ok(result) = game_engine.handle_action(action) {
                        match result {
                            GameActionResult::Success { new_phase } => {
                                requested_phase = Some(new_phase)
                            }
                            GameActionResult::StateChanged {
                                new_phase, effects, ..
                            } => {
                                requested_phase = Some(new_phase);
                                // Effects already represented visually by animation; nothing extra for now
                                let _ = effects; // suppress unused warning if any
                            }
                            _ => {}
                        }
                    }
                }

                ui.memory_mut(|m| {
                    m.data.remove::<Option<(AnswerFlash, Instant)>>(flash_id);
                    m.data
                        .remove::<Option<(AnswerFlash, (usize, usize), u32)>>(pending_answer_id);
                    m.data
                        .remove::<Option<(StealOutcome, (usize, usize), u32)>>(pending_steal_id);
                });
            }
        } else {
            ui.memory_mut(|m| m.data.remove::<Option<(AnswerFlash, Instant)>>(flash_id));
        }

        // Handle event animations
        if let Some(mut controller) = event_animation.take() {
            if controller.update() {
                // Animation completed - reset animation playing state
                game_engine
                    .get_state_mut()
                    .event_state
                    .set_animation_playing(false);
                event_animation = None;
            } else {
                // Animation still running, render it
                if let Some(animation_type) = controller.get_animation_type() {
                    let elapsed = controller.animation_start.elapsed();
                    let t = (elapsed.as_secs_f32() / controller.animation_duration.as_secs_f32())
                        .clamp(0.0, 1.0);

                    let ctx = ui.ctx();
                    let rect = ctx.screen_rect();
                    egui::Area::new("event_animation_overlay".into())
                        .order(egui::Order::Foreground)
                        .movable(false)
                        .interactable(false)
                        .fixed_pos(rect.min)
                        .show(ctx, |ui| {
                            let painter = ui.painter_at(rect);
                            match animation_type {
                                EventAnimationType::DoublePointsMultiplication => {
                                    draw_double_points_animation(&painter, rect, t);
                                }
                                EventAnimationType::HardResetGlitch => {
                                    draw_hard_reset_animation(&painter, rect, t);
                                }
                                EventAnimationType::ReverseQuestionFlip => {
                                    draw_reverse_question_animation(&painter, rect, t);
                                }
                            }
                        });

                    ctx.request_repaint();
                    event_animation = Some(controller);
                }
            }
        }

        // Check for new event animations from game effects
        // This would be triggered by GameEffect::EventAnimation
        if event_animation.is_none() {
            // Check if there's a queued event that should trigger an animation
            if game_engine.get_state().event_state.has_queued_event()
                && !game_engine.get_state().event_state.is_animation_playing()
            {
                if let Some(queued_event) = game_engine.get_state().event_state.queued_event.clone()
                {
                    let mut controller = EventAnimationController::new();
                    let duration = match queued_event {
                        GameEvent::DoublePoints => Duration::from_millis(3000),
                        GameEvent::HardReset => Duration::from_millis(4000),
                        GameEvent::ReverseQuestion => Duration::from_millis(2500),
                    };
                    controller.start_animation(queued_event.clone(), duration);

                    // Mark animation as playing and consume the queued event
                    game_engine
                        .get_state_mut()
                        .event_state
                        .set_animation_playing(true);
                    let _ = game_engine.get_state_mut().event_state.take_queued_event();

                    // For non-Hard Reset events, activate them now for the next cell
                    if !matches!(queued_event, GameEvent::HardReset) {
                        game_engine
                            .get_state_mut()
                            .event_state
                            .activate_event(queued_event);
                    }

                    event_animation = Some(controller);
                }
            }
        }

        // Persist pending answer and steal if still waiting (flash active)
        if pending_answer.is_some() {
            ui.memory_mut(|m| m.data.insert_temp(pending_answer_id, pending_answer));
        }
        if pending_steal.is_some() {
            ui.memory_mut(|m| m.data.insert_temp(pending_steal_id, pending_steal));
        }
        if let Some(controller) = event_animation {
            ui.memory_mut(|m| m.data.insert_temp(event_animation_id, Some(controller)));
        } else {
            ui.memory_mut(|m| {
                m.data
                    .remove::<Option<EventAnimationController>>(event_animation_id)
            });
        }
    });
    next_mode
}

fn draw_showing_overlay(
    ctx: &egui::Context,
    game_engine: &mut GameEngine,
    clue: (usize, usize),
    owner_team_id: u32,
    flash: &mut Option<(AnswerFlash, Instant)>,
    _requested_phase: &mut Option<PlayPhase>,
    pending_answer: &mut Option<(AnswerFlash, (usize, usize), u32)>,
) {
    let screen = ctx.screen_rect();
    egui::Area::new("question_full_overlay".into())
        .order(egui::Order::Foreground)
        .movable(false)
        .interactable(true)
        .fixed_pos(screen.min)
        .show(ctx, |ui| {
            let rect = screen;
            let painter = ui.painter_at(rect);

            // Subtle modal background for dialogue
            paint_subtle_modal_background(&painter, rect);

            let (question, points) = game_engine
                .get_state()
                .board
                .categories
                .get(clue.0)
                .and_then(|cat| cat.clues.get(clue.1))
                .map(|c| (c.question.clone(), c.points))
                .unwrap_or_default();

            ui.allocate_ui_with_layout(
                rect.size(),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    ui.add_space(50.0);

                    // Enhanced points display with amber glow
                    ui.heading(
                        egui::RichText::new(format!("{} pts", points))
                            .color(Palette::AMBER_GLOW)
                            .size(36.0),
                    );

                    ui.add_space(30.0);

                    // Enhanced question text with yellow styling
                    let wrap_width = rect.width() * 0.85;
                    let label = egui::Label::new(
                        egui::RichText::new(question)
                            .size(30.0)
                            .color(Palette::CYBER_YELLOW),
                    )
                    .wrap(true)
                    .truncate(false);
                    ui.add_sized([wrap_width, 0.0], label);
                },
            );

            // Enhanced button area
            let bottom_h = 120.0;
            let bottom_rect = egui::Rect::from_min_size(
                egui::pos2(rect.left(), rect.bottom() - bottom_h - 20.0),
                egui::vec2(rect.width(), bottom_h),
            );

            ui.allocate_ui_at_rect(bottom_rect, |ui| {
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        ui.set_width(bottom_rect.width());
                        ui.horizontal(|ui| {
                            // Block interactions during flash animation
                            let interaction_blocked = flash.is_some() || pending_answer.is_some();

                            if enhanced_modal_button(ui, "Correct", ModalButtonType::Correct)
                                .clicked()
                                && !interaction_blocked
                            {
                                // Start animation first; delay state mutation until animation completes
                                if flash.is_none() && pending_answer.is_none() {
                                    *flash = Some((AnswerFlash::Correct, Instant::now()));
                                    *pending_answer =
                                        Some((AnswerFlash::Correct, clue, owner_team_id));
                                }
                            }

                            ui.add_space(40.0);

                            if enhanced_modal_button(ui, "Incorrect", ModalButtonType::Incorrect)
                                .clicked()
                                && !interaction_blocked
                            {
                                if flash.is_none() && pending_answer.is_none() {
                                    *flash = Some((AnswerFlash::Incorrect, Instant::now()));
                                    *pending_answer =
                                        Some((AnswerFlash::Incorrect, clue, owner_team_id));
                                }
                            }
                        });
                    },
                );
            });
        });
}

fn draw_steal_overlay(
    ctx: &egui::Context,
    question: &str,
    points: u32,
    team_name: &str,
    has_more_contenders: bool,
    flash: &mut Option<(AnswerFlash, Instant)>,
    pending_steal: &mut Option<(StealOutcome, (usize, usize), u32)>,
) -> Option<StealOutcome> {
    let mut outcome = None;
    let screen = ctx.screen_rect();
    egui::Area::new("steal_full_overlay".into())
        .order(egui::Order::Foreground)
        .movable(false)
        .interactable(true)
        .fixed_pos(screen.min)
        .show(ctx, |ui| {
            let rect = screen;
            let painter = ui.painter_at(rect);
            // Subtle modal background for dialogue
            paint_subtle_modal_background(&painter, rect);
            ui.allocate_ui_with_layout(
                rect.size(),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    ui.add_space(24.0);
                    ui.heading(
                        egui::RichText::new(format!("{} pts", points)).color(Palette::AMBER_GLOW),
                    );
                    ui.add_space(12.0);
                    let wrap_width = rect.width() * 0.9;
                    let label = egui::Label::new(egui::RichText::new(question).size(26.0))
                        .wrap(true)
                        .truncate(false);
                    ui.add_sized([wrap_width, 0.0], label);
                    ui.add_space(8.0);
                    let steal_info = if has_more_contenders {
                        format!("Steal Attempt: {}", team_name)
                    } else {
                        format!("Final Attempt: {}", team_name)
                    };
                    ui.label(egui::RichText::new(steal_info).size(20.0));
                },
            );
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
                            // Block interactions during flash animation
                            let interaction_blocked = flash.is_some() || pending_steal.is_some();

                            if enhanced_modal_button(ui, "Correct", ModalButtonType::Correct)
                                .clicked()
                                && !interaction_blocked
                            {
                                // Start animation first; delay state mutation until animation completes
                                if flash.is_none() && pending_steal.is_none() {
                                    *flash = Some((AnswerFlash::Correct, Instant::now()));
                                    outcome = Some(StealOutcome::Correct);
                                }
                            }

                            ui.add_space(40.0);

                            if enhanced_modal_button(ui, "Incorrect", ModalButtonType::Incorrect)
                                .clicked()
                                && !interaction_blocked
                            {
                                if flash.is_none() && pending_steal.is_none() {
                                    *flash = Some((AnswerFlash::Incorrect, Instant::now()));
                                    outcome = Some(StealOutcome::Incorrect);
                                }
                            }
                        });
                    },
                );
            });
        });
    outcome
}

fn draw_resolved_overlay(
    ctx: &egui::Context,
    game_engine: &mut GameEngine,
    clue: (usize, usize),
    next_team_id: u32,
    requested_phase: &mut Option<PlayPhase>,
    flash: &Option<(AnswerFlash, Instant)>,
) {
    let screen = ctx.screen_rect();
    egui::Area::new("resolved_full_overlay".into())
        .order(egui::Order::Foreground)
        .movable(false)
        .interactable(true)
        .fixed_pos(screen.min)
        .show(ctx, |ui| {
            let rect = screen;
            let painter = ui.painter_at(rect);

            // Subtle modal background for dialogue
            paint_subtle_modal_background(&painter, rect);

            let (question, answer, points) = game_engine
                .get_state()
                .board
                .categories
                .get(clue.0)
                .and_then(|cat| cat.clues.get(clue.1))
                .map(|c| (c.question.clone(), c.answer.clone(), c.points))
                .unwrap_or((String::new(), String::new(), 0));

            ui.allocate_ui_with_layout(
                rect.size(),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    ui.add_space(40.0);

                    // Enhanced points display with amber styling
                    ui.heading(
                        egui::RichText::new(format!("{} pts", points))
                            .color(Palette::AMBER_GLOW)
                            .size(32.0),
                    );

                    ui.add_space(25.0);

                    // Enhanced question text with yellow styling
                    let wrap_width = rect.width() * 0.85;
                    let q_label = egui::Label::new(
                        egui::RichText::new(question)
                            .size(26.0)
                            .color(Palette::CYBER_YELLOW),
                    )
                    .wrap(true)
                    .truncate(false);
                    ui.add_sized([wrap_width, 0.0], q_label);

                    ui.add_space(20.0);

                    // Enhanced answer text with amber styling
                    let a_label = egui::Label::new(
                        egui::RichText::new(format!("Answer: {}", answer))
                            .color(Palette::AMBER_GLOW)
                            .size(24.0)
                            .strong(),
                    )
                    .wrap(true)
                    .truncate(false);
                    ui.add_sized([wrap_width, 0.0], a_label);
                },
            );

            // Enhanced button area
            let bottom_h = 100.0;
            let bottom_rect = egui::Rect::from_min_size(
                egui::pos2(rect.left(), rect.bottom() - bottom_h - 20.0),
                egui::vec2(rect.width(), bottom_h),
            );

            ui.allocate_ui_at_rect(bottom_rect, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.set_width(bottom_rect.width());
                    ui.horizontal_centered(|ui| {
                        // Block interactions during flash animation (in case flash is still playing from previous phase)
                        let interaction_blocked = flash.is_some();

                        if enhanced_modal_button(ui, "Close", ModalButtonType::Close).clicked()
                            && !interaction_blocked
                        {
                            let action = GameAction::CloseClue { clue, next_team_id };
                            if let Ok(result) = game_engine.handle_action(action) {
                                match result {
                                    GameActionResult::Success { new_phase } => {
                                        *requested_phase = Some(new_phase)
                                    }
                                    GameActionResult::StateChanged { new_phase, .. } => {
                                        *requested_phase = Some(new_phase)
                                    }
                                    _ => {}
                                }
                            }
                            ui.ctx().request_repaint();
                        }
                    });
                });
            });
        });
}

fn draw_success_animation(painter: &egui::Painter, rect: egui::Rect, t: f32) {
    let center = rect.center();

    // Easing function for smooth animation curves
    let ease_out_bounce = |t: f32| -> f32 {
        if t < 1.0 / 2.75 {
            7.5625 * t * t
        } else if t < 2.0 / 2.75 {
            let t = t - 1.5 / 2.75;
            7.5625 * t * t + 0.75
        } else if t < 2.5 / 2.75 {
            let t = t - 2.25 / 2.75;
            7.5625 * t * t + 0.9375
        } else {
            let t = t - 2.625 / 2.75;
            7.5625 * t * t + 0.984375
        }
    };

    let ease_out = 1.0 - (1.0 - t).powf(3.0);

    // Base green overlay with smooth fade
    let alpha = ((1.0 - ease_out) * 180.0) as u8;
    let base_color = egui::Color32::from_rgba_unmultiplied(0, 255, 170, alpha);
    painter.rect_filled(rect, 0.0, base_color);

    // Multiple expanding rings with different speeds and colors
    for i in 0..4 {
        let ring_t = (t * 1.5 - i as f32 * 0.15).clamp(0.0, 1.0);
        if ring_t > 0.0 {
            let ring_alpha = ((1.0 - ring_t) * 120.0) as u8;
            let ring_radius =
                ease_out_bounce(ring_t) * (rect.width().min(rect.height()) * 0.7) + i as f32 * 20.0;
            let ring_color = match i {
                0 => egui::Color32::from_rgba_unmultiplied(0, 255, 170, ring_alpha),
                1 => egui::Color32::from_rgba_unmultiplied(100, 255, 200, ring_alpha),
                2 => egui::Color32::from_rgba_unmultiplied(200, 255, 220, ring_alpha),
                _ => egui::Color32::from_rgba_unmultiplied(255, 255, 255, ring_alpha / 2),
            };
            painter.circle_stroke(
                center,
                ring_radius,
                egui::Stroke::new(8.0 - i as f32 * 1.5, ring_color),
            );
        }
    }

    // Radiating success lines/burst effect
    let line_count = 12;
    for i in 0..line_count {
        let angle = (i as f32 / line_count as f32) * 2.0 * std::f32::consts::PI;
        let line_t = (t * 2.0 - 0.3).clamp(0.0, 1.0);
        if line_t > 0.0 {
            let length = ease_out * rect.width().min(rect.height()) * 0.4;
            let start_radius = length * 0.3;
            let end_radius = length;

            let start = center + egui::Vec2::angled(angle) * start_radius;
            let end = center + egui::Vec2::angled(angle) * end_radius;

            let line_alpha = ((1.0 - line_t) * 200.0) as u8;
            let line_color = egui::Color32::from_rgba_unmultiplied(0, 255, 170, line_alpha);
            painter.line_segment([start, end], egui::Stroke::new(4.0, line_color));
        }
    }

    // Sparkling particles
    for i in 0..8 {
        let particle_t = (t * 1.8 - i as f32 * 0.1).clamp(0.0, 1.0);
        if particle_t > 0.0 {
            let angle = (i as f32 / 8.0) * 2.0 * std::f32::consts::PI + t * 0.5;
            let radius = ease_out * (rect.width().min(rect.height()) * 0.3);
            let pos = center + egui::Vec2::angled(angle) * radius;

            let particle_alpha = ((1.0 - particle_t) * 255.0) as u8;
            let particle_size = (1.0 - particle_t) * 8.0 + 2.0;
            let particle_color =
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, particle_alpha);
            painter.circle_filled(pos, particle_size, particle_color);
        }
    }

    // Sound wave ripples for audio feedback visualization
    for i in 0..3 {
        let wave_t = (t * 2.5 - i as f32 * 0.3).clamp(0.0, 1.0);
        if wave_t > 0.0 {
            let wave_radius = wave_t * rect.width().min(rect.height()) * 0.6;
            let wave_alpha = ((1.0 - wave_t) * 80.0) as u8;
            let wave_color = egui::Color32::from_rgba_unmultiplied(0, 255, 170, wave_alpha);
            painter.circle_stroke(center, wave_radius, egui::Stroke::new(2.0, wave_color));
        }
    }
}

fn draw_failure_animation(painter: &egui::Painter, rect: egui::Rect, t: f32) {
    // Reuse success animation structure but swap to red palette
    let center = rect.center();
    let ease_out_bounce = |t: f32| -> f32 {
        if t < 1.0 / 2.75 {
            7.5625 * t * t
        } else if t < 2.0 / 2.75 {
            let t = t - 1.5 / 2.75;
            7.5625 * t * t + 0.75
        } else if t < 2.5 / 2.75 {
            let t = t - 2.25 / 2.75;
            7.5625 * t * t + 0.9375
        } else {
            let t = t - 2.625 / 2.75;
            7.5625 * t * t + 0.984375
        }
    };
    let ease_out = 1.0 - (1.0 - t).powf(3.0);
    let alpha = ((1.0 - ease_out) * 180.0) as u8;
    let base_color = egui::Color32::from_rgba_unmultiplied(255, 40, 80, alpha);
    painter.rect_filled(rect, 0.0, base_color);
    for i in 0..4 {
        let ring_t = (t * 1.5 - i as f32 * 0.15).clamp(0.0, 1.0);
        if ring_t > 0.0 {
            let ring_alpha = ((1.0 - ring_t) * 120.0) as u8;
            let ring_radius =
                ease_out_bounce(ring_t) * (rect.width().min(rect.height()) * 0.7) + i as f32 * 20.0;
            let ring_color = match i {
                0 => egui::Color32::from_rgba_unmultiplied(255, 40, 80, ring_alpha),
                1 => egui::Color32::from_rgba_unmultiplied(255, 120, 140, ring_alpha),
                2 => egui::Color32::from_rgba_unmultiplied(255, 200, 210, ring_alpha),
                _ => egui::Color32::from_rgba_unmultiplied(255, 255, 255, ring_alpha / 2),
            };
            painter.circle_stroke(
                center,
                ring_radius,
                egui::Stroke::new(8.0 - i as f32 * 1.5, ring_color),
            );
        }
    }
    let line_count = 12;
    for i in 0..line_count {
        let angle = (i as f32 / line_count as f32) * 2.0 * std::f32::consts::PI;
        let line_t = (t * 2.0 - 0.3).clamp(0.0, 1.0);
        if line_t > 0.0 {
            let length = ease_out * rect.width().min(rect.height()) * 0.4;
            let start_radius = length * 0.3;
            let end_radius = length;
            let start = center + egui::Vec2::angled(angle) * start_radius;
            let end = center + egui::Vec2::angled(angle) * end_radius;
            let line_alpha = ((1.0 - line_t) * 200.0) as u8;
            let line_color = egui::Color32::from_rgba_unmultiplied(255, 40, 80, line_alpha);
            painter.line_segment([start, end], egui::Stroke::new(4.0, line_color));
        }
    }
    for i in 0..8 {
        let particle_t = (t * 1.8 - i as f32 * 0.1).clamp(0.0, 1.0);
        if particle_t > 0.0 {
            let angle = (i as f32 / 8.0) * 2.0 * std::f32::consts::PI + t * 0.5;
            let radius = ease_out * (rect.width().min(rect.height()) * 0.3);
            let pos = center + egui::Vec2::angled(angle) * radius;
            let particle_alpha = ((1.0 - particle_t) * 255.0) as u8;
            let particle_size = (1.0 - particle_t) * 8.0 + 2.0;
            let particle_color =
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, particle_alpha);
            painter.circle_filled(pos, particle_size, particle_color);
        }
    }
    for i in 0..3 {
        let wave_t = (t * 2.5 - i as f32 * 0.3).clamp(0.0, 1.0);
        if wave_t > 0.0 {
            let wave_radius = wave_t * rect.width().min(rect.height()) * 0.6;
            let wave_alpha = ((1.0 - wave_t) * 80.0) as u8;
            let wave_color = egui::Color32::from_rgba_unmultiplied(255, 40, 80, wave_alpha);
            painter.circle_stroke(center, wave_radius, egui::Stroke::new(2.0, wave_color));
        }
    }
}
fn draw_double_points_animation(painter: &egui::Painter, rect: egui::Rect, t: f32) {
    let center = rect.center();

    // Easing functions
    let ease_out = 1.0 - (1.0 - t).powf(3.0);
    let ease_in_out = if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - 2.0 * (1.0 - t).powf(2.0)
    };

    // Cyan/blue color scheme with pulsing effects
    let base_alpha = ((0.7 - ease_out * 0.5) * 255.0) as u8;
    let base_color = egui::Color32::from_rgba_unmultiplied(0, 200, 255, base_alpha);
    painter.rect_filled(rect, 0.0, base_color);

    // Multiplication symbol (×2) in the center
    let text_size = 120.0 + ease_in_out * 40.0;
    let text_alpha = ((1.0 - ease_out * 0.3) * 255.0) as u8;
    let text_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, text_alpha);

    // Draw "×2" text
    let font_id = egui::FontId::proportional(text_size);
    let text = "×2";
    let galley = painter.layout_no_wrap(text.to_string(), font_id, text_color);
    let text_pos = center - galley.size() / 2.0;
    painter.galley(text_pos, galley, text_color);

    // Energy bursts around the multiplication symbol
    let burst_count = 8;
    for i in 0..burst_count {
        let angle = (i as f32 / burst_count as f32) * 2.0 * std::f32::consts::PI + t * 2.0;
        let burst_t = (t * 2.0 - i as f32 * 0.1).clamp(0.0, 1.0);

        if burst_t > 0.0 {
            let length = ease_out * 150.0;
            let start_radius = 80.0;
            let end_radius = start_radius + length;

            let start = center + egui::Vec2::angled(angle) * start_radius;
            let end = center + egui::Vec2::angled(angle) * end_radius;

            let burst_alpha = ((1.0 - burst_t) * 200.0) as u8;
            let burst_color = egui::Color32::from_rgba_unmultiplied(0, 255, 255, burst_alpha);
            painter.line_segment([start, end], egui::Stroke::new(6.0, burst_color));
        }
    }

    // Pulsing rings
    for i in 0..3 {
        let ring_t = (t * 1.5 - i as f32 * 0.2).clamp(0.0, 1.0);
        if ring_t > 0.0 {
            let ring_radius = ease_out * (200.0 + i as f32 * 50.0);
            let ring_alpha = ((1.0 - ring_t) * 150.0) as u8;
            let ring_color = match i {
                0 => egui::Color32::from_rgba_unmultiplied(0, 255, 255, ring_alpha),
                1 => egui::Color32::from_rgba_unmultiplied(100, 200, 255, ring_alpha),
                _ => egui::Color32::from_rgba_unmultiplied(200, 220, 255, ring_alpha),
            };
            painter.circle_stroke(center, ring_radius, egui::Stroke::new(4.0, ring_color));
        }
    }

    // Scaling point value particles
    for i in 0..12 {
        let particle_t = (t * 2.0 - i as f32 * 0.05).clamp(0.0, 1.0);
        if particle_t > 0.0 {
            let angle = (i as f32 / 12.0) * 2.0 * std::f32::consts::PI;
            let radius = ease_out * 250.0;
            let pos = center + egui::Vec2::angled(angle) * radius;

            let particle_alpha = ((1.0 - particle_t) * 255.0) as u8;
            let particle_size = (1.0 - particle_t) * 12.0 + 4.0;
            let particle_color =
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, particle_alpha);
            painter.circle_filled(pos, particle_size, particle_color);
        }
    }
}

fn draw_hard_reset_animation(painter: &egui::Painter, rect: egui::Rect, t: f32) {
    let center = rect.center();

    // Easing functions
    let ease_out = 1.0 - (1.0 - t).powf(3.0);
    let ease_in_out = if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - 2.0 * (1.0 - t).powf(2.0)
    };

    // Red error colors transitioning to normal
    let base_alpha = if t < 0.7 {
        ((0.8 - t * 0.5) * 255.0) as u8
    } else {
        ((0.8 - 0.7 * 0.5) * (1.0 - (t - 0.7) / 0.3) * 255.0) as u8
    };
    let base_color = egui::Color32::from_rgba_unmultiplied(255, 0, 50, base_alpha);
    painter.rect_filled(rect, 0.0, base_color);

    // Screen glitching effect
    if t < 0.6 {
        let glitch_intensity = (0.6 - t) / 0.6;
        for i in 0..20 {
            let y = (i as f32 / 20.0) * rect.height() + rect.min.y;
            let glitch_offset = (glitch_intensity * 50.0 * (t * 10.0 + i as f32).sin()) as f32;
            let glitch_rect = egui::Rect::from_min_size(
                egui::Pos2::new(rect.min.x + glitch_offset, y),
                egui::Vec2::new(rect.width(), rect.height() / 20.0),
            );
            let glitch_alpha = (glitch_intensity * 100.0) as u8;
            let glitch_color = egui::Color32::from_rgba_unmultiplied(255, 100, 100, glitch_alpha);
            painter.rect_filled(glitch_rect, 0.0, glitch_color);
        }
    }

    // "RESET" text with glitch effect
    let text_size = 100.0 + ease_in_out * 20.0;
    let text_alpha = ((1.0 - ease_out * 0.2) * 255.0) as u8;
    let text_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, text_alpha);

    let font_id = egui::FontId::proportional(text_size);
    let text = "RESET";
    let galley = painter.layout_no_wrap(text.to_string(), font_id, text_color);
    let text_pos = center - galley.size() / 2.0;
    painter.galley(text_pos, galley, text_color);

    // Digital artifacts and static
    for i in 0..30 {
        let artifact_t = (t * 3.0 - i as f32 * 0.05).clamp(0.0, 1.0);
        if artifact_t > 0.0 {
            let x = (i as f32 * 123.456).fract() * rect.width() + rect.min.x;
            let y = (i as f32 * 789.123).fract() * rect.height() + rect.min.y;
            let size = (1.0 - artifact_t) * 8.0 + 2.0;

            let artifact_alpha = ((1.0 - artifact_t) * 200.0) as u8;
            let artifact_color =
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, artifact_alpha);
            painter.rect_filled(
                egui::Rect::from_center_size(egui::Pos2::new(x, y), egui::Vec2::splat(size)),
                0.0,
                artifact_color,
            );
        }
    }

    // System reboot sequence lines
    if t > 0.3 {
        let line_t = ((t - 0.3) / 0.7).clamp(0.0, 1.0);
        for i in 0..5 {
            let line_progress = (line_t * 5.0 - i as f32).clamp(0.0, 1.0);
            if line_progress > 0.0 {
                let y = center.y + (i as f32 - 2.0) * 30.0;
                let line_width = line_progress * rect.width() * 0.8;
                let line_start = egui::Pos2::new(center.x - line_width / 2.0, y);
                let line_end = egui::Pos2::new(center.x + line_width / 2.0, y);

                let line_alpha = (line_progress * 255.0) as u8;
                let line_color = egui::Color32::from_rgba_unmultiplied(0, 255, 100, line_alpha);
                painter.line_segment([line_start, line_end], egui::Stroke::new(3.0, line_color));
            }
        }
    }
}

fn draw_reverse_question_animation(painter: &egui::Painter, rect: egui::Rect, t: f32) {
    let center = rect.center();

    // Easing functions
    let ease_out = 1.0 - (1.0 - t).powf(3.0);
    let ease_in_out = if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - 2.0 * (1.0 - t).powf(2.0)
    };

    // Purple/magenta color scheme
    let base_alpha = ((0.6 - ease_out * 0.3) * 255.0) as u8;
    let base_color = egui::Color32::from_rgba_unmultiplied(150, 0, 255, base_alpha);
    painter.rect_filled(rect, 0.0, base_color);

    // Flowing data streams
    for i in 0..8 {
        let stream_t = (t * 2.0 - i as f32 * 0.1).clamp(0.0, 1.0);
        if stream_t > 0.0 {
            let angle = (i as f32 / 8.0) * 2.0 * std::f32::consts::PI;
            let stream_length = ease_out * 300.0;

            for j in 0..10 {
                let segment_t = (stream_t * 10.0 - j as f32).clamp(0.0, 1.0);
                if segment_t > 0.0 {
                    let radius = 100.0 + j as f32 * 20.0;
                    let pos = center + egui::Vec2::angled(angle + t * 0.5) * radius;

                    let segment_alpha = (segment_t * 150.0) as u8;
                    let segment_size = segment_t * 6.0 + 2.0;
                    let segment_color =
                        egui::Color32::from_rgba_unmultiplied(255, 100, 255, segment_alpha);
                    painter.circle_filled(pos, segment_size, segment_color);
                }
            }
        }
    }

    // Flipping text effect - show "?" and "!" symbols
    let flip_progress = ease_in_out;
    let text_size = 80.0;
    let text_alpha = ((1.0 - ease_out * 0.2) * 255.0) as u8;
    let text_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, text_alpha);

    // Rotation effect for the symbols
    let rotation = flip_progress * std::f32::consts::PI;

    let font_id = egui::FontId::proportional(text_size);
    let question_text = "?";
    let exclamation_text = "!";

    // Draw question mark (fading out)
    if flip_progress < 0.5 {
        let q_alpha = ((1.0 - flip_progress * 2.0) * text_alpha as f32) as u8;
        let q_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, q_alpha);
        let galley = painter.layout_no_wrap(question_text.to_string(), font_id.clone(), q_color);
        let text_pos = center - galley.size() / 2.0;
        painter.galley(text_pos, galley, q_color);
    }

    // Draw exclamation mark (fading in)
    if flip_progress > 0.5 {
        let e_alpha = (((flip_progress - 0.5) * 2.0) * text_alpha as f32) as u8;
        let e_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, e_alpha);
        let galley = painter.layout_no_wrap(exclamation_text.to_string(), font_id, e_color);
        let text_pos = center - galley.size() / 2.0;
        painter.galley(text_pos, galley, e_color);
    }

    // Holographic distortion effects
    for i in 0..6 {
        let distortion_t = (t * 1.8 - i as f32 * 0.15).clamp(0.0, 1.0);
        if distortion_t > 0.0 {
            let angle = (i as f32 / 6.0) * 2.0 * std::f32::consts::PI + t * 1.5;
            let radius = ease_out * (150.0 + i as f32 * 30.0);
            let pos = center + egui::Vec2::angled(angle) * radius;

            let distortion_alpha = ((1.0 - distortion_t) * 120.0) as u8;
            let distortion_size = (1.0 - distortion_t) * 15.0 + 5.0;
            let distortion_color =
                egui::Color32::from_rgba_unmultiplied(255, 0, 255, distortion_alpha);
            painter.circle_stroke(
                pos,
                distortion_size,
                egui::Stroke::new(2.0, distortion_color),
            );
        }
    }

    // Mirror effects - vertical lines that simulate reflection
    for i in 0..10 {
        let mirror_t = (t * 2.5 - i as f32 * 0.1).clamp(0.0, 1.0);
        if mirror_t > 0.0 {
            let x = rect.min.x + (i as f32 / 10.0) * rect.width();
            let line_height = mirror_t * rect.height();
            let line_start = egui::Pos2::new(x, center.y - line_height / 2.0);
            let line_end = egui::Pos2::new(x, center.y + line_height / 2.0);

            let mirror_alpha = ((1.0 - mirror_t) * 80.0) as u8;
            let mirror_color = egui::Color32::from_rgba_unmultiplied(200, 100, 255, mirror_alpha);
            painter.line_segment([line_start, line_end], egui::Stroke::new(1.0, mirror_color));
        }
    }
}
