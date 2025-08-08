use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::domain::{Board, SurpriseState, Team, UiMapping};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayPhase {
    Lobby,
    Selecting {
        team_id: u32,
    },
    Showing {
        clue: (usize, usize),
        owner_team_id: u32,
    },
    Steal {
        clue: (usize, usize),
        queue: VecDeque<u32>,
        current: u32,
        owner_team_id: u32,
    },
    Resolved {
        clue: (usize, usize),
        next_team_id: u32,
    },
    Intermission,
    Finished,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub board: Board,
    pub teams: Vec<Team>,
    pub phase: PlayPhase,
    pub active_team: u32,
    pub surprise: SurpriseState,
    pub ui_map: UiMapping,
}

impl GameState {
    pub fn new(board: Board) -> Self {
        let num_rows = board.categories.get(0).map(|c| c.clues.len()).unwrap_or(0);
        Self {
            board: board.clone(),
            teams: Vec::new(),
            phase: PlayPhase::Lobby,
            active_team: 0,
            surprise: SurpriseState::default(),
            ui_map: UiMapping::identity(board.categories.len(), num_rows),
        }
    }

    pub fn add_team(&mut self, name: String) {
        let next_id: u32 = self
            .teams
            .iter()
            .map(|t| t.id)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        self.teams.push(Team {
            id: next_id,
            name,
            score: 0,
        });
        if matches!(self.phase, PlayPhase::Lobby) && self.active_team == 0 {
            self.active_team = next_id;
        }
    }

    pub fn rotate_to_next_active_team(&mut self) {
        if self.teams.is_empty() {
            return;
        }
        if let Some(pos) = self.teams.iter().position(|t| t.id == self.active_team) {
            let next_index = (pos + 1) % self.teams.len();
            self.active_team = self.teams[next_index].id;
        } else {
            self.active_team = self.teams[0].id;
        }
    }
}
