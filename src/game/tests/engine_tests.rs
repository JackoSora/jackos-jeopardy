use super::*;
use crate::game::{GameAction, GameActionResult, PlayPhase};

#[test]
fn test_game_engine_creation() {
    let engine = create_test_game_engine();
    assert!(matches!(engine.get_phase(), PlayPhase::Lobby));
    assert_eq!(engine.team_count(), 0);
}

#[test]
fn test_add_team_action() {
    let mut engine = create_test_game_engine();
    let action = GameAction::AddTeam { name: "Test Team".to_string() };
    
    let result = engine.handle_action(action);
    assert!(result.is_ok());
    assert_eq!(engine.team_count(), 1);
}

#[test]
fn test_start_game_action() {
    let mut engine = create_test_game_with_teams();
    let action = GameAction::StartGame;
    
    let result = engine.handle_action(action);
    assert!(result.is_ok());
    assert!(matches!(engine.get_phase(), PlayPhase::Selecting { .. }));
}

#[test]
fn test_start_game_without_teams_fails() {
    let mut engine = create_test_game_engine();
    let action = GameAction::StartGame;
    
    let result = engine.handle_action(action);
    assert!(result.is_err());
    assert!(matches!(engine.get_phase(), PlayPhase::Lobby));
}

#[test]
fn test_select_clue_action() {
    let mut engine = create_game_in_selecting_phase();
    let clue = (0, 0); // First clue
    let team_id = engine.get_state().teams[0].id;
    let action = GameAction::SelectClue { clue, team_id };
    
    let result = engine.handle_action(action);
    assert!(result.is_ok());
    assert!(matches!(engine.get_phase(), PlayPhase::Showing { .. }));
}

#[test]
fn test_answer_correct_action() {
    let mut engine = create_game_in_selecting_phase();
    let clue = (0, 0);
    let team_id = engine.get_state().teams[0].id;
    
    // First select a clue
    let _ = engine.handle_action(GameAction::SelectClue { clue, team_id });
    
    // Then answer correctly
    let action = GameAction::AnswerCorrect { clue, team_id };
    let result = engine.handle_action(action);
    
    assert!(result.is_ok());
    assert!(matches!(engine.get_phase(), PlayPhase::Resolved { .. }));
    
    // Check that points were awarded
    let team_score = engine.get_team_score(team_id);
    assert!(team_score.is_some());
    assert!(team_score.unwrap() > 0);
}

#[test]
fn test_game_engine_query_methods() {
    let engine = create_test_game_with_teams();
    
    // Test team queries
    assert_eq!(engine.team_count(), 2);
    assert!(engine.get_active_team().is_some());
    
    // Test clue queries
    assert!(engine.is_clue_available((0, 0)));
    assert!(!engine.get_available_clues().is_empty());
    
    // Test clue access
    assert!(engine.get_clue((0, 0)).is_some());
    assert!(engine.get_clue((10, 10)).is_none());
}