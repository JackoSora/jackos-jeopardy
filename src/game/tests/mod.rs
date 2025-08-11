//! Test utilities and test modules for the game logic

use crate::core::Board;
use crate::game::{GameEngine, PlayPhase};

/// Create a simple test board with 2 categories and 2 clues each
pub fn create_test_board() -> Board {
    Board::default_with_dimensions(2, 2)
}

/// Create a test game engine with a simple board
pub fn create_test_game_engine() -> GameEngine {
    GameEngine::new(create_test_board())
}

/// Create a test game state with some teams added
pub fn create_test_game_with_teams() -> GameEngine {
    let mut engine = create_test_game_engine();
    let _ = engine.handle_action(crate::game::GameAction::AddTeam {
        name: "Team 1".to_string(),
    });
    let _ = engine.handle_action(crate::game::GameAction::AddTeam {
        name: "Team 2".to_string(),
    });
    engine
}

/// Create a game in the selecting phase
pub fn create_game_in_selecting_phase() -> GameEngine {
    let mut engine = create_test_game_with_teams();
    let _ = engine.handle_action(crate::game::GameAction::StartGame);
    engine
}

#[cfg(test)]
mod engine_tests;

#[cfg(test)]
mod actions_tests;

#[cfg(test)]
mod rules_tests;

#[cfg(test)]
mod scoring_tests;
