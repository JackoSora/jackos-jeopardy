pub mod state;
pub mod actions;
pub mod rules;
pub mod scoring;
pub mod engine;

#[cfg(test)]
mod tests;

// Re-export the main types for backward compatibility
pub use state::{GameState, PlayPhase};
pub use actions::{GameAction, GameActionResult, GameEffect, FlashType};
// Internal modules - not re-exported as they're used through GameEngine
pub use engine::GameEngine;