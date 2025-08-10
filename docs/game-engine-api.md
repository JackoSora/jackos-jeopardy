# GameEngine API Documentation

The `GameEngine` is the main interface for all game operations in Party Jeopardy!. It coordinates between game state, actions, rules, and scoring systems to provide a unified API for the UI layer.

## Overview

The GameEngine follows an action-based architecture where all game operations are expressed as `GameAction` enums that are processed through the engine. This ensures consistent validation, state management, and effect generation.

## Core Concepts

### GameAction
All game operations are represented as actions:
```rust
pub enum GameAction {
    AddTeam { name: String },
    StartGame,
    SelectClue { clue: (usize, usize), team_id: u32 },
    AnswerCorrect { clue: (usize, usize), team_id: u32 },
    AnswerIncorrect { clue: (usize, usize), team_id: u32 },
    StealAttempt { clue: (usize, usize), team_id: u32, correct: bool },
    CloseClue { clue: (usize, usize), next_team_id: u32 },
    ReturnToConfig,
}
```

### GameActionResult
Actions return structured results:
```rust
pub enum GameActionResult {
    Success { new_phase: PlayPhase },
    Error { message: String },
    StateChanged { new_phase: PlayPhase, effects: Vec<GameEffect> },
}
```

### GameEffect
Visual effects for UI feedback:
```rust
pub enum GameEffect {
    ScoreChanged { team_id: u32, delta: i32 },
    ClueRevealed { clue: (usize, usize) },
    ClueSolved { clue: (usize, usize) },
    TeamRotated { new_active_team: u32 },
    FlashEffect { effect_type: FlashType },
}
```

## API Reference

### Construction
```rust
impl GameEngine {
    /// Create a new game engine with the given board
    pub fn new(board: Board) -> Self
}
```

### Action Processing
```rust
/// Handle a game action and return the result
pub fn handle_action(&mut self, action: GameAction) -> Result<GameActionResult, GameError>

/// Check if a specific action can be performed in the current state
pub fn can_perform_action(&self, action: &GameAction) -> bool

/// Get all actions that are currently available
pub fn get_available_actions(&self) -> Vec<GameAction>
```

### State Queries
```rust
/// Get a reference to the game state (read-only access)
pub fn get_state(&self) -> &GameState

/// Get the current game phase
pub fn get_phase(&self) -> &PlayPhase

/// Check if the game is finished (all clues solved)
pub fn is_game_finished(&self) -> bool
```

### Team Operations
```rust
/// Validate that a team can perform a specific action
pub fn validate_team_action(&self, team_id: u32, action: &GameAction) -> bool

/// Get team score using the scoring engine
pub fn get_team_score(&self, team_id: u32) -> Option<i32>

/// Get leaderboard using the scoring engine
pub fn get_leaderboard(&self) -> Vec<(u32, String, i32)>

/// Check if a team exists
pub fn team_exists(&self, team_id: u32) -> bool

/// Get team statistics
pub fn get_team_stats(&self) -> TeamStats

/// Get the number of teams
pub fn team_count(&self) -> usize

/// Get active team
pub fn get_active_team(&self) -> Option<&Team>

/// Get team by ID
pub fn get_team_by_id(&self, id: u32) -> Option<&Team>
```

### Clue Operations
```rust
/// Check if a clue is available (not solved)
pub fn is_clue_available(&self, clue: (usize, usize)) -> bool

/// Get all available clues
pub fn get_available_clues(&self) -> Vec<(usize, usize)>

/// Get clue by coordinates
pub fn get_clue(&self, clue: (usize, usize)) -> Option<&Clue>
```

### Game Lifecycle
```rust
/// Reset the game to initial state with the same board
pub fn reset(&mut self)

/// Create a new game with a different board
pub fn new_game(&mut self, board: Board)

/// Get a mutable reference to the game state (for serialization/deserialization)
pub fn get_state_mut(&mut self) -> &mut GameState
```

## Usage Patterns

### Basic Game Flow
```rust
// Create engine
let mut engine = GameEngine::new(board);

// Add teams
engine.handle_action(GameAction::AddTeam { name: "Team 1".to_string() })?;
engine.handle_action(GameAction::AddTeam { name: "Team 2".to_string() })?;

// Start game
engine.handle_action(GameAction::StartGame)?;

// Select and answer clues
let team_id = engine.get_active_team().unwrap().id;
engine.handle_action(GameAction::SelectClue { clue: (0, 0), team_id })?;
engine.handle_action(GameAction::AnswerCorrect { clue: (0, 0), team_id })?;
```

### Action Validation
```rust
let action = GameAction::SelectClue { clue: (0, 0), team_id: 1 };

// Check if action is valid before processing
if engine.can_perform_action(&action) {
    match engine.handle_action(action) {
        Ok(GameActionResult::StateChanged { effects, .. }) => {
            // Handle visual effects
            for effect in effects {
                match effect {
                    GameEffect::ScoreChanged { team_id, delta } => {
                        // Update score display
                    }
                    GameEffect::FlashEffect { effect_type } => {
                        // Trigger visual flash
                    }
                    // ... handle other effects
                }
            }
        }
        Ok(_) => { /* Handle other result types */ }
        Err(error) => { /* Handle error */ }
    }
}
```

### State Queries
```rust
// Check game status
if engine.is_game_finished() {
    let leaderboard = engine.get_leaderboard();
    // Display final results
}

// Get available actions for UI
let available_actions = engine.get_available_actions();
for action in available_actions {
    // Enable corresponding UI buttons
}

// Query team information
if let Some(team) = engine.get_active_team() {
    println!("Active team: {} (Score: {})", team.name, team.score);
}
```

## Error Handling

The GameEngine uses typed errors for comprehensive error handling:

```rust
pub enum GameError {
    InvalidAction { action: String, reason: String },
    InvalidTeam { team_id: u32 },
    InvalidClue { clue: (usize, usize) },
    GameNotStarted,
    GameFinished,
    InsufficientTeams,
}
```

Always handle errors appropriately:
```rust
match engine.handle_action(action) {
    Ok(result) => { /* Process successful result */ }
    Err(GameError::InvalidAction { reason, .. }) => {
        // Show user-friendly error message
        eprintln!("Cannot perform action: {}", reason);
    }
    Err(GameError::InvalidTeam { team_id }) => {
        eprintln!("Team {} does not exist", team_id);
    }
    // ... handle other error types
}
```

## Integration with UI

The GameEngine is designed to integrate cleanly with the egui-based UI:

1. **Event Handling**: Convert UI events to GameActions
2. **Validation**: Use `can_perform_action()` to enable/disable UI elements
3. **State Display**: Use query methods to update UI state
4. **Effects**: Process GameEffects for visual feedback
5. **Error Display**: Show user-friendly error messages from GameError

This architecture ensures that all game logic is centralized and testable while providing a clean interface for the UI layer.