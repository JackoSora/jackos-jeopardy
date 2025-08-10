use crate::domain::Board;
use crate::game::actions::{GameAction, GameActionHandler, GameActionResult, GameError};
use crate::game::rules::GameRules;
use crate::game::scoring::ScoringEngine;
use crate::game::state::{GameState, PlayPhase};

/// The main game engine that coordinates all game subsystems
#[derive(Debug)]
pub struct GameEngine {
    pub state: GameState,
    action_handler: GameActionHandler,
    rules: GameRules,
    scoring: ScoringEngine,
}

impl GameEngine {
    /// Create a new game engine with the given board
    pub fn new(board: Board) -> Self {
        Self {
            state: GameState::new(board),
            action_handler: GameActionHandler::new(),
            rules: GameRules::new(),
            scoring: ScoringEngine::new(),
        }
    }

    /// Handle a game action and return the result
    pub fn handle_action(&mut self, action: GameAction) -> Result<GameActionResult, GameError> {
        self.action_handler.handle(&mut self.state, action)
    }

    /// Check if a specific action can be performed in the current state
    pub fn can_perform_action(&self, action: &GameAction) -> bool {
        self.rules.is_action_valid(&self.state, action)
    }

    /// Get all actions that are currently available
    pub fn get_available_actions(&self) -> Vec<GameAction> {
        self.rules.get_available_actions(&self.state)
    }

    /// Check if the game is finished (all clues solved)
    pub fn is_game_finished(&self) -> bool {
        self.rules.is_game_finished(&self.state)
    }

    /// Get the current game phase
    pub fn get_phase(&self) -> &PlayPhase {
        &self.state.phase
    }

    /// Get a reference to the game state (read-only access)
    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    /// Get a mutable reference to the game state (for serialization/deserialization)
    pub fn get_state_mut(&mut self) -> &mut GameState {
        &mut self.state
    }

    /// Validate that a team can perform a specific action
    pub fn validate_team_action(&self, team_id: u32, action: &GameAction) -> bool {
        self.rules
            .validate_team_action(&self.state, team_id, action)
    }

    /// Get team score using the scoring engine
    pub fn get_team_score(&self, team_id: u32) -> Option<i32> {
        self.scoring.get_team_score(&self.state.teams, team_id)
    }

    /// Get leaderboard using the scoring engine
    pub fn get_leaderboard(&self) -> Vec<(u32, String, i32)> {
        self.scoring.get_leaderboard(&self.state.teams)
    }

    /// Check if a team exists
    pub fn team_exists(&self, team_id: u32) -> bool {
        self.scoring.team_exists(&self.state.teams, team_id)
    }

    /// Get team statistics
    pub fn get_team_stats(&self) -> crate::game::scoring::TeamStats {
        self.scoring.get_team_stats(&self.state.teams)
    }

    /// Reset the game to initial state with the same board
    pub fn reset(&mut self) {
        let board = self.state.board.clone();
        self.state = GameState::new(board);
    }

    /// Create a new game with a different board
    pub fn new_game(&mut self, board: Board) {
        self.state = GameState::new(board);
    }

    /// Get the number of teams
    pub fn team_count(&self) -> usize {
        self.state.teams.len()
    }

    /// Check if a clue is available (not solved)
    pub fn is_clue_available(&self, clue: (usize, usize)) -> bool {
        self.state.is_clue_available(clue)
    }

    /// Get all available clues
    pub fn get_available_clues(&self) -> Vec<(usize, usize)> {
        self.state.get_available_clues()
    }

    /// Get active team
    pub fn get_active_team(&self) -> Option<&crate::domain::Team> {
        self.state.get_active_team()
    }

    /// Get team by ID
    pub fn get_team_by_id(&self, id: u32) -> Option<&crate::domain::Team> {
        self.state.get_team_by_id(id)
    }

    /// Get clue by coordinates
    pub fn get_clue(&self, clue: (usize, usize)) -> Option<&crate::domain::Clue> {
        self.state.get_clue(clue)
    }
}
