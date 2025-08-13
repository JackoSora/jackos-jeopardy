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
    TriggerEvent { event: GameEvent },
    AcknowledgeEvent,
    ResolveEvent,
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
    FlashEffect { effect_type: FlashType },
    EventTriggered { event: GameEvent },
    EventAnimation { animation_type: EventAnimationType },
    ScoreReset,
    DoublePointsActivated,
    ReverseQuestionActivated,
}
```

## Event System

The game includes a dynamic event system that triggers special gameplay modifiers every 4 questions. Events add excitement and strategic depth to the game. The event system is fully integrated into the game state with backward compatibility for existing save files and includes sophisticated animation timing control.

### Event Types
```rust
pub enum GameEvent {
    DoublePoints,    // Next correct answer awards double points, incorrect loses double
    HardReset,       // All team scores reset to zero immediately
    ReverseQuestion, // Next clue has question and answer swapped
    ScoreSteal,      // Lowest score team steals 20% from highest score team
}
```

**Current Configuration**: The event system is currently configured to heavily favor Hard Reset events (100% selection weight) while other events are disabled (0% weight). This creates a more dramatic gameplay experience where score resets occur predictably every 4 questions.

### Event State Management
The event system tracks questions answered and manages active events:
```rust
pub struct EventState {
    pub questions_answered: u32,
    pub active_event: Option<GameEvent>,
    pub event_history: Vec<GameEvent>,
}
```

### Game State Integration
The event system is integrated into the main game state with backward compatibility:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    // ... other fields
    #[serde(default)]  // Backward compatibility with old save files
    pub event_state: EventState,
}
```

### Event Animations
Events include visual animations with different phases:
```rust
pub enum EventAnimationType {
    DoublePointsMultiplication, // Cyan/blue with scaling effects (3s)
    HardResetGlitch,           // Red glitch with screen distortion (4s)
    ReverseQuestionFlip,       // Purple with data stream reversal (2.5s)
}

pub enum AnimationPhase {
    Intro,  // 0-20% of animation
    Main,   // 20-80% of animation
    Outro,  // 80-100% of animation
}
```

### Event Integration
Events are automatically triggered when:
- Question count reaches multiples of 4 (4, 8, 12, etc.)
- No event is currently active or queued
- **Weighted selection heavily favoring Hard Reset events (100% weight vs 0% for others)**

Event effects are applied during gameplay:
- **Double Points**: Modifies scoring for the next question only *(currently disabled)*
- **Hard Reset**: Immediately resets all team scores to zero *(primary active event)*
- **Reverse Question**: Swaps question/answer text for the next selected clue *(currently disabled)*
- **Score Steal**: Transfers 20% of points from highest to lowest scoring team *(currently disabled)*

### Event Timing and Animation
Events use a sophisticated queuing system:
- Events are queued when triggered, not immediately activated
- Animations play during the transition period between cell dialogues
- Interaction blocking prevents cell selection during animations
- Different events have different activation timing (immediate vs. next cell)

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

### Event Operations
```rust
/// Get the current event state
pub fn get_event_state(&self) -> &EventState

/// Check if a specific event is currently active
pub fn is_event_active(&self, event: &GameEvent) -> bool

/// Get the number of questions answered (for event triggering)
pub fn get_questions_answered(&self) -> u32

/// Get the event history
pub fn get_event_history(&self) -> &Vec<GameEvent>

/// Check if an event should be triggered
pub fn should_trigger_event(&self) -> bool
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

### Action Validation with Event Handling
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
                    GameEffect::EventTriggered { event } => {
                        // Show event announcement overlay
                        show_event_announcement(event);
                    }
                    GameEffect::EventAnimation { animation_type } => {
                        // Start event animation
                        start_event_animation(animation_type);
                    }
                    GameEffect::ScoreReset => {
                        // Update all team score displays to zero
                        reset_score_displays();
                    }
                    GameEffect::DoublePointsActivated => {
                        // Show double points indicator
                        show_double_points_indicator();
                    }
                    GameEffect::ReverseQuestionActivated => {
                        // Show reverse question indicator
                        show_reverse_question_indicator();
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

### State Queries with Event System
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

// Check event system status
let event_state = engine.get_event_state();
println!("Questions answered: {}", event_state.questions_answered);

if let Some(active_event) = &event_state.active_event {
    match active_event {
        GameEvent::DoublePoints => {
            // Show double points indicator in UI
            show_double_points_ui();
        }
        GameEvent::ReverseQuestion => {
            // Show reverse question indicator in UI
            show_reverse_question_ui();
        }
        GameEvent::HardReset => {
            // Event is instantaneous, no persistent UI needed
        }
    }
}

// Check if event should trigger soon
if engine.should_trigger_event() {
    // Show "Event incoming!" indicator
    show_event_warning();
}
```

## Error Handling

The GameEngine uses typed errors for comprehensive error handling:

```rust
pub enum GameError {
    InvalidAction { action: String, reason: String },
    EventError(EventError),
}

pub enum EventError {
    NoEventAvailable,
    EventAlreadyActive,
    InvalidEventState,
    AnimationFailed { reason: String },
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
    Err(GameError::EventError(event_error)) => {
        match event_error {
            EventError::EventAlreadyActive => {
                eprintln!("An event is already in progress");
            }
            EventError::NoEventAvailable => {
                eprintln!("No events available to trigger");
            }
            EventError::AnimationFailed { reason } => {
                eprintln!("Event animation failed: {}", reason);
            }
            EventError::InvalidEventState => {
                eprintln!("Event system is in an invalid state");
            }
        }
    }
}
```

## Integration with UI

The GameEngine is designed to integrate cleanly with the egui-based UI:

1. **Event Handling**: Convert UI events to GameActions
2. **Validation**: Use `can_perform_action()` to enable/disable UI elements
3. **State Display**: Use query methods to update UI state
4. **Effects**: Process GameEffects for visual feedback
5. **Error Display**: Show user-friendly error messages from GameError
6. **Event Animations**: Handle event system visual effects through the UI layer

### Event System UI Integration

The event system is fully integrated into the game UI with sophisticated visual effects:

#### Event Animation Controller
The UI manages event animations through the `EventAnimationController`:
```rust
use crate::game::events::{EventAnimationController, GameEvent, EventAnimationType};

// Animation controller is stored in UI memory and updated each frame
let mut event_animation: Option<EventAnimationController> = ui
    .memory_mut(|m| m.data.get_temp(event_animation_id))
    .unwrap_or(None);
```

#### Event Animation Types
Each event type has a unique visual animation:

- **Hard Reset** (`EventAnimationType::HardResetGlitch`) - **Primary Active Animation**:
  - 4-second Matrix-style animation with falling terminal commands
  - Shows green digital rain effect with "SYSTEM RESET" message
  - System reboot sequence with progressive loading lines
  - Enhanced with realistic terminal commands and authentic monospace typography

- **Double Points** (`EventAnimationType::DoublePointsMultiplication`) - *Currently Unused*:
  - 3-second cyan/blue animation with scaling effects
  - Displays "×2" symbol with energy bursts and pulsing rings
  - Point multiplication visualization with particle effects

- **Reverse Question** (`EventAnimationType::ReverseQuestionFlip`) - *Currently Unused*:
  - 2.5-second purple animation with data stream effects
  - Text flipping animation showing "?" transforming to "!"
  - Holographic distortion and mirror effects

- **Score Steal** (`EventAnimationType::ScoreStealHeist`) - *Currently Unused*:
  - 3.2-second heist-themed animation with money bag transfer
  - Shows point transfer between teams with visual effects
  - Displays team names and transfer amounts

#### Animation Lifecycle
```rust
// Check for new event animations from game effects
if event_animation.is_none() {
    if let Some(active_event) = &game_engine.get_state().event_state.active_event {
        let mut controller = EventAnimationController::new();
        let duration = match active_event {
            GameEvent::DoublePoints => Duration::from_millis(3000),
            GameEvent::HardReset => Duration::from_millis(4000),
            GameEvent::ReverseQuestion => Duration::from_millis(2500),
        };
        controller.start_animation(active_event.clone(), duration);
        event_animation = Some(controller);
    }
}

// Update and render active animations
if let Some(mut controller) = event_animation.take() {
    if controller.update() {
        // Animation completed
        event_animation = None;
    } else {
        // Render animation overlay
        draw_event_animation(&painter, rect, &controller);
        event_animation = Some(controller);
    }
}
```

#### Event State Indicators
The UI displays event status through various indicators:
- Active event notifications in the game interface
- Visual feedback for event triggers and completions
- Event history tracking for game session review

#### Interaction Blocking
During event announcements, the UI blocks user interactions to ensure proper event presentation:
```rust
let interaction_blocked = flash.is_some() || pending_answer.is_some() || event_animation.is_some();
```

This architecture ensures that all game logic is centralized and testable while providing a clean interface for the UI layer with rich visual feedback for the event system.

## Recent Changes

### UI Layer Simplification (Latest Update)

The UI layer has been simplified to improve separation of concerns:

- **Phase Transition Management**: Removed explicit phase transition handling from the UI layer. Phase changes are now managed entirely by the game engine through action processing.
- **Memory Management**: Simplified flash animation and pending action cleanup. The UI no longer needs to manually manage phase transition memory cleanup.
- **State Synchronization**: The UI now responds to game state changes rather than managing state transitions directly, improving architectural clarity.

These changes reduce UI complexity while maintaining all functionality, making the codebase more maintainable and reducing the potential for UI-layer bugs.
