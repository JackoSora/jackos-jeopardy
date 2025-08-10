use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::domain::Board;
use crate::game::GameState;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub board: Board,
    pub game: Option<GameState>,
}


// Manual saves in ./saves directory
pub fn ensure_saves_dir() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    let dir = cwd.join("saves");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn list_saves() -> Result<Vec<PathBuf>> {
    let dir = ensure_saves_dir()?;
    let mut entries: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map(|e| e == "json").unwrap_or(false) {
            entries.push(path);
        }
    }
    entries.sort();
    Ok(entries)
}

pub fn save_snapshot_named(file_stem: &str, snapshot: &Snapshot) -> Result<PathBuf> {
    let dir = ensure_saves_dir()?;
    let safe_name: String = file_stem
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect();
    let name = if safe_name.is_empty() {
        "untitled".to_string()
    } else {
        safe_name
    };
    let path = dir.join(format!("{}.json", name));
    let json = serde_json::to_string_pretty(snapshot)?;
    fs::write(&path, json)?;
    Ok(path)
}

pub fn load_snapshot_from_path(path: &Path) -> Result<Snapshot> {
    let data = fs::read_to_string(path)?;
    let snapshot: Snapshot = serde_json::from_str(&data)?;
    Ok(snapshot)
}
