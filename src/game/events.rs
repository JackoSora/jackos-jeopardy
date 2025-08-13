use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Represents the different types of game events that can be triggered
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameEvent {
    DoublePoints,
    HardReset,
    ReverseQuestion,
    /// Lowest score team steals 20% of the points from the leading team
    ScoreSteal,
}

/// Tracks the state of the event system within a game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventState {
    pub questions_answered: u32,
    pub active_event: Option<GameEvent>,
    pub queued_event: Option<GameEvent>,
    pub event_history: Vec<GameEvent>,
    pub animation_playing: bool,
    /// Context for the last score steal event (for UI animation)
    #[serde(default)]
    pub last_steal: Option<StealEventContext>,
}

impl EventState {
    pub fn new() -> Self {
        Self {
            questions_answered: 0,
            active_event: None,
            queued_event: None,
            event_history: Vec::new(),
            animation_playing: false,
            last_steal: None,
        }
    }

    /// Check if an event should be triggered based on question count
    pub fn should_trigger_event(&self) -> bool {
        self.questions_answered > 0
            && self.questions_answered % 4 == 0
            && self.active_event.is_none()
            && self.queued_event.is_none()
    }

    /// Increment the question count when a question is fully resolved
    pub fn increment_question_count(&mut self) {
        self.questions_answered += 1;
    }

    /// Activate an event and add it to history
    pub fn activate_event(&mut self, event: GameEvent) {
        self.event_history.push(event.clone());
        self.active_event = Some(event);
    }

    /// Deactivate the current event
    pub fn deactivate_event(&mut self) {
        self.active_event = None;
    }

    /// Check if a specific event type is currently active
    pub fn is_event_active(&self, event: &GameEvent) -> bool {
        self.active_event.as_ref() == Some(event)
    }

    /// Queue an event for animation during transition period
    pub fn queue_event(&mut self, event: GameEvent) {
        self.queued_event = Some(event);
    }

    /// Check if there's a queued event waiting for animation
    pub fn has_queued_event(&self) -> bool {
        self.queued_event.is_some()
    }

    /// Get and consume the queued event
    pub fn take_queued_event(&mut self) -> Option<GameEvent> {
        self.queued_event.take()
    }

    /// Check if an animation is currently playing
    pub fn is_animation_playing(&self) -> bool {
        self.animation_playing
    }

    /// Set animation playing state
    pub fn set_animation_playing(&mut self, playing: bool) {
        self.animation_playing = playing;
    }
}

impl Default for EventState {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for the event system
#[derive(Debug, Clone)]
pub struct EventConfig {
    pub trigger_interval: u32,
    pub enabled_events: Vec<GameEvent>,
    pub animation_duration: Duration,
}

impl EventConfig {
    pub fn new() -> Self {
        Self {
            trigger_interval: 4,
            enabled_events: vec![
                GameEvent::DoublePoints,
                GameEvent::HardReset,
                GameEvent::ReverseQuestion,
                GameEvent::ScoreSteal,
            ],
            animation_duration: Duration::from_millis(3000),
        }
    }

    /// Get a random event from the enabled events list
    pub fn get_random_event(&self) -> Option<GameEvent> {
        if self.enabled_events.is_empty() {
            return None;
        }

        // Weighted selection: DoublePoints (highest) > ReverseQuestion > ScoreSteal > HardReset (lowest)
        // Only consider events that are enabled.
        let mut events: Vec<GameEvent> = Vec::new();
        let mut weights: Vec<u32> = Vec::new();

        for e in &self.enabled_events {
            let w = match e {
                GameEvent::DoublePoints => 0,
                GameEvent::ReverseQuestion => 0,
                GameEvent::ScoreSteal => 0,
                GameEvent::HardReset => 100,
            };
            events.push(e.clone());
            weights.push(w);
        }

        // Fallback to uniform if something odd happens (e.g., zeroed weights)
        if weights.iter().all(|&w| w == 0) {
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            return events.choose(&mut rng).cloned();
        }

        use rand::distributions::WeightedIndex;
        use rand::prelude::Distribution;
        let dist = WeightedIndex::new(&weights).ok();
        if let Some(dist) = dist {
            let mut rng = rand::thread_rng();
            let idx = dist.sample(&mut rng);
            events.get(idx).cloned()
        } else {
            // If weights invalid, fall back to uniform
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            events.choose(&mut rng).cloned()
        }
    }
}

impl Default for EventConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Animation types for different events
#[derive(Debug, Clone, PartialEq)]
pub enum EventAnimationType {
    DoublePointsMultiplication,
    HardResetGlitch,
    ReverseQuestionFlip,
    ScoreStealHeist,
}

/// Animation phases for event announcements
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationPhase {
    Intro,
    Main,
    Outro,
}

/// Controls event animations
#[derive(Clone)]
pub struct EventAnimationController {
    pub active_animation: Option<EventAnimation>,
    pub animation_start: Instant,
    pub animation_duration: Duration,
}

impl EventAnimationController {
    pub fn new() -> Self {
        Self {
            active_animation: None,
            animation_start: Instant::now(),
            animation_duration: Duration::from_millis(3000),
        }
    }

    /// Start a new event animation
    pub fn start_animation(&mut self, event_type: GameEvent, duration: Duration) {
        self.active_animation = Some(EventAnimation {
            event_type,
            animation_phase: AnimationPhase::Intro,
            progress: 0.0,
        });
        self.animation_start = Instant::now();
        self.animation_duration = duration;
    }

    /// Update animation progress and phase
    pub fn update(&mut self) -> bool {
        if let Some(animation) = &mut self.active_animation {
            let elapsed = self.animation_start.elapsed();
            let progress = elapsed.as_millis() as f32 / self.animation_duration.as_millis() as f32;

            animation.progress = progress.min(1.0);

            // Update animation phase based on progress
            animation.animation_phase = if progress < 0.2 {
                AnimationPhase::Intro
            } else if progress < 0.8 {
                AnimationPhase::Main
            } else {
                AnimationPhase::Outro
            };

            // Return true if animation is complete
            if progress >= 1.0 {
                self.active_animation = None;
                return true;
            }
        }
        false
    }

    /// Check if an animation is currently playing
    pub fn is_animating(&self) -> bool {
        self.active_animation.is_some()
    }

    /// Get the current animation type for rendering
    pub fn get_animation_type(&self) -> Option<EventAnimationType> {
        self.active_animation
            .as_ref()
            .map(|anim| match anim.event_type {
                GameEvent::DoublePoints => EventAnimationType::DoublePointsMultiplication,
                GameEvent::HardReset => EventAnimationType::HardResetGlitch,
                GameEvent::ReverseQuestion => EventAnimationType::ReverseQuestionFlip,
                GameEvent::ScoreSteal => EventAnimationType::ScoreStealHeist,
            })
    }
}

impl Default for EventAnimationController {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents an active event animation
#[derive(Debug, Clone)]
pub struct EventAnimation {
    pub event_type: GameEvent,
    pub animation_phase: AnimationPhase,
    pub progress: f32,
}

/// Errors that can occur in the event system
#[derive(Debug, Clone)]
pub enum EventError {
    NoEventAvailable,
    EventAlreadyActive,
    InvalidEventState,
    AnimationFailed { reason: String },
}

impl std::fmt::Display for EventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventError::NoEventAvailable => write!(f, "No events available to trigger"),
            EventError::EventAlreadyActive => write!(f, "An event is already active"),
            EventError::InvalidEventState => write!(f, "Event system is in an invalid state"),
            EventError::AnimationFailed { reason } => write!(f, "Animation failed: {}", reason),
        }
    }
}

impl std::error::Error for EventError {}

/// Double Points event implementation
pub struct DoublePointsEvent;

impl DoublePointsEvent {
    /// Calculate the modified points for a clue during double points event
    pub fn calculate_points(base_points: u32) -> u32 {
        base_points * 2
    }

    /// Calculate the penalty for incorrect answers during double points event
    pub fn calculate_penalty(base_points: u32) -> i32 {
        (base_points * 2) as i32
    }
}

/// Hard Reset event implementation
pub struct HardResetEvent;

impl HardResetEvent {
    /// Reset all team scores to zero
    pub fn reset_all_scores(teams: &mut [crate::core::Team]) {
        for team in teams.iter_mut() {
            team.score = 0;
        }
    }
}

/// Reverse Question event implementation
pub struct ReverseQuestionEvent;

impl ReverseQuestionEvent {
    /// Swap question and answer for a clue
    pub fn apply_to_clue(clue: &mut crate::core::Clue) {
        std::mem::swap(&mut clue.question, &mut clue.answer);
    }

    /// Restore original question and answer for a clue
    pub fn restore_clue(clue: &mut crate::core::Clue) {
        // Since we swapped them, swapping again restores the original
        std::mem::swap(&mut clue.question, &mut clue.answer);
    }
}

/// Context for ScoreSteal event so the UI can display team names and amount
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StealEventContext {
    pub thief_id: u32,
    pub thief_name: String,
    pub victim_id: u32,
    pub victim_name: String,
    pub amount: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Board, Category, Clue};
    use crate::game::{GameAction, GameEngine};

    #[test]
    fn test_backward_compatibility_deserialization() {
        // Test that we can deserialize old GameState JSON without event_state field
        let old_game_state_json = r#"{
            "board": {
                "categories": [
                    {
                        "name": "Category 1",
                        "clues": [
                            {
                                "id": 1,
                                "points": 100,
                                "question": "Test Question",
                                "answer": "Test Answer",
                                "revealed": false,
                                "solved": false
                            }
                        ]
                    }
                ]
            },
            "teams": [
                {
                    "id": 1,
                    "name": "Team 1",
                    "score": 500
                }
            ],
            "phase": {
                "Selecting": {
                    "team_id": 1
                }
            },
            "active_team": 1,
            "surprise": {
                "pending": null,
                "expires_after_clues": 0
            },
            "ui_map": {
                "logical_to_visual": [
                    [0, 0]
                ]
            }
        }"#;

        let result: Result<crate::game::state::GameState, _> =
            serde_json::from_str(old_game_state_json);
        assert!(
            result.is_ok(),
            "Should be able to deserialize old GameState format"
        );

        let game_state = result.unwrap();
        assert_eq!(game_state.event_state.questions_answered, 0);
        assert_eq!(game_state.event_state.active_event, None);
        assert_eq!(game_state.event_state.event_history.len(), 0);
        assert_eq!(game_state.teams.len(), 1);
        assert_eq!(game_state.teams[0].name, "Team 1");
    }

    #[test]
    fn test_event_state_trigger_detection() {
        let mut event_state = EventState::new();

        // Should not trigger initially
        assert!(!event_state.should_trigger_event());

        // Should not trigger before 4 questions
        for i in 1..4 {
            event_state.increment_question_count();
            assert!(
                !event_state.should_trigger_event(),
                "Should not trigger at {} questions",
                i
            );
        }

        // Should trigger at 4 questions
        event_state.increment_question_count();
        assert!(event_state.should_trigger_event());

        // Should not trigger when event is active
        event_state.activate_event(GameEvent::DoublePoints);
        assert!(!event_state.should_trigger_event());

        // Should trigger again at 8 questions after deactivating
        event_state.deactivate_event();
        for _ in 5..=8 {
            event_state.increment_question_count();
        }
        assert!(event_state.should_trigger_event());
    }

    #[test]
    fn test_event_activation_and_history() {
        let mut event_state = EventState::new();

        // Activate an event
        event_state.activate_event(GameEvent::DoublePoints);
        assert!(event_state.is_event_active(&GameEvent::DoublePoints));
        assert!(!event_state.is_event_active(&GameEvent::HardReset));
        assert_eq!(event_state.event_history.len(), 1);

        // Deactivate and activate another
        event_state.deactivate_event();
        event_state.activate_event(GameEvent::HardReset);
        assert!(!event_state.is_event_active(&GameEvent::DoublePoints));
        assert!(event_state.is_event_active(&GameEvent::HardReset));
        assert_eq!(event_state.event_history.len(), 2);
    }

    #[test]
    fn test_event_config_random_selection() {
        let config = EventConfig::new();

        // Should return some event from the enabled list
        let event = config.get_random_event();
        assert!(event.is_some());
        assert!(config.enabled_events.contains(&event.unwrap()));

        // Empty config should return None
        let empty_config = EventConfig {
            trigger_interval: 5,
            enabled_events: vec![],
            animation_duration: Duration::from_millis(3000),
        };
        assert!(empty_config.get_random_event().is_none());
    }

    #[test]
    fn test_animation_controller_lifecycle() {
        let mut controller = EventAnimationController::new();

        // Initially not animating
        assert!(!controller.is_animating());
        assert!(controller.get_animation_type().is_none());

        // Start animation
        controller.start_animation(GameEvent::DoublePoints, Duration::from_millis(100));
        assert!(controller.is_animating());
        assert_eq!(
            controller.get_animation_type(),
            Some(EventAnimationType::DoublePointsMultiplication)
        );

        // Animation should complete after sufficient time
        std::thread::sleep(Duration::from_millis(150));
        let completed = controller.update();
        assert!(completed);
        assert!(!controller.is_animating());
    }

    #[test]
    fn test_event_trigger_timing() {
        let mut event_state = EventState::new();

        // Test that events trigger exactly every 4 questions
        for i in 1..=20 {
            event_state.increment_question_count();
            if i % 4 == 0 {
                assert!(
                    event_state.should_trigger_event(),
                    "Should trigger at question {}",
                    i
                );
                // Simulate event activation
                event_state.activate_event(GameEvent::DoublePoints);
                assert!(
                    !event_state.should_trigger_event(),
                    "Should not trigger when event is active"
                );
                event_state.deactivate_event();
            } else {
                assert!(
                    !event_state.should_trigger_event(),
                    "Should not trigger at question {}",
                    i
                );
            }
        }
    }

    #[test]
    fn test_double_points_calculation() {
        use super::DoublePointsEvent;

        // Test point multiplication
        assert_eq!(DoublePointsEvent::calculate_points(100), 200);
        assert_eq!(DoublePointsEvent::calculate_points(500), 1000);

        // Test penalty calculation
        assert_eq!(DoublePointsEvent::calculate_penalty(100), 200);
        assert_eq!(DoublePointsEvent::calculate_penalty(300), 600);
    }

    #[test]
    fn test_event_history_tracking() {
        let mut event_state = EventState::new();

        // Test that event history is properly tracked
        assert_eq!(event_state.event_history.len(), 0);

        event_state.activate_event(GameEvent::DoublePoints);
        assert_eq!(event_state.event_history.len(), 1);
        assert_eq!(event_state.event_history[0], GameEvent::DoublePoints);

        event_state.deactivate_event();
        event_state.activate_event(GameEvent::HardReset);
        assert_eq!(event_state.event_history.len(), 2);
        assert_eq!(event_state.event_history[1], GameEvent::HardReset);
    }

    #[test]
    fn test_event_state_persistence() {
        // Test that event state can be serialized and deserialized
        let mut original_state = EventState::new();
        original_state.questions_answered = 7;
        original_state.activate_event(GameEvent::ReverseQuestion);

        let serialized = serde_json::to_string(&original_state).unwrap();
        let deserialized: EventState = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.questions_answered, 7);
        assert_eq!(deserialized.active_event, Some(GameEvent::ReverseQuestion));
        assert_eq!(deserialized.event_history.len(), 1);
    }

    #[test]
    fn test_event_integration_with_game_engine() {
        // Create a test board with minimal clues
        let mut board = Board::default();
        board.categories = vec![Category {
            name: "Test Category".to_string(),
            clues: vec![
                Clue {
                    id: 1,
                    points: 100,
                    question: "Q1".to_string(),
                    answer: "A1".to_string(),
                    revealed: false,
                    solved: false,
                },
                Clue {
                    id: 2,
                    points: 200,
                    question: "Q2".to_string(),
                    answer: "A2".to_string(),
                    revealed: false,
                    solved: false,
                },
                Clue {
                    id: 3,
                    points: 300,
                    question: "Q3".to_string(),
                    answer: "A3".to_string(),
                    revealed: false,
                    solved: false,
                },
                Clue {
                    id: 4,
                    points: 400,
                    question: "Q4".to_string(),
                    answer: "A4".to_string(),
                    revealed: false,
                    solved: false,
                },
                Clue {
                    id: 5,
                    points: 500,
                    question: "Q5".to_string(),
                    answer: "A5".to_string(),
                    revealed: false,
                    solved: false,
                },
            ],
        }];

        let mut engine = GameEngine::new(board);

        // Add a team and start the game
        let _ = engine.handle_action(GameAction::AddTeam {
            name: "Test Team".to_string(),
        });
        let _ = engine.handle_action(GameAction::StartGame);

        // Initially no events should be active
        assert_eq!(engine.get_state().event_state.questions_answered, 0);
        assert!(engine.get_state().event_state.active_event.is_none());

        // Play through 4 questions to trigger an event
        for i in 0..4 {
            // Select clue
            let _ = engine.handle_action(GameAction::SelectClue {
                clue: (0, i),
                team_id: 1,
            });
            // Answer correctly
            let _ = engine.handle_action(GameAction::AnswerCorrect {
                clue: (0, i),
                team_id: 1,
            });
            // Close clue (this increments question count and may trigger event)
            let _ = engine.handle_action(GameAction::CloseClue {
                clue: (0, i),
                next_team_id: 1,
            });
        }

        // After 4 questions, an event should be queued
        assert_eq!(engine.get_state().event_state.questions_answered, 4);
        assert!(engine.get_state().event_state.has_queued_event());
        // Event history is only updated when event is activated, not when queued
        assert_eq!(engine.get_state().event_state.event_history.len(), 0);
    }

    #[test]
    fn test_double_points_event_scoring() {
        let mut board = Board::default();
        board.categories = vec![Category {
            name: "Test Category".to_string(),
            clues: vec![Clue {
                id: 1,
                points: 100,
                question: "Q1".to_string(),
                answer: "A1".to_string(),
                revealed: false,
                solved: false,
            }],
        }];

        let mut engine = GameEngine::new(board);
        let _ = engine.handle_action(GameAction::AddTeam {
            name: "Test Team".to_string(),
        });
        let _ = engine.handle_action(GameAction::StartGame);

        // Manually activate double points event
        engine
            .get_state_mut()
            .event_state
            .activate_event(GameEvent::DoublePoints);

        // Select and answer a clue correctly
        let _ = engine.handle_action(GameAction::SelectClue {
            clue: (0, 0),
            team_id: 1,
        });
        let _ = engine.handle_action(GameAction::AnswerCorrect {
            clue: (0, 0),
            team_id: 1,
        });

        // Team should have double points (200 instead of 100)
        let team_score = engine.get_state().teams[0].score;
        assert_eq!(
            team_score, 200,
            "Team should have double points during Double Points event"
        );

        // Event should be deactivated after the question
        assert!(engine.get_state().event_state.active_event.is_none());
    }

    #[test]
    fn test_hard_reset_event_scoring() {
        let mut board = Board::default();
        board.categories = vec![Category {
            name: "Test Category".to_string(),
            clues: vec![Clue {
                id: 1,
                points: 100,
                question: "Q1".to_string(),
                answer: "A1".to_string(),
                revealed: false,
                solved: false,
            }],
        }];

        let mut engine = GameEngine::new(board);
        let _ = engine.handle_action(GameAction::AddTeam {
            name: "Team 1".to_string(),
        });
        let _ = engine.handle_action(GameAction::AddTeam {
            name: "Team 2".to_string(),
        });
        let _ = engine.handle_action(GameAction::StartGame);

        // Give teams some points first
        engine.get_state_mut().teams[0].score = 500;
        engine.get_state_mut().teams[1].score = 300;

        // Trigger hard reset event
        let _ = engine.handle_action(GameAction::TriggerEvent {
            event: GameEvent::HardReset,
        });

        // All team scores should be reset to 0
        assert_eq!(engine.get_state().teams[0].score, 0);
        assert_eq!(engine.get_state().teams[1].score, 0);
    }

    #[test]
    fn test_reverse_question_event_clue_modification() {
        let mut board = Board::default();
        board.categories = vec![Category {
            name: "Test Category".to_string(),
            clues: vec![Clue {
                id: 1,
                points: 100,
                question: "Original Question".to_string(),
                answer: "Original Answer".to_string(),
                revealed: false,
                solved: false,
            }],
        }];

        let mut engine = GameEngine::new(board);
        let _ = engine.handle_action(GameAction::AddTeam {
            name: "Test Team".to_string(),
        });
        let _ = engine.handle_action(GameAction::StartGame);

        // Manually activate reverse question event
        engine
            .get_state_mut()
            .event_state
            .activate_event(GameEvent::ReverseQuestion);

        // Select a clue (this should trigger the question/answer swap)
        let _ = engine.handle_action(GameAction::SelectClue {
            clue: (0, 0),
            team_id: 1,
        });

        // The question and answer should be swapped
        let clue = &engine.get_state().board.categories[0].clues[0];
        assert_eq!(clue.question, "Original Answer");
        assert_eq!(clue.answer, "Original Question");

        // Answer the reverse question correctly
        let _ = engine.handle_action(GameAction::AnswerCorrect {
            clue: (0, 0),
            team_id: 1,
        });

        // The question and answer should be restored
        let clue = &engine.get_state().board.categories[0].clues[0];
        assert_eq!(clue.question, "Original Question");
        assert_eq!(clue.answer, "Original Answer");

        // Event should be deactivated
        assert!(engine.get_state().event_state.active_event.is_none());
    }

    #[test]
    fn test_score_steal_manual_trigger_transfers_points() {
        // Setup engine with two teams and distinct scores
        let mut board = Board::default();
        board.categories = vec![Category {
            name: "Cat".to_string(),
            clues: vec![Clue {
                id: 1,
                points: 100,
                question: "Q".to_string(),
                answer: "A".to_string(),
                revealed: false,
                solved: false,
            }],
        }];

        let mut engine = GameEngine::new(board);
        let _ = engine.handle_action(GameAction::AddTeam { name: "Low".into() });
        let _ = engine.handle_action(GameAction::AddTeam {
            name: "High".into(),
        });
        let _ = engine.handle_action(GameAction::StartGame);

        // Ensure team[0] is lowest and team[1] is highest
        let low_id = engine.get_state().teams[0].id;
        let high_id = engine.get_state().teams[1].id;
        engine.get_state_mut().teams[0].score = 200;
        engine.get_state_mut().teams[1].score = 1000;

        // Trigger ScoreSteal
        let result = engine.handle_action(GameAction::TriggerEvent {
            event: GameEvent::ScoreSteal,
        });
        assert!(result.is_ok());

        // Expect 20% of highest (1000) transferred = 200
        let state = engine.get_state();
        let low_team = state.teams.iter().find(|t| t.id == low_id).unwrap();
        let high_team = state.teams.iter().find(|t| t.id == high_id).unwrap();
        assert_eq!(low_team.score, 400);
        assert_eq!(high_team.score, 800);

        // Context should be populated
        let ctx = state
            .event_state
            .last_steal
            .as_ref()
            .expect("last_steal context should be set");
        assert_eq!(ctx.thief_id, low_id);
        assert_eq!(ctx.victim_id, high_id);
        assert_eq!(ctx.amount, 200);
    }

    #[test]
    fn test_score_steal_no_transfer_if_equal_scores() {
        // Setup engine with two teams having equal scores
        let mut board = Board::default();
        board.categories = vec![Category {
            name: "Cat".to_string(),
            clues: vec![Clue {
                id: 1,
                points: 100,
                question: "Q".to_string(),
                answer: "A".to_string(),
                revealed: false,
                solved: false,
            }],
        }];

        let mut engine = GameEngine::new(board);
        let _ = engine.handle_action(GameAction::AddTeam { name: "A".into() });
        let _ = engine.handle_action(GameAction::AddTeam { name: "B".into() });
        let _ = engine.handle_action(GameAction::StartGame);

        let a_id = engine.get_state().teams[0].id;
        let b_id = engine.get_state().teams[1].id;
        engine.get_state_mut().teams[0].score = 500;
        engine.get_state_mut().teams[1].score = 500;

        let result = engine.handle_action(GameAction::TriggerEvent {
            event: GameEvent::ScoreSteal,
        });
        assert!(result.is_ok());

        // No transfer should occur; scores unchanged
        let state = engine.get_state();
        let a = state.teams.iter().find(|t| t.id == a_id).unwrap();
        let b = state.teams.iter().find(|t| t.id == b_id).unwrap();
        assert_eq!(a.score, 500);
        assert_eq!(b.score, 500);
        assert!(state.event_state.last_steal.is_none());
    }
}
