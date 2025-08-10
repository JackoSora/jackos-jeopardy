pub mod state;
pub mod actions;
pub mod rules;
pub mod scoring;
pub mod engine;

// Re-export the main types for backward compatibility
pub use state::{GameState, PlayPhase};
pub use actions::{GameAction, GameActionResult, GameActionHandler, GameEffect, FlashType, GameError};
pub use rules::GameRules;
pub use scoring::{ScoringEngine, TeamStats};
pub use engine::GameEngine;