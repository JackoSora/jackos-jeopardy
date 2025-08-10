use crate::domain::Team;
use crate::game::scoring::ScoringEngine;

#[test]
fn test_award_points() {
    let scoring = ScoringEngine::new();
    let mut teams = vec![
        Team {
            id: 1,
            name: "Team 1".to_string(),
            score: 0,
        },
        Team {
            id: 2,
            name: "Team 2".to_string(),
            score: 0,
        },
    ];

    // Award points to team 1
    let result = scoring.award_points(&mut teams, 1, 100);
    assert!(result);
    assert_eq!(teams[0].score, 100);

    // Award points to non-existent team
    let result = scoring.award_points(&mut teams, 999, 100);
    assert!(!result);
}

#[test]
fn test_deduct_points() {
    let scoring = ScoringEngine::new();
    let mut teams = vec![Team {
        id: 1,
        name: "Team 1".to_string(),
        score: 200,
    }];

    // Deduct points
    let result = scoring.deduct_points(&mut teams, 1, 50);
    assert!(result);
    assert_eq!(teams[0].score, 150);

    // Deduct from non-existent team
    let result = scoring.deduct_points(&mut teams, 999, 50);
    assert!(!result);
}

#[test]
fn test_get_team_score() {
    let scoring = ScoringEngine::new();
    let teams = vec![
        Team {
            id: 1,
            name: "Team 1".to_string(),
            score: 150,
        },
        Team {
            id: 2,
            name: "Team 2".to_string(),
            score: 75,
        },
    ];

    assert_eq!(scoring.get_team_score(&teams, 1), Some(150));
    assert_eq!(scoring.get_team_score(&teams, 2), Some(75));
    assert_eq!(scoring.get_team_score(&teams, 999), None);
}

#[test]
fn test_get_leaderboard() {
    let scoring = ScoringEngine::new();
    let teams = vec![
        Team {
            id: 1,
            name: "Team A".to_string(),
            score: 100,
        },
        Team {
            id: 2,
            name: "Team B".to_string(),
            score: 200,
        },
        Team {
            id: 3,
            name: "Team C".to_string(),
            score: 150,
        },
    ];

    let leaderboard = scoring.get_leaderboard(&teams);

    // Should be sorted by score descending
    assert_eq!(leaderboard.len(), 3);
    assert_eq!(leaderboard[0], (2, "Team B".to_string(), 200));
    assert_eq!(leaderboard[1], (3, "Team C".to_string(), 150));
    assert_eq!(leaderboard[2], (1, "Team A".to_string(), 100));
}

#[test]
fn test_add_team() {
    let scoring = ScoringEngine::new();
    let mut teams = vec![Team {
        id: 1,
        name: "Team 1".to_string(),
        score: 0,
    }];

    let new_team_id = scoring.add_team(&mut teams, "Team 2".to_string());

    assert_eq!(teams.len(), 2);
    assert_eq!(new_team_id, 2);
    assert_eq!(teams[1].name, "Team 2");
    assert_eq!(teams[1].score, 0);
}

#[test]
fn test_rotate_active_team() {
    let scoring = ScoringEngine::new();
    let teams = vec![
        Team {
            id: 1,
            name: "Team 1".to_string(),
            score: 0,
        },
        Team {
            id: 2,
            name: "Team 2".to_string(),
            score: 0,
        },
        Team {
            id: 3,
            name: "Team 3".to_string(),
            score: 0,
        },
    ];

    // Rotate from team 1 to team 2
    let next_team = scoring.rotate_active_team(&teams, 1);
    assert_eq!(next_team, 2);

    // Rotate from team 3 back to team 1 (wrap around)
    let next_team = scoring.rotate_active_team(&teams, 3);
    assert_eq!(next_team, 1);

    // Handle non-existent current team
    let next_team = scoring.rotate_active_team(&teams, 999);
    assert_eq!(next_team, 1); // Should default to first team
}

#[test]
fn test_team_exists() {
    let scoring = ScoringEngine::new();
    let teams = vec![
        Team {
            id: 1,
            name: "Team 1".to_string(),
            score: 0,
        },
        Team {
            id: 2,
            name: "Team 2".to_string(),
            score: 0,
        },
    ];

    assert!(scoring.team_exists(&teams, 1));
    assert!(scoring.team_exists(&teams, 2));
    assert!(!scoring.team_exists(&teams, 999));
}

#[test]
fn test_get_team_stats() {
    let scoring = ScoringEngine::new();
    let teams = vec![
        Team {
            id: 1,
            name: "Team 1".to_string(),
            score: 100,
        },
        Team {
            id: 2,
            name: "Team 2".to_string(),
            score: 200,
        },
        Team {
            id: 3,
            name: "Team 3".to_string(),
            score: 50,
        },
    ];

    let stats = scoring.get_team_stats(&teams);

    assert_eq!(stats.total_teams, 3);
    assert_eq!(stats.highest_score, 200);
    assert_eq!(stats.lowest_score, 50);
    assert_eq!(stats.total_points, 350);
    assert!((stats.average_score - 116.67).abs() < 0.1); // Approximately 116.67
}

#[test]
fn test_empty_teams_stats() {
    let scoring = ScoringEngine::new();
    let teams = vec![];

    let stats = scoring.get_team_stats(&teams);

    assert_eq!(stats.total_teams, 0);
    assert_eq!(stats.highest_score, 0);
    assert_eq!(stats.lowest_score, 0);
    assert_eq!(stats.total_points, 0);
    assert_eq!(stats.average_score, 0.0);
}
