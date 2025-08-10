use eframe::egui;

use crate::app::AppMode;
use crate::domain::Board;
use crate::game::{GameEngine, PlayPhase, GameAction, GameActionResult, GameEffect, FlashType};
use crate::theme::Palette;
use crate::ui::{paint_enhanced_clue_cell, paint_enhanced_category_header, paint_enhanced_modal_background};
use crate::theme::{enhanced_modal_button, ModalButtonType, adjust_brightness};


use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
enum AnswerFlash { Correct, Incorrect }

#[derive(Clone, Copy, PartialEq)]
enum StealOutcome { Correct, Incorrect }

pub fn show(ctx: &egui::Context, game_engine: &mut GameEngine) -> Option<AppMode> {
    egui::SidePanel::left("teams")
        .frame(crate::theme::panel_frame())
        .show(ctx, |ui| {
            ui.heading(egui::RichText::new("Teams").color(Palette::CYAN));
            let in_lobby = matches!(game_engine.get_phase(), PlayPhase::Lobby);
            for team in &mut game_engine.get_state_mut().teams {
                ui.horizontal(|ui| {
                    if in_lobby { ui.add(egui::TextEdit::singleline(&mut team.name)); ui.label(format!(" — {}", team.score)); }
                    else { ui.label(format!("{} — {}", team.name, team.score)); }
                });
            }
            if crate::theme::accent_button(ui, "Add Team").clicked() { 
                let action = GameAction::AddTeam { name: format!("Team {}", game_engine.team_count() + 1) };
                let _ = game_engine.handle_action(action);
            }
        });

    let mut next_mode: Option<AppMode> = None;
    egui::CentralPanel::default().show(ctx, |ui| {
        crate::theme::paint_board_background(ui);
        ui.heading(egui::RichText::new("Game Board").color(Palette::CYAN));
        let mut requested_phase: Option<PlayPhase> = None;
        let flash_id = ui.id().with("answer_flash");
        let mut flash: Option<(AnswerFlash, Instant)> = ui.memory_mut(|m| m.data.get_temp(flash_id)).unwrap_or(None);

        match game_engine.get_phase() {
            PlayPhase::Lobby => {
                ui.label("Lobby: Add teams and press Start");
                if crate::theme::accent_button(ui, "Start").clicked() {
                    let action = GameAction::StartGame;
                    if let Ok(result) = game_engine.handle_action(action) {
                        match result {
                            GameActionResult::Success { new_phase } => requested_phase = Some(new_phase),
                            GameActionResult::StateChanged { new_phase, .. } => requested_phase = Some(new_phase),
                            _ => {}
                        }
                    }
                }
            }
            PlayPhase::Selecting { team_id } => {
                ui.label(egui::RichText::new(format!("Selecting — Active Team: {}", team_id)).color(Palette::MAGENTA));
                let cols = game_engine.get_state().board.categories.len().max(1);
                let rows = game_engine.get_state().board.categories.get(0).map(|c| c.clues.len()).unwrap_or(0);
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
                let cell_h = if rows > 0 { (remaining_height / rows as f32).max(50.0) } else { 70.0 };
                ui.horizontal(|ui| {
                    ui.set_width(available.x);
                    for cat in &game_engine.get_state().board.categories {
                        let (rect, _) = ui.allocate_exact_size(egui::vec2(cell_w, header_h), egui::Sense::hover());
                        let painter = ui.painter_at(rect);
                        paint_enhanced_category_header(&painter, rect, &cat.name);
                    }
                });
                let mut clicked_clue: Option<(usize, usize)> = None;
                for r in 0..rows {
                    ui.horizontal(|ui| {
                        ui.set_width(available.x);
                        for (ci, cat) in game_engine.get_state().board.categories.iter().enumerate() {
                            let clue = &cat.clues[r];
                            let (rect, response) = ui.allocate_exact_size(egui::vec2(cell_w, cell_h), egui::Sense::click());
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
                    let action = GameAction::SelectClue { clue, team_id: *team_id };
                    if let Ok(result) = game_engine.handle_action(action) {
                        match result {
                            GameActionResult::Success { new_phase } => requested_phase = Some(new_phase),
                            GameActionResult::StateChanged { new_phase, .. } => requested_phase = Some(new_phase),
                            _ => {}
                        }
                    }
                }
            }
            PlayPhase::Showing { clue, owner_team_id } => { draw_showing_overlay(ctx, game_engine, *clue, *owner_team_id, &mut flash, &mut requested_phase); }
            PlayPhase::Steal { clue, queue: _, current, owner_team_id: _ } => {
                let current_team_id = *current; 
                let has_more = if let PlayPhase::Steal { queue, .. } = &game_engine.get_state().phase {
                    !queue.is_empty()
                } else {
                    false
                };
                // Precompute immutable data needed for overlay
                let (question, points) = game_engine.get_state().board.categories.get(clue.0).and_then(|cat| cat.clues.get(clue.1)).map(|c| (c.question.clone(), c.points)).unwrap_or_default();
                let team_name = game_engine.get_state().teams.iter().find(|t| t.id == current_team_id).map(|t| t.name.clone()).unwrap_or_else(|| format!("#{}", current_team_id));
                if let Some(outcome) = draw_steal_overlay(ctx, &question, points, &team_name, has_more) {
                    match outcome {
                        StealOutcome::Correct => {
                            let action = GameAction::StealAttempt { clue: *clue, team_id: current_team_id, correct: true };
                            if let Ok(result) = game_engine.handle_action(action) {
                                match result {
                                    GameActionResult::Success { new_phase } => requested_phase = Some(new_phase),
                                    GameActionResult::StateChanged { new_phase, effects, .. } => {
                                        requested_phase = Some(new_phase);
                                        for effect in effects {
                                            if let GameEffect::FlashEffect { effect_type: FlashType::Correct } = effect {
                                                flash = Some((AnswerFlash::Correct, Instant::now()));
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        StealOutcome::Incorrect => {
                            let action = GameAction::StealAttempt { clue: *clue, team_id: current_team_id, correct: false };
                            if let Ok(result) = game_engine.handle_action(action) {
                                match result {
                                    GameActionResult::Success { new_phase } => requested_phase = Some(new_phase),
                                    GameActionResult::StateChanged { new_phase, effects, .. } => {
                                        requested_phase = Some(new_phase);
                                        for effect in effects {
                                            if let GameEffect::FlashEffect { effect_type: FlashType::Incorrect } = effect {
                                                flash = Some((AnswerFlash::Incorrect, Instant::now()));
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            PlayPhase::Resolved { clue, next_team_id } => { draw_resolved_overlay(ctx, game_engine, *clue, *next_team_id, &mut requested_phase); }
            PlayPhase::Intermission => { ui.label("Intermission"); }
            PlayPhase::Finished => { ui.label("Finished"); if crate::theme::secondary_button(ui, "Back to Config").clicked() { next_mode = Some(AppMode::Config(crate::domain::ConfigState { board: Board::default() })); } }
        }

        if let Some(p) = requested_phase { game_engine.get_state_mut().phase = p; ui.memory_mut(|m| m.data.remove::<Option<(AnswerFlash, Instant)>>(flash_id)); }

        if let Some((kind, start)) = flash { 
            let elapsed = start.elapsed(); 
            let duration = Duration::from_millis(1200); // Extended duration for more expressive animation
            if elapsed < duration { 
                let t = (elapsed.as_secs_f32() / duration.as_secs_f32()).clamp(0.0, 1.0);
                let rect = ui.max_rect();
                let painter = ui.painter();
                
                match kind {
                    AnswerFlash::Correct => {
                        // Success burst animation with multiple layers
                        draw_success_animation(&painter, rect, t);
                    }
                    AnswerFlash::Incorrect => {
                        // Failure shake and pulse animation
                        draw_failure_animation(&painter, rect, t);
                    }
                }
                
                ui.ctx().request_repaint(); 
                ui.memory_mut(|m| m.data.insert_temp(flash_id, Some((kind, start)))); 
            } else { 
                ui.memory_mut(|m| m.data.remove::<Option<(AnswerFlash, Instant)>>(flash_id)); 
            } 
        } else { 
            ui.memory_mut(|m| m.data.remove::<Option<(AnswerFlash, Instant)>>(flash_id)); 
        }
    });
    next_mode
}

fn draw_showing_overlay(ctx: &egui::Context, game_engine: &mut GameEngine, clue: (usize, usize), owner_team_id: u32, flash: &mut Option<(AnswerFlash, Instant)>, requested_phase: &mut Option<PlayPhase>) {
    let screen = ctx.screen_rect();
    egui::Area::new("question_full_overlay".into()).order(egui::Order::Foreground).movable(false).interactable(true).fixed_pos(screen.min).show(ctx, |ui| {
        let rect = screen;
        let painter = ui.painter_at(rect);
        
        // Enhanced modal background
        paint_enhanced_modal_background(&painter, rect);
        
        let (question, points) = game_engine.get_state().board.categories.get(clue.0).and_then(|cat| cat.clues.get(clue.1)).map(|c| (c.question.clone(), c.points)).unwrap_or_default();
        
        ui.allocate_ui_with_layout(rect.size(), egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.add_space(50.0);
            
            // Enhanced points display with glow
            ui.heading(egui::RichText::new(format!("{} pts", points))
                .color(adjust_brightness(Palette::CYAN, 1.3))
                .size(36.0));
            
            ui.add_space(30.0);
            
            // Enhanced question text
            let wrap_width = rect.width() * 0.85;
            let label = egui::Label::new(egui::RichText::new(question)
                .size(30.0)
                .color(adjust_brightness(Palette::TEXT, 1.1)))
                .wrap(true)
                .truncate(false);
            ui.add_sized([wrap_width, 0.0], label);
        });
        
        // Enhanced button area
        let bottom_h = 120.0;
        let bottom_rect = egui::Rect::from_min_size(
            egui::pos2(rect.left(), rect.bottom() - bottom_h - 20.0),
            egui::vec2(rect.width(), bottom_h),
        );
        
        ui.allocate_ui_at_rect(bottom_rect, |ui| {
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                ui.set_width(bottom_rect.width());
                ui.horizontal(|ui| {
                    if enhanced_modal_button(ui, "Correct", ModalButtonType::Correct).clicked() {
                        let action = GameAction::AnswerCorrect { clue, team_id: owner_team_id };
                        if let Ok(result) = game_engine.handle_action(action) {
                            match result {
                                GameActionResult::Success { new_phase } => *requested_phase = Some(new_phase),
                                GameActionResult::StateChanged { new_phase, effects, .. } => {
                                    *requested_phase = Some(new_phase);
                                    for effect in effects {
                                        if let GameEffect::FlashEffect { effect_type: FlashType::Correct } = effect {
                                            *flash = Some((AnswerFlash::Correct, Instant::now()));
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    
                    ui.add_space(40.0);
                    
                    if enhanced_modal_button(ui, "Incorrect", ModalButtonType::Incorrect).clicked() {
                        let action = GameAction::AnswerIncorrect { clue, team_id: owner_team_id };
                        if let Ok(result) = game_engine.handle_action(action) {
                            match result {
                                GameActionResult::Success { new_phase } => *requested_phase = Some(new_phase),
                                GameActionResult::StateChanged { new_phase, effects, .. } => {
                                    *requested_phase = Some(new_phase);
                                    for effect in effects {
                                        if let GameEffect::FlashEffect { effect_type: FlashType::Incorrect } = effect {
                                            *flash = Some((AnswerFlash::Incorrect, Instant::now()));
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                });
            });
        });
    });
}

fn draw_steal_overlay(ctx: &egui::Context, question: &str, points: u32, team_name: &str, has_more_contenders: bool) -> Option<StealOutcome> {
    let mut outcome = None; let screen = ctx.screen_rect();
    egui::Area::new("steal_full_overlay".into()).order(egui::Order::Foreground).movable(false).interactable(true).fixed_pos(screen.min).show(ctx, |ui| {
        let rect = screen; let painter = ui.painter_at(rect); painter.rect_filled(rect, 0.0, Palette::BG_DARK);
        ui.allocate_ui_with_layout(rect.size(), egui::Layout::top_down(egui::Align::Center), |ui| { ui.add_space(24.0); ui.heading(egui::RichText::new(format!("{} pts", points)).color(Palette::CYAN)); ui.add_space(12.0); let wrap_width = rect.width() * 0.9; let label = egui::Label::new(egui::RichText::new(question).size(26.0)).wrap(true).truncate(false); ui.add_sized([wrap_width,0.0], label); ui.add_space(8.0); let steal_info = if has_more_contenders { format!("Steal Attempt: {}", team_name) } else { format!("Final Attempt: {}", team_name) }; ui.label(egui::RichText::new(steal_info).size(20.0)); });
        let bottom_h = 90.0; let bottom_rect = egui::Rect::from_min_size(egui::pos2(rect.left(), rect.bottom() - bottom_h - 8.0), egui::vec2(rect.width(), bottom_h));
        ui.allocate_ui_at_rect(bottom_rect, |ui| { ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| { ui.set_width(bottom_rect.width()); ui.horizontal(|ui| { let correct_btn = egui::Button::new(egui::RichText::new("Correct").strong().color(egui::Color32::BLACK)).fill(Palette::CYAN).min_size(egui::vec2(160.0,44.0)); if ui.add(correct_btn).clicked() { outcome = Some(StealOutcome::Correct); } ui.add_space(24.0); let incorrect_btn = egui::Button::new(egui::RichText::new("Incorrect").strong().color(egui::Color32::WHITE)).fill(Palette::MAGENTA).min_size(egui::vec2(160.0,44.0)); if ui.add(incorrect_btn).clicked() { outcome = Some(StealOutcome::Incorrect); } }); }); });
    }); outcome }

fn draw_resolved_overlay(ctx: &egui::Context, game_engine: &mut GameEngine, clue: (usize, usize), next_team_id: u32, requested_phase: &mut Option<PlayPhase>) {
    let screen = ctx.screen_rect();
    egui::Area::new("resolved_full_overlay".into()).order(egui::Order::Foreground).movable(false).interactable(true).fixed_pos(screen.min).show(ctx, |ui| {
        let rect = screen;
        let painter = ui.painter_at(rect);
        
        // Enhanced modal background
        paint_enhanced_modal_background(&painter, rect);
        
        let (question, answer, points) = game_engine.get_state().board.categories.get(clue.0).and_then(|cat| cat.clues.get(clue.1)).map(|c| (c.question.clone(), c.answer.clone(), c.points)).unwrap_or((String::new(), String::new(), 0));
        
        ui.allocate_ui_with_layout(rect.size(), egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.add_space(40.0);
            
            // Enhanced points display
            ui.heading(egui::RichText::new(format!("{} pts", points))
                .color(adjust_brightness(Palette::CYAN, 1.3))
                .size(32.0));
            
            ui.add_space(25.0);
            
            // Enhanced question text
            let wrap_width = rect.width() * 0.85;
            let q_label = egui::Label::new(egui::RichText::new(question)
                .size(26.0)
                .color(adjust_brightness(Palette::TEXT, 1.1)))
                .wrap(true)
                .truncate(false);
            ui.add_sized([wrap_width, 0.0], q_label);
            
            ui.add_space(20.0);
            
            // Enhanced answer text with special styling
            let a_label = egui::Label::new(egui::RichText::new(format!("Answer: {}", answer))
                .color(adjust_brightness(Palette::MAGENTA, 1.2))
                .size(24.0)
                .strong())
                .wrap(true)
                .truncate(false);
            ui.add_sized([wrap_width, 0.0], a_label);
        });
        
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
                    if enhanced_modal_button(ui, "Close", ModalButtonType::Close).clicked() {
                        let action = GameAction::CloseClue { clue, next_team_id };
                        if let Ok(result) = game_engine.handle_action(action) {
                            match result {
                                GameActionResult::Success { new_phase } => *requested_phase = Some(new_phase),
                                GameActionResult::StateChanged { new_phase, .. } => *requested_phase = Some(new_phase),
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
            let ring_radius = ease_out_bounce(ring_t) * (rect.width().min(rect.height()) * 0.7) + i as f32 * 20.0;
            let ring_color = match i {
                0 => egui::Color32::from_rgba_unmultiplied(0, 255, 170, ring_alpha),
                1 => egui::Color32::from_rgba_unmultiplied(100, 255, 200, ring_alpha),
                2 => egui::Color32::from_rgba_unmultiplied(200, 255, 220, ring_alpha),
                _ => egui::Color32::from_rgba_unmultiplied(255, 255, 255, ring_alpha / 2),
            };
            painter.circle_stroke(center, ring_radius, egui::Stroke::new(8.0 - i as f32 * 1.5, ring_color));
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
            let particle_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, particle_alpha);
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
    let center = rect.center();
    
    // Shake effect - offset the entire animation slightly
    let shake_intensity = (1.0 - t) * 15.0;
    let shake_freq = 30.0; // High frequency for violent shake
    let shake_x = (t * shake_freq * 2.0 * std::f32::consts::PI).sin() * shake_intensity;
    let shake_y = (t * shake_freq * 1.7 * std::f32::consts::PI).cos() * shake_intensity * 0.7;
    let shaken_center = center + egui::Vec2::new(shake_x, shake_y);
    
    let ease_out = 1.0 - (1.0 - t).powf(2.0);
    
    // Pulsing red overlay
    let pulse = (t * 8.0 * std::f32::consts::PI).sin().abs();
    let alpha = ((1.0 - ease_out) * (120.0 + pulse * 100.0)) as u8;
    let base_color = egui::Color32::from_rgba_unmultiplied(255, 0, 150, alpha);
    
    // Apply shake to the background rect
    let shaken_rect = egui::Rect::from_center_size(shaken_center, rect.size());
    painter.rect_filled(shaken_rect, 0.0, base_color);
    
    // Angry jagged border effect
    let border_points = 20;
    for i in 0..border_points {
        let angle = (i as f32 / border_points as f32) * 2.0 * std::f32::consts::PI;
        let next_angle = ((i + 1) as f32 / border_points as f32) * 2.0 * std::f32::consts::PI;
        
        // Jagged radius variation
        let base_radius = rect.width().min(rect.height()) * 0.4;
        let jagged_variance = (i as f32 * 2.3).sin() * 20.0 * (1.0 - ease_out);
        let radius = base_radius + jagged_variance;
        let next_radius = base_radius + ((i + 1) as f32 * 2.3).sin() * 20.0 * (1.0 - ease_out);
        
        let start = shaken_center + egui::Vec2::angled(angle) * radius;
        let end = shaken_center + egui::Vec2::angled(next_angle) * next_radius;
        
        let border_alpha = ((1.0 - ease_out) * 255.0) as u8;
        let border_color = egui::Color32::from_rgba_unmultiplied(255, 0, 150, border_alpha);
        painter.line_segment([start, end], egui::Stroke::new(6.0, border_color));
    }
    
    // X marks and error symbols
    let x_size = 40.0 * (1.0 - ease_out);
    if x_size > 5.0 {
        let x_alpha = ((1.0 - ease_out) * 255.0) as u8;
        let x_color = egui::Color32::from_rgba_unmultiplied(255, 100, 100, x_alpha);
        let x_stroke = egui::Stroke::new(8.0, x_color);
        
        // Draw X in center
        painter.line_segment([
            shaken_center + egui::Vec2::new(-x_size, -x_size),
            shaken_center + egui::Vec2::new(x_size, x_size)
        ], x_stroke);
        painter.line_segment([
            shaken_center + egui::Vec2::new(-x_size, x_size),
            shaken_center + egui::Vec2::new(x_size, -x_size)
        ], x_stroke);
    }
    
    // Angry electrical-like zaps
    for i in 0..6 {
        let zap_t = (t * 3.0 - i as f32 * 0.2).clamp(0.0, 1.0);
        if zap_t > 0.0 && zap_t < 0.8 {
            let angle = (i as f32 / 6.0) * 2.0 * std::f32::consts::PI;
            let length = (1.0 - zap_t) * rect.width().min(rect.height()) * 0.3;
            
            let start = shaken_center;
            let end = start + egui::Vec2::angled(angle) * length;
            
            let zap_alpha = ((1.0 - zap_t) * 200.0) as u8;
            let zap_color = egui::Color32::from_rgba_unmultiplied(255, 50, 100, zap_alpha);
            painter.line_segment([start, end], egui::Stroke::new(3.0, zap_color));
        }
    }
    
    // Distorted sound waves for wrong answer audio feedback
    for i in 0..4 {
        let wave_t = (t * 2.2 - i as f32 * 0.2).clamp(0.0, 1.0);
        if wave_t > 0.0 {
            let base_radius = wave_t * rect.width().min(rect.height()) * 0.5;
            // Add distortion to make waves look "broken" or "wrong"
            let distortion = (wave_t * 15.0 + i as f32 * 3.0).sin() * 8.0;
            let wave_radius = base_radius + distortion;
            let wave_alpha = ((1.0 - wave_t) * 60.0) as u8;
            let wave_color = egui::Color32::from_rgba_unmultiplied(255, 0, 150, wave_alpha);
            painter.circle_stroke(shaken_center, wave_radius, egui::Stroke::new(1.5, wave_color));
        }
    }
}
