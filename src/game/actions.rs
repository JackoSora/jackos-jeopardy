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
    ReturnToConfig,
}

#[derive(Debug, Clone)]
pub enum GameActionResult {
    Success {
        new_phase: PlayPhase,
    },
    Error {
        message: String,
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
    TeamRotated { new_active_team: u32 },
    FlashEffect { effect_type: FlashType },
}

#[derive(Debug, Clone, Copy)]
pub enum FlashType {
    Correct,
    Incorrect,
}

#[derive(Debug, Clone)]
pub enum GameError {
    InvalidAction { action: String, reason: String },
    InvalidTeam { team_id: u32 },
    InvalidClue { clue: (usize, usize) },
    GameNotStarted,
    GameFinished,
    InsufficientTeams,
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

        let new_phase = PlayPhase::Showing {
            clue,
            owner_team_id: team_id,
        };
        state.phase = new_phase.clone();

        Ok(GameActionResult::Success { new_phase })
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

                // Award points to team
                if self
                    .scoring
                    .award_points(&mut state.teams, team_id, c.points as i32)
                {
                    effects.push(GameEffect::ScoreChanged {
                        team_id,
                        delta: c.points as i32,
                    });
                }
            }
        }

        effects.push(GameEffect::FlashEffect {
            effect_type: FlashType::Correct,
        });

        let new_phase = PlayPhase::Resolved {
            clue,
            next_team_id: team_id,
        };
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

        // Deduct points from team
        if let Some(category) = state.board.categories.get(clue.0) {
            if let Some(c) = category.clues.get(clue.1) {
                if self
                    .scoring
                    .deduct_points(&mut state.teams, team_id, c.points as i32)
                {
                    effects.push(GameEffect::ScoreChanged {
                        team_id,
                        delta: -(c.points as i32),
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
            owner_team_id,
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

                        // Award points to stealing team
                        if self
                            .scoring
                            .award_points(&mut state.teams, team_id, c.points as i32)
                        {
                            effects.push(GameEffect::ScoreChanged {
                                team_id,
                                delta: c.points as i32,
                            });
                        }
                    }
                }

                effects.push(GameEffect::FlashEffect {
                    effect_type: FlashType::Correct,
                });

                let new_phase = PlayPhase::Resolved {
                    clue,
                    next_team_id: team_id,
                };
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
                            c.solved = true;
                            effects.push(GameEffect::ClueSolved { clue });
                        }
                    }

                    let new_phase = PlayPhase::Resolved {
                        clue,
                        next_team_id: *owner_team_id,
                    };
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

        let new_phase = PlayPhase::Selecting {
            team_id: next_team_id,
        };
        state.phase = new_phase.clone();

        Ok(GameActionResult::Success { new_phase })
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
