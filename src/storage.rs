use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::domain::Board;
use crate::game::GameState;

pub fn save_board_json(path: &Path, board: &Board) -> Result<()> {
    let json = serde_json::to_string_pretty(board)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load_board_json(path: &Path) -> Result<Board> {
    let data = fs::read_to_string(path)?;
    let board: Board = serde_json::from_str(&data)?;
    Ok(board)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub board: Board,
    pub game: Option<GameState>,
}

pub fn autosave_path() -> Option<PathBuf> {
    let dirs = ProjectDirs::from("com", "RustyKrab", "PartyJeopardy")?;
    let dir = dirs.config_dir();
    let _ = fs::create_dir_all(dir);
    Some(dir.join("state.json"))
}

pub fn save_snapshot(snapshot: &Snapshot) -> Result<()> {
    if let Some(path) = autosave_path() {
        let json = serde_json::to_string_pretty(snapshot)?;
        fs::write(path, json)?;
    }
    Ok(())
}

pub fn load_snapshot() -> Result<Option<Snapshot>> {
    if let Some(path) = autosave_path() {
        if path.exists() {
            let data = fs::read_to_string(path)?;
            let snapshot: Snapshot = serde_json::from_str(&data)?;
            return Ok(Some(snapshot));
        }
    }
    Ok(None)
}
