# Event System Documentation

The Party Jeopardy! event system adds dynamic gameplay modifiers that trigger automatically during the game to create excitement and strategic depth.

## Overview

Events are triggered automatically every 4 questions answered, regardless of whether they were answered correctly or incorrectly. When triggered, a random event is selected and applied to the game with accompanying visual animations. The event system includes sophisticated animation timing control to ensure smooth gameplay flow without interference during cell selection.

## Event Types

### Double Points Event
- **Trigger**: Random selection when event threshold is reached
- **Duration**: Active until the next question is fully resolved
- **Effect**: 
  - Correct answers award double points (e.g., 200 → 400)
  - Incorrect answers deduct double points as penalty
- **Animation**: 3-second cyan/blue multiplication effects with energy bursts
- **Strategy**: High risk, high reward - teams must decide if the potential double points are worth the double penalty risk

### Hard Reset Event  
- **Trigger**: Random selection when event threshold is reached
- **Duration**: Instantaneous
- **Effect**: All team scores are immediately reset to zero
- **Animation**: 4-second red glitch effects with screen distortion and digital artifacts
- **Strategy**: Great equalizer - gives trailing teams a fresh start while leaders lose their advantage

### Reverse Question Event
- **Trigger**: Random selection when event threshold is reached  
- **Duration**: Active until the next question is fully resolved
- **Effect**: The next selected clue has its question and answer text swapped
- **Animation**: 2.5-second purple data stream reversal with holographic distortion
- **Strategy**: Tests knowledge from a different angle - players must think about what question would produce the given "answer"

### Score Steal Event
- **Trigger**: Random selection when event threshold is reached
- **Duration**: Instantaneous
- **Effect**: The lowest-scoring team steals 20% of the leading team's current points (floored), applied immediately upon queuing/trigger.
- **Animation**: ~3.2-second heist animation showing a money bag moving from the leader to the lowest team, with team names and +/- amounts.
- **Strategy**: Momentum shift mechanic—helps trailing teams catch up and keeps competition tight.

## Technical Implementation

### Core Data Structures

```rust
/// Event types available in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameEvent {
    DoublePoints,
    HardReset,
    ReverseQuestion,
}

/// Tracks event system state within a game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventState {
    pub questions_answered: u32,
    pub active_event: Option<GameEvent>,
    pub queued_event: Option<GameEvent>,
    pub event_history: Vec<GameEvent>,
    pub animation_playing: bool,
}

/// Configuration for event system behavior
#[derive(Debug, Clone)]
pub struct EventConfig {
    pub trigger_interval: u32,           // Questions between events (default: 4)
    pub enabled_events: Vec<GameEvent>,  // Which events can be triggered
    pub animation_duration: Duration,    // Base animation duration
}
```

### Game State Integration

The event system is integrated into the main `GameState` struct with backward compatibility:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub board: Board,
    pub teams: Vec<Team>,
    pub phase: PlayPhase,
    pub active_team: u32,
    pub surprise: SurpriseState,
    pub ui_map: UiMapping,
    #[serde(default)]  // Ensures backward compatibility with old save files
    pub event_state: EventState,
}
```

The `#[serde(default)]` attribute ensures that when loading old game save files that don't contain event state data, a new `EventState` with default values is automatically created. This maintains compatibility with existing save files while enabling the new event system functionality.

### Event Triggering Logic

Events are triggered when:
1. `questions_answered % 4 == 0` (every 4th question)
2. `questions_answered > 0` (not on the very first question)
3. `active_event.is_none()` (no event currently active)
4. `queued_event.is_none()` (no event currently queued)

```rust
impl EventState {
    /// Check if an event should be triggered
    pub fn should_trigger_event(&self) -> bool {
        self.questions_answered > 0 
            && self.questions_answered % 4 == 0
            && self.active_event.is_none()
            && self.queued_event.is_none()
    }
}
```

### Event Selection

Events are selected randomly with equal probability from the enabled events list:

```rust
impl EventConfig {
    /// Get a random event from enabled events
    pub fn get_random_event(&self) -> Option<GameEvent> {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        self.enabled_events.choose(&mut rng).cloned()
    }
}
```

### Animation System

Each event has a corresponding animation with three phases:

```rust
/// Animation phases for smooth transitions
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationPhase {
    Intro,  // 0-20% of animation duration
    Main,   // 20-80% of animation duration  
    Outro,  // 80-100% of animation duration
}

/// Animation types with distinct visual styles
#[derive(Debug, Clone, PartialEq)]
pub enum EventAnimationType {
    DoublePointsMultiplication, // Scaling effects, cyan/blue colors
    HardResetGlitch,           // Screen distortion, red error colors
    ReverseQuestionFlip,       // Data streams, purple/magenta colors
    ScoreStealHeist,           // Money-bag transfer with team labels
}
```

The `EventAnimationController` manages animation lifecycle:

```rust
impl EventAnimationController {
    /// Start a new event animation
    pub fn start_animation(&mut self, event_type: GameEvent, duration: Duration);
    
    /// Update animation progress (call each frame)
    pub fn update(&mut self) -> bool; // Returns true when complete
    
    /// Check if animation is currently playing
    pub fn is_animating(&self) -> bool;
    
    /// Get current animation type for rendering
    pub fn get_animation_type(&self) -> Option<EventAnimationType>;
}
```

## Animation Timing and Queuing System

The event system includes sophisticated timing control to ensure animations play at the correct moment without interfering with gameplay:

### Event Queuing
When an event is triggered (every 4 questions), it is first queued rather than immediately activated:
- Events are queued during the `handle_close_clue` action
- Queued events wait for the transition period between closing one cell and opening the next
- This prevents animation loops and ensures smooth gameplay flow

### Animation Phases
Event animations are managed through the `EventAnimationController`:

```rust
pub struct EventAnimationController {
    pub active_animation: Option<EventAnimation>,
    pub animation_start: Instant,
    pub animation_duration: Duration,
}

pub struct EventAnimation {
    pub event_type: GameEvent,
    pub animation_phase: AnimationPhase,
    pub progress: f32,
}

pub enum AnimationPhase {
    Intro,  // 0-20% of animation duration
    Main,   // 20-80% of animation duration
    Outro,  // 80-100% of animation duration
}
```

### Timing Control
- **Event Detection**: Occurs when closing a clue dialogue after 4 questions
- **Event Queuing**: Event is stored in `EventState.queued_event`
- **Animation Trigger**: Animation plays immediately after cell dialogue closes
- **Effect Application**: 
  - Hard Reset: Applied immediately when queued
  - Double Points/Reverse Question: Applied when animation starts for next cell
    - Score Steal: Applied immediately when queued (and when manually triggered), with UI context captured for names/amount
- **Interaction Blocking**: Cell selection is blocked during animation playback

### State Management
The event system tracks multiple states to ensure proper timing:
- `questions_answered`: Tracks progress toward next event trigger
- `queued_event`: Stores event waiting for animation during transition
- `active_event`: Currently active event affecting gameplay
- `animation_playing`: Prevents interactions during animation
- `event_history`: Complete record of triggered events

## Integration with Game Actions

### Event-Related Actions

```rust
pub enum GameAction {
    // ... existing actions
    QueueEvent { event: GameEvent },      // Queue an event for transition period
    PlayEventAnimation { event: GameEvent }, // Start event animation
    TriggerEvent { event: GameEvent },    // Manually trigger an event (testing)
    AcknowledgeEvent,                     // User acknowledges event announcement
    ResolveEvent,                         // Deactivate current event
}
```

### Event Effects

Events generate visual effects for the UI layer:

```rust
pub enum GameEffect {
    // ... existing effects
    EventTriggered { event: GameEvent },           // Show event announcement
    EventQueued { event: GameEvent },              // Event queued for transition
    EventAnimation { animation_type: EventAnimationType }, // Start animation
    ScoreReset,                                    // Update score displays
    DoublePointsActivated,                         // Show double points indicator
    ReverseQuestionActivated,                      // Show reverse question indicator
    ScoreStealApplied { context: StealEventContext }, // Applied immediately, drives heist overlay labels
}
```

### Automatic Event Triggering

Events are automatically triggered in the `handle_close_clue` action:

```rust
fn handle_close_clue(&self, state: &mut GameState, clue: (usize, usize), next_team_id: u32) -> Result<GameActionResult, GameError> {
    // Increment question count
    state.event_state.increment_question_count();
    
    // Check for event trigger
    if state.event_state.should_trigger_event() {
        if let Some(event) = EventConfig::default().get_random_event() {
            // Trigger event and collect effects
            match self.handle_trigger_event(state, event) {
                Ok(GameActionResult::StateChanged { effects, .. }) => {
                    // Add event effects to close clue effects
                }
                // ... handle other cases
            }
        }
    }
    
    // Continue with normal clue closing logic
}
```

## Event-Specific Logic

### Double Points Event

```rust
pub struct DoublePointsEvent;

impl DoublePointsEvent {
    /// Calculate doubled points for correct answers
    pub fn calculate_points(base_points: u32) -> u32 {
        base_points * 2
    }
    
    /// Calculate doubled penalty for incorrect answers  
    pub fn calculate_penalty(base_points: u32) -> i32 {
        (base_points * 2) as i32
    }
}
```

Applied in scoring logic:
```rust
let points = if state.event_state.is_event_active(&GameEvent::DoublePoints) {
    DoublePointsEvent::calculate_points(clue.points) as i32
} else {
    clue.points as i32
};
```

### Hard Reset Event

```rust
pub struct HardResetEvent;

impl HardResetEvent {
    /// Reset all team scores to zero
    pub fn reset_all_scores(teams: &mut [Team]) {
        for team in teams.iter_mut() {
            team.score = 0;
        }
    }
}
```

Applied immediately when event is triggered:
```rust
GameEvent::HardReset => {
    for team in &mut state.teams {
        team.score = 0;
    }
    effects.push(GameEffect::ScoreReset);
}
```

### Reverse Question Event

```rust
pub struct ReverseQuestionEvent;

impl ReverseQuestionEvent {
    /// Swap question and answer text
    pub fn apply_to_clue(clue: &mut Clue) {
        std::mem::swap(&mut clue.question, &mut clue.answer);
    }
    
    /// Restore original question and answer
    pub fn restore_clue(clue: &mut Clue) {
        std::mem::swap(&mut clue.question, &mut clue.answer);
    }
}
```

Applied when clue is selected and restored when resolved:
```rust
// On clue selection
if state.event_state.is_event_active(&GameEvent::ReverseQuestion) {
    if let Some(clue) = get_clue_mut(clue_coords) {
        ReverseQuestionEvent::apply_to_clue(clue);
    }
}

// On clue resolution  
if state.event_state.is_event_active(&GameEvent::ReverseQuestion) {
    if let Some(clue) = get_clue_mut(clue_coords) {
        ReverseQuestionEvent::restore_clue(clue);
    }
    state.event_state.deactivate_event();
}
```

## UI Integration Guidelines

### Event Announcements

When `GameEffect::EventTriggered` is received:
1. Show full-screen event announcement overlay
2. Block all game interactions during announcement
3. Display event name, description, and visual theme
4. Auto-dismiss after animation completes or user acknowledges

### Event Animations

When `GameEffect::EventAnimation` is received:
1. Start the appropriate animation based on `EventAnimationType`
2. Use event-specific color schemes and effects
3. Ensure animations don't interfere with gameplay readability
4. Provide option to skip/speed up animations for accessibility

### Active Event Indicators

While events are active, show persistent UI indicators:
- **Double Points**: Glowing multiplier icon near score displays
- **Reverse Question**: Flip/mirror icon near clue selection area
- **Hard Reset**: No persistent indicator (instantaneous effect)

### Event History

Consider showing event history in game statistics:
- List of events that occurred during the game
- Question numbers when events were triggered
- Impact on final scores

## Testing Considerations

### Unit Tests

The event system includes comprehensive unit tests covering all major functionality:

#### Core Event System Tests
- **Event trigger detection**: Validates events trigger exactly every 4 questions
- **Event activation/deactivation lifecycle**: Tests event state transitions
- **Random event selection**: Ensures proper distribution and empty list handling
- **Animation controller state management**: Tests animation lifecycle and timing
- **Event queuing system**: Validates queuing, dequeuing, and state management
- **Backward compatibility**: Ensures old save files load correctly with default event state

#### Event-Specific Logic Tests
- **Double Points calculation**: Tests point multiplication and penalty calculation
- **Hard Reset functionality**: Validates immediate score reset for all teams
- **Reverse Question logic**: Tests question/answer swapping and restoration
- **Event integration**: Tests event effects within full game engine context

#### Game Engine Integration Tests
- **Action processing**: Tests all event-related game actions
- **State persistence**: Validates event state serialization/deserialization
- **Effect generation**: Tests proper GameEffect generation for UI feedback
- **Error handling**: Tests event-specific error conditions and recovery

### Integration Tests

Test event system integration with complete game flow:
- Events trigger at correct question counts (4, 8, 12, etc.)
- Event effects apply correctly to gameplay mechanics
- Event state persists across game sessions with backward compatibility
- UI properly handles all event effects and animations
- Animation timing works correctly with cell selection flow
- Interaction blocking functions properly during animations

### Manual Testing Scenarios

1. **Event Triggering**: Play through 4+ questions and verify events trigger at correct intervals
2. **Double Points**: Test both correct and incorrect answers during event, verify double scoring
3. **Hard Reset**: Verify all team scores reset immediately when event triggers
4. **Reverse Question**: Confirm question/answer swap on selection and restoration on resolution
5. **Animation Flow**: Test all animation types, phases, and timing
6. **Queuing System**: Verify events queue properly and animations play during transitions
7. **Interaction Blocking**: Confirm cell selection is blocked during animations
8. **Edge Cases**: Test event triggering with different team counts, game states, and save/load scenarios
9. **Backward Compatibility**: Load old save files and verify event system initializes correctly

## Performance Considerations

- Event state is lightweight and serializable
- Animation calculations are frame-rate independent
- Random event selection is O(1) with pre-configured event list
- Event effects are applied efficiently without game state copying
- Animation controller uses minimal memory allocation

## Test Coverage

The event system includes extensive test coverage with over 15 comprehensive unit tests:

### Core Functionality Tests
- `test_backward_compatibility_deserialization`: Ensures old save files load correctly
- `test_event_state_trigger_detection`: Validates trigger timing at 4-question intervals
- `test_event_activation_and_history`: Tests event lifecycle and history tracking
- `test_event_config_random_selection`: Validates random event selection logic
- `test_animation_controller_lifecycle`: Tests animation state management
- `test_event_trigger_timing`: Comprehensive timing validation across 20 questions

### Event-Specific Tests
- `test_double_points_calculation`: Tests point multiplication and penalty logic
- `test_event_history_tracking`: Validates event history persistence
- `test_event_state_persistence`: Tests serialization/deserialization
- `test_double_points_event_scoring`: Integration test with game engine
- `test_hard_reset_event_scoring`: Tests immediate score reset functionality
- `test_reverse_question_event_clue_modification`: Tests question/answer swapping

### Integration Tests
- `test_event_integration_with_game_engine`: Full game flow with event triggering
- Tests cover complete game scenarios with multiple teams and question sequences
- Validates event effects apply correctly within game context
- Tests event state management across game sessions

All tests use realistic game scenarios with proper board setup, team management, and action sequences to ensure the event system works correctly in real gameplay situations.

## Future Enhancements

Potential additions to the event system:
- **Team-Specific Events**: Events that affect only certain teams
- **Category Events**: Events that modify entire categories
- **Combo Events**: Multiple events active simultaneously
- **Custom Events**: User-defined events with configurable effects
- **Event Probability**: Weighted random selection instead of equal probability
- **Event Cooldowns**: Prevent same event from triggering consecutively
- **Event Preview**: Show upcoming event indicators to build anticipation
- **Event Statistics**: Track event frequency and impact on game outcomes