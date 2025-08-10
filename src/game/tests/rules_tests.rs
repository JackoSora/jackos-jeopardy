use super::*;
use crate::game::rules::GameRules;
use crate::game::GameAction;

#[test]
fn test_can_add_team_rules() {
    let rules = GameRules::new();
    let engine = create_test_game_engine();

    // Should be able to add teams in lobby
    assert!(rules.can_add_team(engine.get_state()));

    // Should not be able to add teams after game starts
    let engine = create_game_in_selecting_phase();
    assert!(!rules.can_add_team(engine.get_state()));
}

#[test]
fn test_can_start_game_rules() {
    let rules = GameRules::new();
    let engine = create_test_game_engine();

    // Should not be able to start without teams
    assert!(!rules.can_start_game(engine.get_state()));

    // Should be able to start with teams
    let engine = create_test_game_with_teams();
    assert!(rules.can_start_game(engine.get_state()));
}

#[test]
fn test_can_select_clue_rules() {
    let rules = GameRules::new();
    let engine = create_game_in_selecting_phase();

    // Should be able to select available clues in selecting phase
    assert!(rules.can_select_clue(engine.get_state(), (0, 0)));

    // Should not be able to select invalid clues
    assert!(!rules.can_select_clue(engine.get_state(), (10, 10)));

    // Should not be able to select clues in lobby
    let lobby_engine = create_test_game_with_teams();
    assert!(!rules.can_select_clue(lobby_engine.get_state(), (0, 0)));
}

#[test]
fn test_validate_team_action() {
    let rules = GameRules::new();
    let engine = create_game_in_selecting_phase();
    let team_id = engine.get_state().teams[0].id;
    let other_team_id = engine.get_state().teams[1].id;

    // Active team should be able to select clues
    let select_action = GameAction::SelectClue {
        clue: (0, 0),
        team_id,
    };
    assert!(rules.validate_team_action(engine.get_state(), team_id, &select_action));

    // Non-active team should not be able to select clues
    let invalid_select = GameAction::SelectClue {
        clue: (0, 0),
        team_id: other_team_id,
    };
    assert!(!rules.validate_team_action(engine.get_state(), other_team_id, &invalid_select));
}

#[test]
fn test_get_steal_queue() {
    let rules = GameRules::new();
    let engine = create_test_game_with_teams();
    let owner_team_id = engine.get_state().teams[0].id;

    let steal_queue = rules.get_steal_queue(engine.get_state(), owner_team_id);

    // Should contain all teams except the owner
    assert_eq!(steal_queue.len(), 1);
    assert!(!steal_queue.contains(&owner_team_id));
}

#[test]
fn test_is_game_finished() {
    let rules = GameRules::new();
    let engine = create_test_game_engine();

    // Game should not be finished initially
    assert!(!rules.is_game_finished(engine.get_state()));

    // Create a game state where all clues are solved
    let mut engine = create_test_game_engine();
    let state = engine.get_state_mut();
    for category in &mut state.board.categories {
        for clue in &mut category.clues {
            clue.solved = true;
        }
    }

    assert!(rules.is_game_finished(engine.get_state()));
}

#[test]
fn test_get_available_actions() {
    let rules = GameRules::new();

    // In lobby, should be able to add teams and start game (if teams exist)
    let engine = create_test_game_with_teams();
    let actions = rules.get_available_actions(engine.get_state());

    assert!(
        actions
            .iter()
            .any(|a| matches!(a, GameAction::AddTeam { .. }))
    );
    assert!(actions.iter().any(|a| matches!(a, GameAction::StartGame)));

    // In selecting phase, should be able to select clues
    let engine = create_game_in_selecting_phase();
    let actions = rules.get_available_actions(engine.get_state());

    assert!(
        actions
            .iter()
            .any(|a| matches!(a, GameAction::SelectClue { .. }))
    );
}
