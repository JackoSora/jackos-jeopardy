use crate::game::events::{EventAnimationType, EventError, GameEvent};
use crate::game::rules::GameRules;
use crate::game::scoring::ScoringEngine;
use crate::game::state::PlayPhase;

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
    ScoreChanged { team_id: u32, delta: i32 },
    ClueRevealed { clue: (usize, usize) },
    ClueSolved { clue: (usize, usize) },
    FlashEffect { effect_type: FlashType },
    EventTriggered { event: GameEvent },
    EventQueued { event: GameEvent },
    EventAnimation { animation_type: EventAnimationType },
    ScoreReset,
    DoublePointsActivated,
    ReverseQuestionActivated,
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

        let new_phase = PlayPhase::Showing {
            clue,
            owner_team_id: team_id,
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

        let mut effects = Vec::new();

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

        effects.push(GameEffect::FlashEffect {
            effect_type: FlashType::Incorrect,
        });

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
        if !matches!(event, GameEvent::HardReset) {
            state.event_state.activate_event(event.clone());
        }

        let effects = vec![GameEffect::EventAnimation {
            animation_type: match event {
                GameEvent::DoublePoints => EventAnimationType::DoublePointsMultiplication,
                GameEvent::HardReset => EventAnimationType::HardResetGlitch,
                GameEvent::ReverseQuestion => EventAnimationType::ReverseQuestionFlip,
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
}
