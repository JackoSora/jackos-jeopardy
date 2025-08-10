use crate::domain::Team;

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

    /// Get the score of a specific team
    pub fn get_team_score(&self, teams: &[Team], team_id: u32) -> Option<i32> {
        teams.iter().find(|t| t.id == team_id).map(|t| t.score)
    }

    /// Get a leaderboard sorted by score (highest first)
    pub fn get_leaderboard(&self, teams: &[Team]) -> Vec<(u32, String, i32)> {
        let mut leaderboard: Vec<(u32, String, i32)> = teams
            .iter()
            .map(|t| (t.id, t.name.clone(), t.score))
            .collect();
        
        // Sort by score descending
        leaderboard.sort_by(|a, b| b.2.cmp(&a.2));
        leaderboard
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

    /// Rotate to the next active team in the list
    pub fn rotate_active_team(&self, teams: &[Team], current_active: u32) -> u32 {
        if teams.is_empty() {
            return current_active;
        }
        
        if let Some(pos) = teams.iter().position(|t| t.id == current_active) {
            let next_index = (pos + 1) % teams.len();
            teams[next_index].id
        } else {
            // If current active team not found, return first team
            teams[0].id
        }
    }

    /// Get team statistics
    pub fn get_team_stats(&self, teams: &[Team]) -> TeamStats {
        if teams.is_empty() {
            return TeamStats {
                total_teams: 0,
                highest_score: 0,
                lowest_score: 0,
                average_score: 0.0,
                total_points: 0,
            };
        }

        let scores: Vec<i32> = teams.iter().map(|t| t.score).collect();
        let total_points: i32 = scores.iter().sum();
        let highest_score = *scores.iter().max().unwrap_or(&0);
        let lowest_score = *scores.iter().min().unwrap_or(&0);
        let average_score = if teams.is_empty() { 0.0 } else { total_points as f64 / teams.len() as f64 };

        TeamStats {
            total_teams: teams.len(),
            highest_score,
            lowest_score,
            average_score,
            total_points,
        }
    }

    /// Check if a team exists
    pub fn team_exists(&self, teams: &[Team], team_id: u32) -> bool {
        teams.iter().any(|t| t.id == team_id)
    }

    /// Get team by ID
    pub fn get_team<'a>(&self, teams: &'a [Team], team_id: u32) -> Option<&'a Team> {
        teams.iter().find(|t| t.id == team_id)
    }

    /// Update team name
    pub fn update_team_name(&self, teams: &mut Vec<Team>, team_id: u32, new_name: String) -> bool {
        if let Some(team) = teams.iter_mut().find(|t| t.id == team_id) {
            team.name = new_name;
            true
        } else {
            false
        }
    }

    /// Remove a team (useful for lobby phase)
    pub fn remove_team(&self, teams: &mut Vec<Team>, team_id: u32) -> bool {
        if let Some(pos) = teams.iter().position(|t| t.id == team_id) {
            teams.remove(pos);
            true
        } else {
            false
        }
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