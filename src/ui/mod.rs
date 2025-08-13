// UI module for game-specific components
pub mod board;
pub mod indicators;
pub mod modals;

// Enhanced UI components
pub mod cell_manager;
pub mod header_animations;
pub mod layout_transitions;

// Re-export commonly used items
pub use board::{paint_enhanced_category_header, paint_enhanced_clue_cell};
pub use modals::paint_subtle_modal_background;

// Re-export enhanced UI components
pub use cell_manager::{CellId, CellManager, CellResponse, CellState, EditField, EnhancedCell};
pub use header_animations::{HeaderAnimationManager, HeaderState};
pub use layout_transitions::{BoardEditorTransitionSystem, ConfigLayoutState};
