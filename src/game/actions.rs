use crate::core::Team;
use crate::game::events::{EventAnimationType, EventError, GameEvent, StealEventContext};
use crate::game::rules::GameRules;
use crate::game::scoring::ScoringEngine;
use crate::game::state::{GameState, PlayPhase};

/// Utility function to determine question value from clue coordinates
fn get_question_points(state: &GameState, clue: (usize, usize)) -> u32 {
    state
        .board
        .categories
        .get(clue.0)
        .and_then(|cat| cat.clues.get(clue.1))
        .map(|c| c.points)
        .unwrap_or(0)
}

/// Determine max attempts based on question value
fn calculate_max_attempts(points: u32) -> u32 {
    if points > 500 { 2 } else { 1 }
}

#[derive(Debug, Clone)]
pub enum GameAction {
    AddTeam {
        name: String,
    },
    StartGame,
    SelectClue {
        clue: (usize, usize),
        team_id: u32,
    },
    AnswerCorrect {
        clue: (usize, usize),
        team_id: u32,
    },
    AnswerIncorrect {
        clue: (usize, usize),
        team_id: u32,
    },
    StealAttempt {
        clue: (usize, usize),
        team_id: u32,
        correct: bool,
    },
    CloseClue {
        clue: (usize, usize),
        next_team_id: u32,
    },
    QueueEvent {
        event: GameEvent,
    },
    PlayEventAnimation {
        event: GameEvent,
    },
    TriggerEvent {
        event: GameEvent,
    },
    AcknowledgeEvent,
    ResolveEvent,
    ReturnToConfig,
    ManualPointsAdjustment {
        team_id: u32,
        new_points: i32,
    },
}

#[derive(Debug, Clone)]
pub enum GameActionResult {
    Success {
        new_phase: PlayPhase,
    },
    StateChanged {
        new_phase: PlayPhase,
        effects: Vec<GameEffect>,
    },
}

#[derive(Debug, Clone)]
pub enum GameEffect {
    ScoreChanged {
        team_id: u32,
        delta: i32,
    },
    ClueRevealed {
        clue: (usize, usize),
    },
    ClueSolved {
        clue: (usize, usize),
    },
    FlashEffect {
        effect_type: FlashType,
    },
    EventTriggered {
        event: GameEvent,
    },
    EventQueued {
        event: GameEvent,
    },
    EventAnimation {
        animation_type: EventAnimationType,
    },
    ScoreReset,
    DoublePointsActivated,
    ReverseQuestionActivated,
    ScoreStealApplied {
        context: StealEventContext,
    },
    ManualScoreAdjustment {
        team_id: u32,
        old_score: i32,
        new_score: i32,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum FlashType {
    Correct,
    Incorrect,
}

#[derive(Debug, Clone)]
pub enum GameError {
    InvalidAction { action: String, reason: String },
    EventError(EventError),
}

#[derive(Debug)]
pub struct GameActionHandler {
    rules: GameRules,
    scoring: ScoringEngine,
}

impl GameActionHandler {
    pub fn new() -> Self {
        Self {
            rules: GameRules::new(),
            scoring: ScoringEngine::new(),
        }
    }

    pub fn handle(
        &self,
        state: &mut crate::game::state::GameState,
        action: GameAction,
    ) -> Result<GameActionResult, GameError> {
        match action {
            GameAction::AddTeam { name } => self.handle_add_team(state, name),
            GameAction::StartGame => self.handle_start_game(state),
            GameAction::SelectClue { clue, team_id } => {
                self.handle_select_clue(state, clue, team_id)
            }
            GameAction::AnswerCorrect { clue, team_id } => {
                self.handle_answer_correct(state, clue, team_id)
            }
            GameAction::AnswerIncorrect { clue, team_id } => {
                self.handle_answer_incorrect(state, clue, team_id)
            }
            GameAction::StealAttempt {
                clue,
                team_id,
                correct,
            } => self.handle_steal_attempt(state, clue, team_id, correct),
            GameAction::CloseClue { clue, next_team_id } => {
                self.handle_close_clue(state, clue, next_team_id)
            }
            GameAction::QueueEvent { event } => self.handle_queue_event(state, event),
            GameAction::PlayEventAnimation { event } => {
                self.handle_play_event_animation(state, event)
            }
            GameAction::TriggerEvent { event } => self.handle_trigger_event(state, event),
            GameAction::AcknowledgeEvent => self.handle_acknowledge_event(state),
            GameAction::ResolveEvent => self.handle_resolve_event(state),
            GameAction::ReturnToConfig => self.handle_return_to_config(state),
            GameAction::ManualPointsAdjustment {
                team_id,
                new_points,
            } => self.handle_manual_points_adjustment(state, team_id, new_points),
        }
    }

    fn handle_add_team(
        &self,
        state: &mut crate::game::state::GameState,
        name: String,
    ) -> Result<GameActionResult, GameError> {
        if !self.rules.can_add_team(state) {
            return Err(GameError::InvalidAction {
                action: "AddTeam".to_string(),
                reason: "Can only add teams in lobby phase".to_string(),
            });
        }

        let team_id = self.scoring.add_team(&mut state.teams, name);
        if matches!(state.phase, PlayPhase::Lobby) && state.active_team == 0 {
            state.active_team = team_id;
        }
        Ok(GameActionResult::Success {
            new_phase: state.phase.clone(),
        })
    }

    fn handle_start_game(
        &self,
        state: &mut crate::game::state::GameState,
    ) -> Result<GameActionResult, GameError> {
        if !self.rules.can_start_game(state) {
            return Err(GameError::InvalidAction {
                action: "StartGame".to_string(),
                reason: "Game can only be started from lobby with at least one team".to_string(),
            });
        }

        let first_team_id = state.teams[0].id;
        state.active_team = first_team_id;
        let new_phase = PlayPhase::Selecting {
            team_id: first_team_id,
        };
        state.phase = new_phase.clone();

        Ok(GameActionResult::Success { new_phase })
    }

    fn handle_select_clue(
        &self,
        state: &mut crate::game::state::GameState,
        clue: (usize, usize),
        team_id: u32,
    ) -> Result<GameActionResult, GameError> {
        let action = GameAction::SelectClue { clue, team_id };
        if !self.rules.validate_team_action(state, team_id, &action) {
            return Err(GameError::InvalidAction {
                action: "SelectClue".to_string(),
                reason: "Invalid clue selection or wrong team".to_string(),
            });
        }

        let mut effects = Vec::new();

        // If Reverse Question event is active, swap question and answer
        if state
            .event_state
            .is_event_active(&GameEvent::ReverseQuestion)
        {
            if let Some(category) = state.board.categories.get_mut(clue.0) {
                if let Some(c) = category.clues.get_mut(clue.1) {
                    use crate::game::events::ReverseQuestionEvent;
                    ReverseQuestionEvent::apply_to_clue(c);
                    effects.push(GameEffect::ReverseQuestionActivated);
                }
            }
        }

        let points = get_question_points(state, clue);
        let max_attempts = calculate_max_attempts(points);

        let new_phase = PlayPhase::Showing {
            clue,
            owner_team_id: team_id,
            attempt_count: 1,
            max_attempts,
        };
        state.phase = new_phase.clone();

        if effects.is_empty() {
            Ok(GameActionResult::Success { new_phase })
        } else {
            Ok(GameActionResult::StateChanged { new_phase, effects })
        }
    }

    fn handle_answer_correct(
        &self,
        state: &mut crate::game::state::GameState,
        clue: (usize, usize),
        team_id: u32,
    ) -> Result<GameActionResult, GameError> {
        let action = GameAction::AnswerCorrect { clue, team_id };
        if !self.rules.validate_team_action(state, team_id, &action) {
            return Err(GameError::InvalidAction {
                action: "AnswerCorrect".to_string(),
                reason: "Can only answer in showing phase with correct team".to_string(),
            });
        }

        let mut effects = Vec::new();

        // Mark clue as revealed and solved
        if let Some(category) = state.board.categories.get_mut(clue.0) {
            if let Some(c) = category.clues.get_mut(clue.1) {
                c.revealed = true;
                c.solved = true;
                effects.push(GameEffect::ClueRevealed { clue });
                effects.push(GameEffect::ClueSolved { clue });

                // Calculate points (double if Double Points event is active)
                let points = if state.event_state.is_event_active(&GameEvent::DoublePoints) {
                    use crate::game::events::DoublePointsEvent;
                    DoublePointsEvent::calculate_points(c.points) as i32
                } else {
                    c.points as i32
                };

                // Award points to team
                if self.scoring.award_points(&mut state.teams, team_id, points) {
                    effects.push(GameEffect::ScoreChanged {
                        team_id,
                        delta: points,
                    });
                }

                // If this was a double points question, resolve the event
                if state.event_state.is_event_active(&GameEvent::DoublePoints) {
                    state.event_state.deactivate_event();
                }

                // If this was a reverse question, restore the clue and resolve the event
                if state
                    .event_state
                    .is_event_active(&GameEvent::ReverseQuestion)
                {
                    use crate::game::events::ReverseQuestionEvent;
                    ReverseQuestionEvent::restore_clue(c);
                    state.event_state.deactivate_event();
                }
            }
        }

        effects.push(GameEffect::FlashEffect {
            effect_type: FlashType::Correct,
        });

        // Always rotate the selecting team after a question resolves
        let next_team_id = self
            .scoring
            .rotate_active_team(&state.teams, state.active_team);
        state.active_team = next_team_id;

        let new_phase = PlayPhase::Resolved { clue, next_team_id };
        state.phase = new_phase.clone();

        Ok(GameActionResult::StateChanged { new_phase, effects })
    }

    fn handle_answer_incorrect(
        &self,
        state: &mut crate::game::state::GameState,
        clue: (usize, usize),
        team_id: u32,
    ) -> Result<GameActionResult, GameError> {
        let action = GameAction::AnswerIncorrect { clue, team_id };
        if !self.rules.validate_team_action(state, team_id, &action) {
            return Err(GameError::InvalidAction {
                action: "AnswerIncorrect".to_string(),
                reason: "Can only answer in showing phase with correct team".to_string(),
            });
        }

        // Get current attempt info from showing phase
        if let PlayPhase::Showing {
            attempt_count,
            max_attempts,
            ..
        } = &state.phase
        {
            let mut effects = Vec::new();

            // Always play the incorrect animation
            effects.push(GameEffect::FlashEffect {
                effect_type: FlashType::Incorrect,
            });

            // Check if this is the final attempt
            if *attempt_count < *max_attempts {
                // First attempt on high-value question - no point deduction, stay in showing
                let new_phase = PlayPhase::Showing {
                    clue,
                    owner_team_id: team_id,
                    attempt_count: attempt_count + 1,
                    max_attempts: *max_attempts,
                };
                state.phase = new_phase.clone();

                return Ok(GameActionResult::StateChanged { new_phase, effects });
            } else {
                // Final attempt failed - proceed with existing logic (deduct points, go to stealing)
                return self.handle_final_attempt_incorrect(state, clue, team_id, effects);
            }
        } else {
            return Err(GameError::InvalidAction {
                action: "AnswerIncorrect".to_string(),
                reason: "Can only answer incorrect in showing phase".to_string(),
            });
        }
    }

    fn handle_final_attempt_incorrect(
        &self,
        state: &mut crate::game::state::GameState,
        clue: (usize, usize),
        team_id: u32,
        mut effects: Vec<GameEffect>,
    ) -> Result<GameActionResult, GameError> {
        // Deduct points from team (double penalty if Double Points event is active)
        if let Some(category) = state.board.categories.get(clue.0) {
            if let Some(c) = category.clues.get(clue.1) {
                let penalty = if state.event_state.is_event_active(&GameEvent::DoublePoints) {
                    use crate::game::events::DoublePointsEvent;
                    DoublePointsEvent::calculate_penalty(c.points)
                } else {
                    c.points as i32
                };

                if self
                    .scoring
                    .deduct_points(&mut state.teams, team_id, penalty)
                {
                    effects.push(GameEffect::ScoreChanged {
                        team_id,
                        delta: -penalty,
                    });
                }
            }
        }

        // Create steal queue using rules
        let mut queue = self.rules.get_steal_queue(state, team_id);
        let current = queue.pop_front().unwrap_or(team_id);

        let new_phase = PlayPhase::Steal {
            clue,
            queue,
            current,
            owner_team_id: team_id,
        };
        state.phase = new_phase.clone();

        Ok(GameActionResult::StateChanged { new_phase, effects })
    }

    fn handle_steal_attempt(
        &self,
        state: &mut crate::game::state::GameState,
        clue: (usize, usize),
        team_id: u32,
        correct: bool,
    ) -> Result<GameActionResult, GameError> {
        let action = GameAction::StealAttempt {
            clue,
            team_id,
            correct,
        };
        if !self.rules.validate_team_action(state, team_id, &action) {
            return Err(GameError::InvalidAction {
                action: "StealAttempt".to_string(),
                reason: "Invalid steal attempt or wrong team".to_string(),
            });
        }

        if let PlayPhase::Steal {
            queue,
            current,
            owner_team_id: _,
            ..
        } = &mut state.phase
        {
            let mut effects = Vec::new();

            if correct {
                // Mark clue as revealed and solved
                if let Some(category) = state.board.categories.get_mut(clue.0) {
                    if let Some(c) = category.clues.get_mut(clue.1) {
                        c.revealed = true;
                        c.solved = true;
                        effects.push(GameEffect::ClueRevealed { clue });
                        effects.push(GameEffect::ClueSolved { clue });

                        // Calculate points (double if Double Points event is active)
                        let points = if state.event_state.is_event_active(&GameEvent::DoublePoints)
                        {
                            use crate::game::events::DoublePointsEvent;
                            DoublePointsEvent::calculate_points(c.points) as i32
                        } else {
                            c.points as i32
                        };

                        // Award points to stealing team
                        if self.scoring.award_points(&mut state.teams, team_id, points) {
                            effects.push(GameEffect::ScoreChanged {
                                team_id,
                                delta: points,
                            });
                        }

                        // If this was a double points question, resolve the event
                        if state.event_state.is_event_active(&GameEvent::DoublePoints) {
                            state.event_state.deactivate_event();
                        }

                        // If this was a reverse question, restore the clue and resolve the event
                        if state
                            .event_state
                            .is_event_active(&GameEvent::ReverseQuestion)
                        {
                            use crate::game::events::ReverseQuestionEvent;
                            ReverseQuestionEvent::restore_clue(c);
                            state.event_state.deactivate_event();
                        }
                    }
                }

                effects.push(GameEffect::FlashEffect {
                    effect_type: FlashType::Correct,
                });

                // Always rotate the selecting team after a question resolves
                let next_team_id = self
                    .scoring
                    .rotate_active_team(&state.teams, state.active_team);
                state.active_team = next_team_id;

                let new_phase = PlayPhase::Resolved { clue, next_team_id };
                state.phase = new_phase.clone();

                Ok(GameActionResult::StateChanged { new_phase, effects })
            } else {
                effects.push(GameEffect::FlashEffect {
                    effect_type: FlashType::Incorrect,
                });

                if let Some(next_team) = queue.pop_front() {
                    *current = next_team;
                    Ok(GameActionResult::StateChanged {
                        new_phase: state.phase.clone(),
                        effects,
                    })
                } else {
                    // No more teams, mark clue as solved without points
                    if let Some(category) = state.board.categories.get_mut(clue.0) {
                        if let Some(c) = category.clues.get_mut(clue.1) {
                            // If this was a reverse question, restore the clue before marking as solved
                            if state
                                .event_state
                                .is_event_active(&GameEvent::ReverseQuestion)
                            {
                                use crate::game::events::ReverseQuestionEvent;
                                ReverseQuestionEvent::restore_clue(c);
                                state.event_state.deactivate_event();
                            }

                            c.solved = true;
                            effects.push(GameEffect::ClueSolved { clue });
                        }
                    }

                    // No successful stealers; still rotate the selecting team
                    let next_team_id = self
                        .scoring
                        .rotate_active_team(&state.teams, state.active_team);
                    state.active_team = next_team_id;

                    let new_phase = PlayPhase::Resolved { clue, next_team_id };
                    state.phase = new_phase.clone();

                    Ok(GameActionResult::StateChanged { new_phase, effects })
                }
            }
        } else {
            Err(GameError::InvalidAction {
                action: "StealAttempt".to_string(),
                reason: "Can only steal in steal phase".to_string(),
            })
        }
    }

    fn handle_close_clue(
        &self,
        state: &mut crate::game::state::GameState,
        clue: (usize, usize),
        next_team_id: u32,
    ) -> Result<GameActionResult, GameError> {
        let action = GameAction::CloseClue { clue, next_team_id };
        if !self.rules.is_action_valid(state, &action) {
            return Err(GameError::InvalidAction {
                action: "CloseClue".to_string(),
                reason: "Can only close clue in resolved phase".to_string(),
            });
        }

        // Increment question count for event system
        state.event_state.increment_question_count();

        let mut effects = Vec::new();

        // Check if an event should be triggered
        if state.event_state.should_trigger_event() {
            // Select a random event
            use crate::game::events::EventConfig;
            let config = EventConfig::default();

            if let Some(event) = config.get_random_event() {
                // Queue the event for animation during transition
                state.event_state.queue_event(event.clone());

                // Apply immediate effects for Hard Reset
                if matches!(event, GameEvent::HardReset) {
                    // Reset all team scores immediately
                    for team in &mut state.teams {
                        team.score = 0;
                    }
                    effects.push(GameEffect::ScoreReset);
                } else if matches!(event, GameEvent::ScoreSteal) {
                    // Apply score steal immediately and store context
                    if let Some((thief_idx, victim_idx)) =
                        lowest_and_highest_team_indices(&state.teams)
                    {
                        let (thief, victim) = {
                            let (left, right) = state.teams.split_at_mut(victim_idx.max(thief_idx));
                            if thief_idx < victim_idx {
                                (&mut left[thief_idx], &mut right[0])
                            } else {
                                (&mut right[0], &mut left[victim_idx])
                            }
                        };
                        let amount = ((victim.score as f32) * 0.20).floor() as i32;
                        let amount = amount.max(0);
                        victim.score = victim.score.saturating_sub(amount);
                        thief.score = thief.score.saturating_add(amount);
                        // Save context for UI
                        state.event_state.last_steal = Some(StealEventContext {
                            thief_id: thief.id,
                            thief_name: thief.name.clone(),
                            victim_id: victim.id,
                            victim_name: victim.name.clone(),
                            amount,
                        });
                        effects.push(GameEffect::ScoreChanged {
                            team_id: victim.id,
                            delta: -amount,
                        });
                        effects.push(GameEffect::ScoreChanged {
                            team_id: thief.id,
                            delta: amount,
                        });
                        effects.push(GameEffect::ScoreStealApplied {
                            context: state.event_state.last_steal.clone().unwrap(),
                        });
                    }
                }

                effects.push(GameEffect::EventQueued { event });
            }
        }

        let new_phase = PlayPhase::Selecting {
            team_id: next_team_id,
        };
        state.phase = new_phase.clone();

        if effects.is_empty() {
            Ok(GameActionResult::Success { new_phase })
        } else {
            Ok(GameActionResult::StateChanged { new_phase, effects })
        }
    }

    fn handle_queue_event(
        &self,
        state: &mut crate::game::state::GameState,
        event: GameEvent,
    ) -> Result<GameActionResult, GameError> {
        // Queue the event for animation during transition
        state.event_state.queue_event(event.clone());

        let mut effects = vec![GameEffect::EventQueued {
            event: event.clone(),
        }];

        // Apply immediate effects for Hard Reset
        if matches!(event, GameEvent::HardReset) {
            // Reset all team scores immediately
            for team in &mut state.teams {
                team.score = 0;
            }
            effects.push(GameEffect::ScoreReset);
        }

        Ok(GameActionResult::StateChanged {
            new_phase: state.phase.clone(),
            effects,
        })
    }

    fn handle_play_event_animation(
        &self,
        state: &mut crate::game::state::GameState,
        event: GameEvent,
    ) -> Result<GameActionResult, GameError> {
        // Set animation playing state
        state.event_state.set_animation_playing(true);

        // For non-Hard Reset events, activate them now for the next cell
        if !matches!(event, GameEvent::HardReset | GameEvent::ScoreSteal) {
            state.event_state.activate_event(event.clone());
        }

        let effects = vec![GameEffect::EventAnimation {
            animation_type: match event {
                GameEvent::DoublePoints => EventAnimationType::DoublePointsMultiplication,
                GameEvent::HardReset => EventAnimationType::HardResetGlitch,
                GameEvent::ReverseQuestion => EventAnimationType::ReverseQuestionFlip,
                GameEvent::ScoreSteal => EventAnimationType::ScoreStealHeist,
            },
        }];

        Ok(GameActionResult::StateChanged {
            new_phase: state.phase.clone(),
            effects,
        })
    }

    fn handle_trigger_event(
        &self,
        state: &mut crate::game::state::GameState,
        event: GameEvent,
    ) -> Result<GameActionResult, GameError> {
        // Check if an event is already active
        if state.event_state.active_event.is_some() {
            return Err(GameError::EventError(EventError::EventAlreadyActive));
        }

        // Activate the event
        state.event_state.activate_event(event.clone());

        let mut effects = vec![
            GameEffect::EventTriggered {
                event: event.clone(),
            },
            GameEffect::EventAnimation {
                animation_type: match event {
                    GameEvent::DoublePoints => EventAnimationType::DoublePointsMultiplication,
                    GameEvent::HardReset => EventAnimationType::HardResetGlitch,
                    GameEvent::ReverseQuestion => EventAnimationType::ReverseQuestionFlip,
                    GameEvent::ScoreSteal => EventAnimationType::ScoreStealHeist,
                },
            },
        ];

        // Apply immediate event effects
        match event {
            GameEvent::HardReset => {
                // Reset all team scores immediately
                for team in &mut state.teams {
                    team.score = 0;
                }
                effects.push(GameEffect::ScoreReset);
            }
            GameEvent::DoublePoints => {
                effects.push(GameEffect::DoublePointsActivated);
            }
            GameEvent::ReverseQuestion => {
                effects.push(GameEffect::ReverseQuestionActivated);
            }
            GameEvent::ScoreSteal => {
                // Apply immediately when triggered manually too
                if let Some((thief_idx, victim_idx)) = lowest_and_highest_team_indices(&state.teams)
                {
                    let (thief, victim) = {
                        let (left, right) = state.teams.split_at_mut(victim_idx.max(thief_idx));
                        if thief_idx < victim_idx {
                            (&mut left[thief_idx], &mut right[0])
                        } else {
                            (&mut right[0], &mut left[victim_idx])
                        }
                    };
                    let amount = ((victim.score as f32) * 0.20).floor() as i32;
                    let amount = amount.max(0);
                    victim.score = victim.score.saturating_sub(amount);
                    thief.score = thief.score.saturating_add(amount);
                    state.event_state.last_steal = Some(StealEventContext {
                        thief_id: thief.id,
                        thief_name: thief.name.clone(),
                        victim_id: victim.id,
                        victim_name: victim.name.clone(),
                        amount,
                    });
                    effects.push(GameEffect::ScoreChanged {
                        team_id: victim.id,
                        delta: -amount,
                    });
                    effects.push(GameEffect::ScoreChanged {
                        team_id: thief.id,
                        delta: amount,
                    });
                    effects.push(GameEffect::ScoreStealApplied {
                        context: state.event_state.last_steal.clone().unwrap(),
                    });
                }
            }
        }

        Ok(GameActionResult::StateChanged {
            new_phase: state.phase.clone(),
            effects,
        })
    }

    fn handle_acknowledge_event(
        &self,
        state: &mut crate::game::state::GameState,
    ) -> Result<GameActionResult, GameError> {
        // This is called when the user acknowledges the event announcement
        // The event remains active but the animation phase is complete
        Ok(GameActionResult::Success {
            new_phase: state.phase.clone(),
        })
    }

    fn handle_resolve_event(
        &self,
        state: &mut crate::game::state::GameState,
    ) -> Result<GameActionResult, GameError> {
        // This is called when the event effect should be removed
        // (e.g., after the double points question is answered)
        state.event_state.deactivate_event();

        Ok(GameActionResult::Success {
            new_phase: state.phase.clone(),
        })
    }

    fn handle_return_to_config(
        &self,
        _state: &mut crate::game::state::GameState,
    ) -> Result<GameActionResult, GameError> {
        // This will be handled at the app level, not at the game state level
        Ok(GameActionResult::Success {
            new_phase: PlayPhase::Finished,
        })
    }

    fn handle_manual_points_adjustment(
        &self,
        state: &mut crate::game::state::GameState,
        team_id: u32,
        new_points: i32,
    ) -> Result<GameActionResult, GameError> {
        // Find the team and update their score
        if let Some(team) = state.teams.iter_mut().find(|t| t.id == team_id) {
            let old_score = team.score;
            team.score = new_points;

            let effects = vec![GameEffect::ManualScoreAdjustment {
                team_id,
                old_score,
                new_score: new_points,
            }];

            Ok(GameActionResult::StateChanged {
                new_phase: state.phase.clone(),
                effects,
            })
        } else {
            Err(GameError::InvalidAction {
                action: "ManualPointsAdjustment".to_string(),
                reason: format!("Team with ID {} not found", team_id),
            })
        }
    }
}

/// Find indices of the lowest-scoring team (thief) and highest-scoring team (victim).
/// Returns None if fewer than 2 teams or all scores equal.
fn lowest_and_highest_team_indices(teams: &[Team]) -> Option<(usize, usize)> {
    if teams.len() < 2 {
        return None;
    }
    let mut min_i = 0usize;
    let mut max_i = 0usize;
    for (i, t) in teams.iter().enumerate() {
        if t.score < teams[min_i].score {
            min_i = i;
        }
        if t.score > teams[max_i].score {
            max_i = i;
        }
    }
    if min_i == max_i {
        None
    } else {
        Some((min_i, max_i))
    }
}

#[cfg(test)]
mod manual_points_tests {
    use super::*;
    use crate::core::Board;
    use crate::game::GameEngine;

    #[test]
    fn test_manual_points_adjustment_valid_team() {
        let board = Board::default();
        let mut engine = GameEngine::new(board);

        // Add a team
        let _ = engine.handle_action(GameAction::AddTeam {
            name: "Test Team".to_string(),
        });

        let team_id = engine.get_state().teams[0].id;
        let initial_score = engine.get_state().teams[0].score;

        // Manually adjust points
        let result = engine.handle_action(GameAction::ManualPointsAdjustment {
            team_id,
            new_points: 500,
        });

        assert!(result.is_ok());

        // Verify the score was updated
        let updated_score = engine.get_state().teams[0].score;
        assert_eq!(updated_score, 500);
        assert_ne!(updated_score, initial_score);

        // Verify the result contains the correct effect
        if let Ok(GameActionResult::StateChanged { effects, .. }) = result {
            assert_eq!(effects.len(), 1);
            if let GameEffect::ManualScoreAdjustment {
                team_id: effect_team_id,
                old_score,
                new_score,
            } = &effects[0]
            {
                assert_eq!(*effect_team_id, team_id);
                assert_eq!(*old_score, initial_score);
                assert_eq!(*new_score, 500);
            } else {
                panic!("Expected ManualScoreAdjustment effect");
            }
        } else {
            panic!("Expected StateChanged result");
        }
    }

    #[test]
    fn test_manual_points_adjustment_invalid_team() {
        let board = Board::default();
        let mut engine = GameEngine::new(board);

        // Try to adjust points for non-existent team
        let result = engine.handle_action(GameAction::ManualPointsAdjustment {
            team_id: 999,
            new_points: 500,
        });

        assert!(result.is_err());

        if let Err(GameError::InvalidAction { action, reason }) = result {
            assert_eq!(action, "ManualPointsAdjustment");
            assert!(reason.contains("Team with ID 999 not found"));
        } else {
            panic!("Expected InvalidAction error");
        }
    }

    #[test]
    fn test_manual_points_adjustment_negative_points() {
        let board = Board::default();
        let mut engine = GameEngine::new(board);

        // Add a team
        let _ = engine.handle_action(GameAction::AddTeam {
            name: "Test Team".to_string(),
        });

        let team_id = engine.get_state().teams[0].id;

        // Manually adjust to negative points
        let result = engine.handle_action(GameAction::ManualPointsAdjustment {
            team_id,
            new_points: -100,
        });

        assert!(result.is_ok());

        // Verify negative points are allowed
        let updated_score = engine.get_state().teams[0].score;
        assert_eq!(updated_score, -100);
    }
}
#[cfg(test)]
mod two_attempt_tests {
    use super::*;
    use crate::core::{Board, Category, Clue};
    use crate::game::GameEngine;

    fn create_test_board_with_high_value_questions() -> Board {
        let mut board = Board::default();
        board.categories = vec![Category {
            name: "Test Category".to_string(),
            clues: vec![
                Clue {
                    id: 1,
                    question: "Low value question".to_string(),
                    answer: "Low answer".to_string(),
                    points: 200,
                    solved: false,
                    revealed: false,
                },
                Clue {
                    id: 2,
                    question: "High value question".to_string(),
                    answer: "High answer".to_string(),
                    points: 800,
                    solved: false,
                    revealed: false,
                },
            ],
        }];
        board
    }

    #[test]
    fn test_utility_functions() {
        let board = create_test_board_with_high_value_questions();
        let state = crate::game::state::GameState::new(board);

        // Test get_question_points
        assert_eq!(get_question_points(&state, (0, 0)), 200);
        assert_eq!(get_question_points(&state, (0, 1)), 800);
        assert_eq!(get_question_points(&state, (1, 0)), 0); // Invalid clue

        // Test calculate_max_attempts
        assert_eq!(calculate_max_attempts(200), 1);
        assert_eq!(calculate_max_attempts(500), 1);
        assert_eq!(calculate_max_attempts(501), 2);
        assert_eq!(calculate_max_attempts(800), 2);
    }

    #[test]
    fn test_low_value_question_single_attempt() {
        let board = create_test_board_with_high_value_questions();
        let mut engine = GameEngine::new(board);

        // Add a team and start game
        let _ = engine.handle_action(GameAction::AddTeam {
            name: "Test Team".to_string(),
        });
        let _ = engine.handle_action(GameAction::StartGame);

        let team_id = engine.get_state().teams[0].id;

        // Select low-value question (200 points)
        let result = engine.handle_action(GameAction::SelectClue {
            clue: (0, 0),
            team_id,
        });
        assert!(result.is_ok());

        // Verify showing phase with single attempt
        if let PlayPhase::Showing {
            attempt_count,
            max_attempts,
            ..
        } = &engine.get_state().phase
        {
            assert_eq!(*attempt_count, 1);
            assert_eq!(*max_attempts, 1);
        } else {
            panic!("Expected Showing phase");
        }

        // Answer incorrectly - should go directly to stealing
        let result = engine.handle_action(GameAction::AnswerIncorrect {
            clue: (0, 0),
            team_id,
        });
        assert!(result.is_ok());

        // Should be in stealing phase
        assert!(matches!(engine.get_state().phase, PlayPhase::Steal { .. }));

        // Team should have lost points
        assert_eq!(engine.get_state().teams[0].score, -200);
    }

    #[test]
    fn test_high_value_question_first_attempt_incorrect_second_correct() {
        let board = create_test_board_with_high_value_questions();
        let mut engine = GameEngine::new(board);

        // Add a team and start game
        let _ = engine.handle_action(GameAction::AddTeam {
            name: "Test Team".to_string(),
        });
        let _ = engine.handle_action(GameAction::StartGame);

        let team_id = engine.get_state().teams[0].id;

        // Select high-value question (800 points)
        let result = engine.handle_action(GameAction::SelectClue {
            clue: (0, 1),
            team_id,
        });
        assert!(result.is_ok());

        // Verify showing phase with two attempts
        if let PlayPhase::Showing {
            attempt_count,
            max_attempts,
            ..
        } = &engine.get_state().phase
        {
            assert_eq!(*attempt_count, 1);
            assert_eq!(*max_attempts, 2);
        } else {
            panic!("Expected Showing phase");
        }

        // First attempt incorrect - should stay in showing phase, no point deduction
        let initial_score = engine.get_state().teams[0].score;
        let result = engine.handle_action(GameAction::AnswerIncorrect {
            clue: (0, 1),
            team_id,
        });
        assert!(result.is_ok());

        // Should still be in showing phase with incremented attempt count
        if let PlayPhase::Showing {
            attempt_count,
            max_attempts,
            ..
        } = &engine.get_state().phase
        {
            assert_eq!(*attempt_count, 2);
            assert_eq!(*max_attempts, 2);
        } else {
            panic!("Expected Showing phase after first incorrect attempt");
        }

        // No points should be deducted yet
        assert_eq!(engine.get_state().teams[0].score, initial_score);

        // Second attempt correct - should award points and go to resolved
        let result = engine.handle_action(GameAction::AnswerCorrect {
            clue: (0, 1),
            team_id,
        });
        assert!(result.is_ok());

        // Should be in resolved phase
        assert!(matches!(
            engine.get_state().phase,
            PlayPhase::Resolved { .. }
        ));

        // Team should have gained points
        assert_eq!(engine.get_state().teams[0].score, initial_score + 800);

        // Clue should be solved
        assert!(engine.get_state().board.categories[0].clues[1].solved);
    }

    #[test]
    fn test_high_value_question_both_attempts_incorrect() {
        let board = create_test_board_with_high_value_questions();
        let mut engine = GameEngine::new(board);

        // Add two teams and start game
        let _ = engine.handle_action(GameAction::AddTeam {
            name: "Team 1".to_string(),
        });
        let _ = engine.handle_action(GameAction::AddTeam {
            name: "Team 2".to_string(),
        });
        let _ = engine.handle_action(GameAction::StartGame);

        let team_id = engine.get_state().teams[0].id;

        // Select high-value question (800 points)
        let _ = engine.handle_action(GameAction::SelectClue {
            clue: (0, 1),
            team_id,
        });

        // First attempt incorrect
        let initial_score = engine.get_state().teams[0].score;
        let _ = engine.handle_action(GameAction::AnswerIncorrect {
            clue: (0, 1),
            team_id,
        });

        // Should still be in showing phase, no point deduction
        assert!(matches!(
            engine.get_state().phase,
            PlayPhase::Showing { .. }
        ));
        assert_eq!(engine.get_state().teams[0].score, initial_score);

        // Second attempt incorrect - should deduct points and go to stealing
        let result = engine.handle_action(GameAction::AnswerIncorrect {
            clue: (0, 1),
            team_id,
        });
        assert!(result.is_ok());

        // Should be in stealing phase
        assert!(matches!(engine.get_state().phase, PlayPhase::Steal { .. }));

        // Team should have lost points
        assert_eq!(engine.get_state().teams[0].score, initial_score - 800);
    }

    #[test]
    fn test_high_value_question_first_attempt_correct() {
        let board = create_test_board_with_high_value_questions();
        let mut engine = GameEngine::new(board);

        // Add a team and start game
        let _ = engine.handle_action(GameAction::AddTeam {
            name: "Test Team".to_string(),
        });
        let _ = engine.handle_action(GameAction::StartGame);

        let team_id = engine.get_state().teams[0].id;

        // Select high-value question (800 points)
        let _ = engine.handle_action(GameAction::SelectClue {
            clue: (0, 1),
            team_id,
        });

        // First attempt correct - should award points and go to resolved
        let initial_score = engine.get_state().teams[0].score;
        let result = engine.handle_action(GameAction::AnswerCorrect {
            clue: (0, 1),
            team_id,
        });
        assert!(result.is_ok());

        // Should be in resolved phase
        assert!(matches!(
            engine.get_state().phase,
            PlayPhase::Resolved { .. }
        ));

        // Team should have gained points
        assert_eq!(engine.get_state().teams[0].score, initial_score + 800);

        // Clue should be solved
        assert!(engine.get_state().board.categories[0].clues[1].solved);
    }

    #[test]
    fn test_boundary_condition_500_points() {
        let mut board = Board::default();
        board.categories = vec![Category {
            name: "Test Category".to_string(),
            clues: vec![Clue {
                id: 1,
                question: "Exactly 500 points".to_string(),
                answer: "Answer".to_string(),
                points: 500,
                solved: false,
                revealed: false,
            }],
        }];

        let mut engine = GameEngine::new(board);

        // Add a team and start game
        let _ = engine.handle_action(GameAction::AddTeam {
            name: "Test Team".to_string(),
        });
        let _ = engine.handle_action(GameAction::StartGame);

        let team_id = engine.get_state().teams[0].id;

        // Select 500-point question
        let result = engine.handle_action(GameAction::SelectClue {
            clue: (0, 0),
            team_id,
        });
        assert!(result.is_ok());

        // Should have single attempt (500 points is not > 500)
        if let PlayPhase::Showing {
            attempt_count,
            max_attempts,
            ..
        } = &engine.get_state().phase
        {
            assert_eq!(*attempt_count, 1);
            assert_eq!(*max_attempts, 1);
        } else {
            panic!("Expected Showing phase");
        }
    }
}
#[cfg(test)]
mod edge_case_tests {
    use super::*;
    use crate::core::{Board, Category, Clue};
    use crate::game::GameEngine;

    #[test]
    fn test_invalid_clue_coordinates() {
        let board = Board::default();
        let state = crate::game::state::GameState::new(board);

        // Test invalid category index
        assert_eq!(get_question_points(&state, (999, 0)), 0);

        // Test invalid clue index
        assert_eq!(get_question_points(&state, (0, 999)), 0);

        // Both invalid
        assert_eq!(get_question_points(&state, (999, 999)), 0);
    }

    #[test]
    fn test_zero_point_questions() {
        let mut board = Board::default();
        board.categories = vec![Category {
            name: "Test Category".to_string(),
            clues: vec![Clue {
                id: 1,
                question: "Zero points".to_string(),
                answer: "Answer".to_string(),
                points: 0,
                solved: false,
                revealed: false,
            }],
        }];

        let mut engine = GameEngine::new(board);

        // Add a team and start game
        let _ = engine.handle_action(GameAction::AddTeam {
            name: "Test Team".to_string(),
        });
        let _ = engine.handle_action(GameAction::StartGame);

        let team_id = engine.get_state().teams[0].id;

        // Select zero-point question
        let result = engine.handle_action(GameAction::SelectClue {
            clue: (0, 0),
            team_id,
        });
        assert!(result.is_ok());

        // Should have single attempt (0 points defaults to single attempt)
        if let PlayPhase::Showing {
            attempt_count,
            max_attempts,
            ..
        } = &engine.get_state().phase
        {
            assert_eq!(*attempt_count, 1);
            assert_eq!(*max_attempts, 1);
        } else {
            panic!("Expected Showing phase");
        }
    }

    #[test]
    fn test_attempt_count_validation() {
        // This test verifies that attempt_count cannot exceed max_attempts
        // The logic in handle_answer_incorrect should prevent this
        let mut board = Board::default();
        board.categories = vec![Category {
            name: "Test Category".to_string(),
            clues: vec![Clue {
                id: 1,
                question: "High value question".to_string(),
                answer: "Answer".to_string(),
                points: 800,
                solved: false,
                revealed: false,
            }],
        }];

        let mut engine = GameEngine::new(board);

        // Add a team and start game
        let _ = engine.handle_action(GameAction::AddTeam {
            name: "Test Team".to_string(),
        });
        let _ = engine.handle_action(GameAction::StartGame);

        let team_id = engine.get_state().teams[0].id;

        // Select high-value question
        let _ = engine.handle_action(GameAction::SelectClue {
            clue: (0, 0),
            team_id,
        });

        // First incorrect attempt
        let _ = engine.handle_action(GameAction::AnswerIncorrect {
            clue: (0, 0),
            team_id,
        });

        // Verify we're still in showing phase with attempt_count = 2
        if let PlayPhase::Showing {
            attempt_count,
            max_attempts,
            ..
        } = &engine.get_state().phase
        {
            assert_eq!(*attempt_count, 2);
            assert_eq!(*max_attempts, 2);
        } else {
            panic!("Expected Showing phase after first incorrect attempt");
        }

        // Second incorrect attempt should go to stealing phase
        let result = engine.handle_action(GameAction::AnswerIncorrect {
            clue: (0, 0),
            team_id,
        });
        assert!(result.is_ok());

        // Should be in stealing phase now
        assert!(matches!(engine.get_state().phase, PlayPhase::Steal { .. }));
    }

    #[test]
    fn test_answer_incorrect_in_wrong_phase() {
        let board = Board::default();
        let mut engine = GameEngine::new(board);

        // Add a team but don't start game (stay in lobby)
        let _ = engine.handle_action(GameAction::AddTeam {
            name: "Test Team".to_string(),
        });

        let team_id = engine.get_state().teams[0].id;

        // Try to answer incorrect while in lobby phase
        let result = engine.handle_action(GameAction::AnswerIncorrect {
            clue: (0, 0),
            team_id,
        });

        // Should return an error
        assert!(result.is_err());
        if let Err(GameError::InvalidAction { action, reason }) = result {
            assert_eq!(action, "AnswerIncorrect");
            assert!(reason.contains("Can only answer in showing phase"));
        } else {
            panic!("Expected InvalidAction error");
        }
    }
}
