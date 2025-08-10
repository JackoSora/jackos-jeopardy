use super::*;
use crate::game::{FlashType, GameAction, GameActionResult, GameEffect};

#[test]
fn test_add_team_action_handler() {
    let mut engine = create_test_game_engine();
    let initial_count = engine.team_count();

    let action = GameAction::AddTeam {
        name: "New Team".to_string(),
    };
    let result = engine.handle_action(action);

    assert!(result.is_ok());
    assert_eq!(engine.team_count(), initial_count + 1);

    // Verify the team was added with correct name
    let teams = &engine.get_state().teams;
    assert!(teams.iter().any(|t| t.name == "New Team"));
}

#[test]
fn test_start_game_validation() {
    let mut engine = create_test_game_engine();

    // Should fail without teams
    let result = engine.handle_action(GameAction::StartGame);
    assert!(result.is_err());

    // Add a team and try again
    let _ = engine.handle_action(GameAction::AddTeam {
        name: "Team 1".to_string(),
    });
    let result = engine.handle_action(GameAction::StartGame);
    assert!(result.is_ok());
}

#[test]
fn test_clue_selection_validation() {
    let mut engine = create_game_in_selecting_phase();
    let team_id = engine.get_state().teams[0].id;

    // Valid clue selection
    let valid_action = GameAction::SelectClue {
        clue: (0, 0),
        team_id,
    };
    let result = engine.handle_action(valid_action);
    assert!(result.is_ok());

    // Reset to selecting phase for next test
    let mut engine = create_game_in_selecting_phase();

    // Invalid clue selection (out of bounds)
    let invalid_action = GameAction::SelectClue {
        clue: (10, 10),
        team_id,
    };
    let result = engine.handle_action(invalid_action);
    assert!(result.is_err());
}

#[test]
fn test_answer_correct_effects() {
    let mut engine = create_game_in_selecting_phase();
    let clue = (0, 0);
    let team_id = engine.get_state().teams[0].id;

    // Select clue first
    let _ = engine.handle_action(GameAction::SelectClue { clue, team_id });

    // Answer correctly
    let action = GameAction::AnswerCorrect { clue, team_id };
    let result = engine.handle_action(action);

    assert!(result.is_ok());

    if let Ok(GameActionResult::StateChanged { effects, .. }) = result {
        // Should have score change and flash effects
        let has_score_effect = effects
            .iter()
            .any(|e| matches!(e, GameEffect::ScoreChanged { .. }));
        let has_flash_effect = effects.iter().any(|e| {
            matches!(
                e,
                GameEffect::FlashEffect {
                    effect_type: FlashType::Correct
                }
            )
        });

        assert!(has_score_effect);
        assert!(has_flash_effect);
    }
}

#[test]
fn test_answer_incorrect_creates_steal_phase() {
    let mut engine = create_game_in_selecting_phase();
    let clue = (0, 0);
    let team_id = engine.get_state().teams[0].id;

    // Select clue first
    let _ = engine.handle_action(GameAction::SelectClue { clue, team_id });

    // Answer incorrectly
    let action = GameAction::AnswerIncorrect { clue, team_id };
    let result = engine.handle_action(action);

    assert!(result.is_ok());
    assert!(matches!(engine.get_phase(), PlayPhase::Steal { .. }));
}

#[test]
fn test_steal_attempt_correct() {
    let mut engine = create_game_in_selecting_phase();
    let clue = (0, 0);
    let owner_team_id = engine.get_state().teams[0].id;
    let stealing_team_id = engine.get_state().teams[1].id;

    // Set up steal phase
    let _ = engine.handle_action(GameAction::SelectClue {
        clue,
        team_id: owner_team_id,
    });
    let _ = engine.handle_action(GameAction::AnswerIncorrect {
        clue,
        team_id: owner_team_id,
    });

    // Attempt steal (correct)
    let action = GameAction::StealAttempt {
        clue,
        team_id: stealing_team_id,
        correct: true,
    };
    let result = engine.handle_action(action);

    assert!(result.is_ok());
    assert!(matches!(engine.get_phase(), PlayPhase::Resolved { .. }));

    // Check that stealing team got points
    let stealing_team_score = engine.get_team_score(stealing_team_id);
    assert!(stealing_team_score.is_some());
    assert!(stealing_team_score.unwrap() > 0);
}
