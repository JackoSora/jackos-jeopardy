// UI module for game-specific components
pub mod board;
pub mod modals;
pub mod indicators;

// Re-export commonly used items
pub use board::{paint_enhanced_clue_cell, paint_enhanced_category_header};
pub use modals::{paint_enhanced_modal_background};
pub use indicators::{paint_active_team_indicator, paint_game_phase_indicator};