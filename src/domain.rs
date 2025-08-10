use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub clues: Vec<Clue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clue {
    pub id: u32,
    pub points: u32,
    pub question: String,
    pub answer: String,
    pub revealed: bool,
    pub solved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub categories: Vec<Category>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: u32,
    pub name: String,
    pub score: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Surprise {
    DoubleNext,
    ShuffleOneRound,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SurpriseState {
    pub pending: Option<Surprise>,
    pub expires_after_clues: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiMapping {
    // maps logical (cat,row) → visual positions; supports temporary shuffles
    pub logical_to_visual: Vec<(usize, usize)>,
}

impl UiMapping {
    pub fn identity(num_categories: usize, num_rows: usize) -> Self {
        let mut logical_to_visual = Vec::with_capacity(num_categories * num_rows);
        for c in 0..num_categories {
            for r in 0..num_rows {
                logical_to_visual.push((c, r));
            }
        }
        Self { logical_to_visual }
    }

}

impl Default for Board {
    fn default() -> Self {
        Self::default_with_dimensions(6, 5)
    }
}

impl Board {
    pub fn default_with_dimensions(num_categories: usize, num_rows: usize) -> Self {
        let mut categories = Vec::with_capacity(num_categories);
        let mut next_id: u32 = 1;
        for i in 0..num_categories {
            let name = format!("Category {}", i + 1);
            let mut clues = Vec::with_capacity(num_rows);
            for row in 0..num_rows {
                let points = ((row as u32) + 1) * 100;
                clues.push(Clue {
                    id: next_id,
                    points,
                    question: String::new(),
                    answer: String::new(),
                    revealed: false,
                    solved: false,
                });
                next_id += 1;
            }
            categories.push(Category { name, clues });
        }
        Board { categories }
    }
}

#[derive(Debug, Clone)]
pub struct ConfigState {
    pub board: Board,
}
