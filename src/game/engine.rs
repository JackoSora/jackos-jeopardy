use crate::core::Board;
use crate::game::actions::{GameAction, GameActionHandler, GameActionResult, GameError};
use crate::game::state::{GameState, PlayPhase};

/// The main game engine that coordinates all game subsystems
#[derive(Debug)]
pub struct GameEngine {
    pub state: GameState,
    action_handler: GameActionHandler,
}

impl GameEngine {
    /// Create a new game engine with the given board
    pub fn new(board: Board) -> Self {
        Self {
            state: GameState::new(board),
            action_handler: GameActionHandler::new(),
        }
    }

    /// Handle a game action and return the result
    pub fn handle_action(&mut self, action: GameAction) -> Result<GameActionResult, GameError> {
        self.action_handler.handle(&mut self.state, action)
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


    /// Get the number of teams
    pub fn team_count(&self) -> usize {
        self.state.teams.len()
    }

    // API methods for tests and future use
    pub fn get_team_score(&self, team_id: u32) -> Option<i32> {
        self.state.teams.iter().find(|t| t.id == team_id).map(|t| t.score)
    }

    pub fn get_active_team(&self) -> Option<&crate::core::Team> {
        self.state.teams.iter().find(|t| t.id == self.state.active_team)
    }

    pub fn is_clue_available(&self, clue: (usize, usize)) -> bool {
        self.state.is_clue_available(clue)
    }

    pub fn get_available_clues(&self) -> Vec<(usize, usize)> {
        let mut available = Vec::new();
        for (cat_idx, category) in self.state.board.categories.iter().enumerate() {
            for (clue_idx, clue) in category.clues.iter().enumerate() {
                if !clue.solved {
                    available.push((cat_idx, clue_idx));
                }
            }
        }
        available
    }

    pub fn get_clue(&self, clue: (usize, usize)) -> Option<&crate::core::Clue> {
        self.state.get_clue(clue)
    }

}
