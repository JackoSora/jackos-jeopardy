use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::core::{Board, Clue, SurpriseState, Team, UiMapping};
use crate::game::events::EventState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayPhase {
    Lobby,
    Selecting {
        team_id: u32,
    },
    Showing {
        clue: (usize, usize),
        owner_team_id: u32,
        attempt_count: u32,
        max_attempts: u32,
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
    #[serde(default)]
    pub event_state: EventState,
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
            event_state: EventState::default(),
        }
    }

    pub fn get_team_by_id(&self, id: u32) -> Option<&Team> {
        self.teams.iter().find(|t| t.id == id)
    }

    pub fn get_available_clues(&self) -> Vec<(usize, usize)> {
        let mut available = Vec::new();
        for (cat_idx, category) in self.board.categories.iter().enumerate() {
            for (clue_idx, clue) in category.clues.iter().enumerate() {
                if !clue.solved {
                    available.push((cat_idx, clue_idx));
                }
            }
        }
        available
    }

    pub fn get_clue(&self, clue: (usize, usize)) -> Option<&Clue> {
        self.board.categories.get(clue.0)?.clues.get(clue.1)
    }

    pub fn is_clue_available(&self, clue: (usize, usize)) -> bool {
        if let Some(c) = self.get_clue(clue) {
            !c.solved
        } else {
            false
        }
    }
}
