use crate::core::Team;

#[derive(Debug)]
pub struct ScoringEngine;

impl ScoringEngine {
    pub fn new() -> Self {
        Self
    }

    /// Award points to a specific team
    pub fn award_points(&self, teams: &mut Vec<Team>, team_id: u32, points: i32) -> bool {
        if let Some(team) = teams.iter_mut().find(|t| t.id == team_id) {
            team.score += points;
            true
        } else {
            false
        }
    }

    /// Deduct points from a specific team
    pub fn deduct_points(&self, teams: &mut Vec<Team>, team_id: u32, points: i32) -> bool {
        if let Some(team) = teams.iter_mut().find(|t| t.id == team_id) {
            team.score -= points;
            true
        } else {
            false
        }
    }


    /// Add a new team and return its ID
    pub fn add_team(&self, teams: &mut Vec<Team>, name: String) -> u32 {
        let next_id: u32 = teams
            .iter()
            .map(|t| t.id)
            .max()
            .unwrap_or(0)
            .saturating_add(1);

        teams.push(Team {
            id: next_id,
            name,
            score: 0,
        });

        next_id
    }

    // API methods for tests
    pub fn get_team_stats(&self, teams: &[Team]) -> TeamStats {
        TeamStats {
            total_teams: teams.len(),
            highest_score: teams.iter().map(|t| t.score).max().unwrap_or(0),
            lowest_score: teams.iter().map(|t| t.score).min().unwrap_or(0),
            average_score: if teams.is_empty() { 0.0 } else {
                teams.iter().map(|t| t.score).sum::<i32>() as f64 / teams.len() as f64
            },
            total_points: teams.iter().map(|t| t.score).sum(),
        }
    }

    pub fn get_team_score(&self, teams: &[Team], team_id: u32) -> Option<i32> {
        teams.iter().find(|t| t.id == team_id).map(|t| t.score)
    }

    pub fn get_leaderboard(&self, teams: &[Team]) -> Vec<(u32, String, i32)> {
        let mut leaderboard: Vec<(u32, String, i32)> = teams
            .iter()
            .map(|t| (t.id, t.name.clone(), t.score))
            .collect();
        leaderboard.sort_by(|a, b| b.2.cmp(&a.2));
        leaderboard
    }

    pub fn rotate_active_team(&self, teams: &[Team], current_active: u32) -> u32 {
        if teams.is_empty() {
            return current_active;
        }
        if let Some(pos) = teams.iter().position(|t| t.id == current_active) {
            let next_index = (pos + 1) % teams.len();
            teams[next_index].id
        } else {
            teams[0].id
        }
    }

    pub fn team_exists(&self, teams: &[Team], team_id: u32) -> bool {
        teams.iter().any(|t| t.id == team_id)
    }

}

#[derive(Debug, Clone)]
pub struct TeamStats {
    pub total_teams: usize,
    pub highest_score: i32,
    pub lowest_score: i32,
    pub average_score: f64,
    pub total_points: i32,
}

