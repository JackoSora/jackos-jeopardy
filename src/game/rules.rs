use crate::game::actions::GameAction;
use crate::game::state::{GameState, PlayPhase};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct GameRules;

impl GameRules {
    pub fn new() -> Self {
        Self
    }

    /// Check if a clue can be selected in the current game state
    pub fn can_select_clue(&self, state: &GameState, clue: (usize, usize)) -> bool {
        // Can only select clues in the selecting phase
        if !matches!(state.phase, PlayPhase::Selecting { .. }) {
            return false;
        }

        // Clue must exist and be available (not solved)
        state.is_clue_available(clue)
    }

    /// Check if the game can be started
    pub fn can_start_game(&self, state: &GameState) -> bool {
        // Must be in lobby phase
        if !matches!(state.phase, PlayPhase::Lobby) {
            return false;
        }

        // Must have at least one team
        !state.teams.is_empty()
    }

    /// Check if a team can be added
    pub fn can_add_team(&self, state: &GameState) -> bool {
        // Can only add teams in lobby phase
        matches!(state.phase, PlayPhase::Lobby)
    }

    /// Generate the steal queue for a given clue, excluding the owner team
    pub fn get_steal_queue(&self, state: &GameState, excluding_team: u32) -> VecDeque<u32> {
        let mut others: Vec<u32> = state
            .teams
            .iter()
            .filter(|t| t.id != excluding_team)
            .map(|t| t.id)
            .collect();

        // Shuffle the order for fairness
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        others.as_mut_slice().shuffle(&mut rng);

        VecDeque::from(others)
    }

    // API methods for tests  
    pub fn is_game_finished(&self, state: &GameState) -> bool {
        for category in &state.board.categories {
            for clue in &category.clues {
                if !clue.solved {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_available_actions(&self, state: &GameState) -> Vec<GameAction> {
        let mut actions = Vec::new();
        match &state.phase {
            PlayPhase::Lobby => {
                actions.push(GameAction::AddTeam {
                    name: "New Team".to_string(),
                });
                if self.can_start_game(state) {
                    actions.push(GameAction::StartGame);
                }
            }
            PlayPhase::Selecting { team_id } => {
                for clue in state.get_available_clues() {
                    actions.push(GameAction::SelectClue {
                        clue,
                        team_id: *team_id,
                    });
                }
            }
            _ => {} // Other phases simplified for now
        }
        actions
    }


    /// Validate if a team can perform a specific action
    pub fn validate_team_action(
        &self,
        state: &GameState,
        team_id: u32,
        action: &GameAction,
    ) -> bool {
        // Check if team exists
        if state.teams.iter().find(|t| t.id == team_id).is_none() {
            return false;
        }

        match action {
            GameAction::AddTeam { .. } => {
                // Anyone can add teams in lobby
                self.can_add_team(state)
            }
            GameAction::StartGame => {
                // Anyone can start the game if conditions are met
                self.can_start_game(state)
            }
            GameAction::SelectClue {
                clue,
                team_id: action_team_id,
            } => {
                // Only the active team can select clues
                if let PlayPhase::Selecting {
                    team_id: active_team,
                } = state.phase
                {
                    *action_team_id == active_team && self.can_select_clue(state, *clue)
                } else {
                    false
                }
            }
            GameAction::AnswerCorrect {
                team_id: action_team_id,
                ..
            }
            | GameAction::AnswerIncorrect {
                team_id: action_team_id,
                ..
            } => {
                // Only the owner team can answer
                if let PlayPhase::Showing { owner_team_id, .. } = state.phase {
                    *action_team_id == owner_team_id
                } else {
                    false
                }
            }
            GameAction::StealAttempt {
                team_id: action_team_id,
                ..
            } => {
                // Only the current stealing team can attempt
                if let PlayPhase::Steal { current, .. } = state.phase {
                    *action_team_id == current
                } else {
                    false
                }
            }
            GameAction::CloseClue { .. } => {
                // Anyone can close a clue in resolved phase
                matches!(state.phase, PlayPhase::Resolved { .. })
            }
            GameAction::ReturnToConfig => {
                // Anyone can return to config
                true
            }
        }
    }


    /// Check if a specific action is valid in the current state
    pub fn is_action_valid(&self, state: &GameState, action: &GameAction) -> bool {
        match action {
            GameAction::AddTeam { .. } => self.can_add_team(state),
            GameAction::StartGame => self.can_start_game(state),
            GameAction::SelectClue { clue, team_id } => {
                if let PlayPhase::Selecting {
                    team_id: active_team,
                } = state.phase
                {
                    *team_id == active_team && self.can_select_clue(state, *clue)
                } else {
                    false
                }
            }
            GameAction::AnswerCorrect { team_id, .. }
            | GameAction::AnswerIncorrect { team_id, .. } => {
                if let PlayPhase::Showing { owner_team_id, .. } = state.phase {
                    *team_id == owner_team_id
                } else {
                    false
                }
            }
            GameAction::StealAttempt { team_id, .. } => {
                if let PlayPhase::Steal { current, .. } = state.phase {
                    *team_id == current
                } else {
                    false
                }
            }
            GameAction::CloseClue { .. } => {
                matches!(state.phase, PlayPhase::Resolved { .. })
            }
            GameAction::ReturnToConfig => true,
        }
    }
}
