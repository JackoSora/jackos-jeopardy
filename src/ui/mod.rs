// UI module for game-specific components
pub mod board;
pub mod indicators;
pub mod modals;

// Re-export commonly used items
pub use board::{paint_enhanced_category_header, paint_enhanced_clue_cell};
pub use indicators::{paint_active_team_indicator, paint_game_phase_indicator};
pub use modals::paint_enhanced_modal_background;
