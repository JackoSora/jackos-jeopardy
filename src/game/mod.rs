pub mod actions;
pub mod engine;
pub mod rules;
pub mod scoring;
pub mod state;

#[cfg(test)]
mod tests;

// Re-export the main types for backward compatibility
pub use actions::{FlashType, GameAction, GameActionResult, GameEffect};
pub use state::{GameState, PlayPhase};
// Internal modules - not re-exported as they're used through GameEngine
pub use engine::GameEngine;
