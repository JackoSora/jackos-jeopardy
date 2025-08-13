use eframe::egui;
use std::collections::HashMap;

pub type CellId = (usize, usize); // (column, row)

#[derive(Clone)]
pub struct CellManager {
    cells: HashMap<CellId, EnhancedCell>,
    animation_time: f32,
}

#[derive(Clone)]
pub struct EnhancedCell {
    state: CellState,
    hover_animation: f32,
    focus_animation: f32,
}

#[derive(Debug, Clone)]
pub enum CellState {
    Empty,
    Editing { field: EditField },
    Filled { question: String, answer: String },
}

#[derive(Debug, Clone)]
pub enum EditField {
    Question,
    Answer,
}

pub struct CellResponse {
    pub question_changed: bool,
    pub answer_changed: bool,
    pub needs_repaint: bool,
}

impl CellManager {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            animation_time: 0.0,
        }
    }

    pub fn update_animations(&mut self) -> bool {
        self.animation_time += 0.016; // Assume ~60fps
        let mut needs_repaint = false;

        for cell in self.cells.values_mut() {
            if cell.hover_animation > 0.0 {
                cell.hover_animation = (cell.hover_animation - 0.05).max(0.0);
                needs_repaint = true;
            }
            if cell.focus_animation > 0.0 {
                cell.focus_animation = (cell.focus_animation - 0.05).max(0.0);
                needs_repaint = true;
            }
        }

        needs_repaint
    }

    pub fn update_cell_state(&mut self, id: CellId, question: &str, answer: &str) {
        let state = if question.is_empty() && answer.is_empty() {
            CellState::Empty
        } else {
            CellState::Filled {
                question: question.to_string(),
                answer: answer.to_string(),
            }
        };

        if let Some(cell) = self.cells.get_mut(&id) {
            cell.state = state;
        } else {
            self.cells.insert(
                id,
                EnhancedCell {
                    state,
                    hover_animation: 0.0,
                    focus_animation: 0.0,
                },
            );
        }
    }

    pub fn get_or_create_cell(&mut self, id: CellId) -> &mut EnhancedCell {
        self.cells.entry(id).or_insert_with(|| EnhancedCell {
            state: CellState::Empty,
            hover_animation: 0.0,
            focus_animation: 0.0,
        })
    }

    pub fn handle_cell_response(&mut self, id: CellId, response: CellResponse) {
        if let Some(cell) = self.cells.get_mut(&id) {
            if response.needs_repaint {
                cell.hover_animation = 1.0;
            }
        }
    }

    pub fn cleanup_unused_cells(&mut self, valid_ids: &[CellId]) {
        let valid_set: std::collections::HashSet<_> = valid_ids.iter().collect();
        self.cells.retain(|id, _| valid_set.contains(id));
    }
}

impl EnhancedCell {
    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        points: u32,
        question: &mut String,
        answer: &mut String,
    ) -> CellResponse {
        let mut response = CellResponse {
            question_changed: false,
            answer_changed: false,
            needs_repaint: false,
        };

        // Draw enhanced cell background with border
        let border_color = crate::theme::Palette::CYAN;
        let bg_color = if self.hover_animation > 0.0 {
            crate::theme::Palette::BG_ACTIVE
        } else {
            crate::theme::Palette::BG_PANEL
        };

        ui.painter().rect(
            rect,
            4.0, // rounding
            bg_color,
            egui::Stroke::new(1.0, border_color),
        );

        // Create layout for question and answer
        let question_rect = egui::Rect::from_min_size(
            rect.min + egui::vec2(4.0, 4.0),
            egui::vec2(rect.width() - 8.0, rect.height() * 0.4),
        );
        let answer_rect = egui::Rect::from_min_size(
            rect.min + egui::vec2(4.0, rect.height() * 0.5),
            egui::vec2(rect.width() - 8.0, rect.height() * 0.4),
        );
        let points_rect = egui::Rect::from_min_size(
            rect.min + egui::vec2(4.0, rect.height() * 0.9),
            egui::vec2(rect.width() - 8.0, rect.height() * 0.1),
        );

        // Question field
        let mut question_copy = question.clone();
        let question_response = ui.put(
            question_rect,
            egui::TextEdit::multiline(&mut question_copy)
                .hint_text("Question")
                .font(egui::FontId::proportional(12.0)),
        );
        if question_response.changed() {
            *question = question_copy;
            response.question_changed = true;
        }

        // Answer field
        let mut answer_copy = answer.clone();
        let answer_response = ui.put(
            answer_rect,
            egui::TextEdit::multiline(&mut answer_copy)
                .hint_text("Answer")
                .font(egui::FontId::proportional(12.0)),
        );
        if answer_response.changed() {
            *answer = answer_copy;
            response.answer_changed = true;
        }

        // Points display
        ui.put(
            points_rect,
            egui::Label::new(
                egui::RichText::new(format!("${points}"))
                    .color(crate::theme::Palette::CYBER_YELLOW)
                    .size(10.0),
            ),
        );

        // Handle hover state
        if question_response.hovered() || answer_response.hovered() {
            self.hover_animation = 1.0;
            response.needs_repaint = true;
        }

        response
    }
}

impl Default for CellManager {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_manager_creation() {
        let mut manager = CellManager::new();
        assert_eq!(manager.cells.len(), 0);
        assert!(!manager.update_animations()); // Should not need repaint initially
    }

    #[test]
    fn test_cell_state_management() {
        let mut manager = CellManager::new();
        let cell_id = (0, 0);

        manager.update_cell_state(cell_id, "Question", "Answer");
        let cell = manager.get_or_create_cell(cell_id);

        match &cell.state {
            CellState::Filled { question, answer } => {
                assert_eq!(question, "Question");
                assert_eq!(answer, "Answer");
            }
            _ => panic!("Expected Filled state"),
        }
    }

    #[test]
    fn test_cell_cleanup() {
        let mut manager = CellManager::new();

        // Create some cells
        manager.get_or_create_cell((0, 0));
        manager.get_or_create_cell((1, 1));
        manager.get_or_create_cell((2, 2));
        assert_eq!(manager.cells.len(), 3);

        // Clean up, keeping only (0,0) and (1,1)
        let valid_ids = vec![(0, 0), (1, 1)];
        manager.cleanup_unused_cells(&valid_ids);
        assert_eq!(manager.cells.len(), 2);
        assert!(manager.cells.contains_key(&(0, 0)));
        assert!(manager.cells.contains_key(&(1, 1)));
        assert!(!manager.cells.contains_key(&(2, 2)));
    }
}
