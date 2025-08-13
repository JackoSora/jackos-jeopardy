use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigLayoutState {
    BoardView,
    EditorView,
}

#[derive(Clone)]
pub struct BoardEditorTransitionSystem {
    current_state: ConfigLayoutState,
    target_state: ConfigLayoutState,
    transition_progress: f32,
    is_transitioning: bool,
    transition_start: Option<Instant>,
    transition_duration: f32, // in seconds
}

impl BoardEditorTransitionSystem {
    pub fn new() -> Self {
        Self {
            current_state: ConfigLayoutState::BoardView,
            target_state: ConfigLayoutState::BoardView,
            transition_progress: 0.0,
            is_transitioning: false,
            transition_start: None,
            transition_duration: 0.3, // 300ms as specified in requirements
        }
    }

    pub fn update(&mut self) -> bool {
        if !self.is_transitioning {
            return false;
        }

        if let Some(start_time) = self.transition_start {
            let elapsed = start_time.elapsed().as_secs_f32();
            self.transition_progress = (elapsed / self.transition_duration).min(1.0);

            if self.transition_progress >= 1.0 {
                // Transition complete
                self.current_state = self.target_state.clone();
                self.is_transitioning = false;
                self.transition_start = None;
                self.transition_progress = 0.0;
                return true; // Final repaint needed
            }

            return true; // Needs repaint for animation
        }

        false
    }

    pub fn transition_to(&mut self, state: ConfigLayoutState) {
        if self.current_state != state {
            self.target_state = state;
            self.is_transitioning = true;
            self.transition_start = Some(Instant::now());
            self.transition_progress = 0.0;
        }
    }

    pub fn get_current_state(&self) -> &ConfigLayoutState {
        &self.current_state
    }

    pub fn get_transition_progress(&self) -> f32 {
        if self.is_transitioning {
            // Apply smooth easing function
            self.smooth_step(self.transition_progress)
        } else {
            0.0
        }
    }

    pub fn is_transitioning(&self) -> bool {
        self.is_transitioning
    }

    // Smooth step easing function for better visual transitions
    fn smooth_step(&self, t: f32) -> f32 {
        t * t * (3.0 - 2.0 * t)
    }
}

impl Default for BoardEditorTransitionSystem {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transition_system_creation() {
        let system = BoardEditorTransitionSystem::new();
        assert_eq!(system.get_current_state(), &ConfigLayoutState::BoardView);
        assert!(!system.is_transitioning());
    }

    #[test]
    fn test_transition_initiation() {
        let mut system = BoardEditorTransitionSystem::new();

        system.transition_to(ConfigLayoutState::EditorView);
        assert!(system.is_transitioning());
        assert_eq!(system.get_current_state(), &ConfigLayoutState::BoardView); // Still in old state during transition
    }

    #[test]
    fn test_transition_progress() {
        let mut system = BoardEditorTransitionSystem::new();

        system.transition_to(ConfigLayoutState::EditorView);
        let progress = system.get_transition_progress();
        assert!((0.0..=1.0).contains(&progress));
    }
}
