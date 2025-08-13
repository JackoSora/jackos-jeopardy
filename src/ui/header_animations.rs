use eframe::egui;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum HeaderState {
    Config,
    Game,
}

pub struct HeaderAnimationManager {
    current_state: HeaderState,
    elements: HashMap<String, HeaderElement>,
    needs_repaint: bool,
}

struct HeaderElement {
    text: String,
    position: egui::Pos2,
    alpha: f32,
    color: egui::Color32,
    font_size: f32,
}

impl HeaderAnimationManager {
    pub fn new() -> Self {
        Self {
            current_state: HeaderState::Config,
            elements: HashMap::new(),
            needs_repaint: false,
        }
    }

    pub fn update(&mut self) -> bool {
        let needs_repaint = self.needs_repaint;
        self.needs_repaint = false;
        needs_repaint
    }

    pub fn get_current_state(&self) -> &HeaderState {
        &self.current_state
    }

    pub fn transition_to(&mut self, state: HeaderState) {
        if self.current_state != state {
            self.current_state = state;
            self.needs_repaint = true;
        }
    }

    pub fn update_element(
        &mut self,
        id: String,
        text: String,
        position: egui::Pos2,
        alpha: f32,
        color: egui::Color32,
        font_size: f32,
    ) {
        let element = HeaderElement {
            text,
            position,
            alpha,
            color,
            font_size,
        };
        self.elements.insert(id, element);
        self.needs_repaint = true;
    }

    pub fn render_element(&self, ui: &mut egui::Ui, id: &str) {
        if let Some(element) = self.elements.get(id) {
            let mut color = element.color;
            color[3] = (element.alpha * 255.0) as u8;

            ui.painter().text(
                element.position,
                egui::Align2::LEFT_TOP,
                &element.text,
                egui::FontId::proportional(element.font_size),
                color,
            );
        }
    }
}

impl Default for HeaderAnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_animation_manager_creation() {
        let mut manager = HeaderAnimationManager::new();
        assert_eq!(manager.get_current_state(), &HeaderState::Config);
        assert!(!manager.update()); // Should not need repaint initially
    }

    #[test]
    fn test_header_state_transitions() {
        let mut manager = HeaderAnimationManager::new();
        assert_eq!(manager.get_current_state(), &HeaderState::Config);

        manager.transition_to(HeaderState::Game);
        assert_eq!(manager.get_current_state(), &HeaderState::Game);
        assert!(manager.update()); // Should need repaint after transition
    }

    #[test]
    fn test_header_element_management() {
        let mut manager = HeaderAnimationManager::new();

        manager.update_element(
            "test".to_string(),
            "Test Text".to_string(),
            egui::pos2(0.0, 0.0),
            1.0,
            egui::Color32::WHITE,
            16.0,
        );

        assert!(manager.elements.contains_key("test"));
        assert!(manager.update()); // Should need repaint after element update
    }
}
