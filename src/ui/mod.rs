// UI module for game-specific components
pub mod board;
pub mod indicators;
pub mod modals;

// Re-export commonly used items
pub use board::{paint_enhanced_category_header, paint_enhanced_clue_cell};
pub use modals::paint_subtle_modal_background;
