// Enhanced cell rendering for config mode with visual boundaries and content hierarchy
use eframe::egui;
use std::time::{Duration, Instant};

use crate::theme::{
    animations::{AnimationState, ease_in_out_cubic, smooth_step},
    colors::Palette,
    effects::{GlowConfig, paint_glow_rect, paint_gradient_rect},
    utils::{adjust_brightness, lerp_color, with_alpha},
};

/// Visual state of a config cell
#[derive(Clone, Debug, PartialEq)]
pub enum CellState {
    Empty,
    Editing { field: EditField },
    Filled { question: String, answer: String },
    Hovered,
    Focused,
}

/// Which field is being edited
#[derive(Clone, Debug, PartialEq)]
pub enum EditField {
    Question,
    Answer,
}

/// Enhanced configuration cell with animation support
#[derive(Clone)]
pub struct EnhancedConfigCell {
    state: CellState,
    pub(crate) hover_animation: Option<AnimationState>,
    pub(crate) focus_animation: Option<AnimationState>,
    pub(crate) content_animation: Option<AnimationState>,
    visual_config: CellVisualConfig,
    last_update: Instant,
}

/// Visual configuration for cells
#[derive(Clone, Debug)]
pub struct CellVisualConfig {
    pub border_style: BorderStyle,
    pub background_gradient: GradientConfig,
    pub text_styling: TextStyling,
    pub interaction_config: InteractionConfig,
}

/// Border styling configuration
#[derive(Clone, Debug)]
pub struct BorderStyle {
    pub width: f32,
    pub color: egui::Color32,
    pub hover_color: egui::Color32,
    pub focus_color: egui::Color32,
    pub rounding: f32,
    pub glow_config: Option<GlowConfig>,
}

/// Background gradient configuration
#[derive(Clone, Debug)]
pub struct GradientConfig {
    pub start_color: egui::Color32,
    pub end_color: egui::Color32,
    pub hover_start: egui::Color32,
    pub hover_end: egui::Color32,
    pub focus_start: egui::Color32,
    pub focus_end: egui::Color32,
}

/// Text styling configuration
#[derive(Clone, Debug)]
pub struct TextStyling {
    pub question_font_size: f32,
    pub answer_font_size: f32,
    pub points_font_size: f32,
    pub question_color: egui::Color32,
    pub answer_color: egui::Color32,
    pub points_color: egui::Color32,
    pub placeholder_color: egui::Color32,
}

/// Interaction feedback configuration
#[derive(Clone, Debug)]
pub struct InteractionConfig {
    pub hover_duration: Duration,
    pub focus_duration: Duration,
    pub content_transition_duration: Duration,
    pub hover_scale: f32,
    pub focus_glow_intensity: f32,
}

impl Default for CellVisualConfig {
    fn default() -> Self {
        Self {
            border_style: BorderStyle {
                width: 2.0,
                color: Palette::CYAN,
                hover_color: adjust_brightness(Palette::CYAN, 1.3),
                focus_color: adjust_brightness(Palette::CYAN, 1.5),
                rounding: 8.0,
                glow_config: Some(GlowConfig::cyan_glow(0.3, 4.0)),
            },
            background_gradient: GradientConfig {
                start_color: adjust_brightness(Palette::BG_PANEL, 1.1),
                end_color: adjust_brightness(Palette::BG_PANEL, 0.9),
                hover_start: adjust_brightness(Palette::BG_ACTIVE, 1.2),
                hover_end: adjust_brightness(Palette::BG_ACTIVE, 1.0),
                focus_start: adjust_brightness(Palette::BG_ACTIVE, 1.3),
                focus_end: adjust_brightness(Palette::BG_ACTIVE, 1.1),
            },
            text_styling: TextStyling {
                question_font_size: 14.0,
                answer_font_size: 13.0,
                points_font_size: 16.0,
                question_color: adjust_brightness(Palette::TEXT, 1.1),
                answer_color: adjust_brightness(Palette::TEXT, 0.9),
                points_color: Palette::MAGENTA,
                placeholder_color: adjust_brightness(Palette::TEXT, 0.6),
            },
            interaction_config: InteractionConfig {
                hover_duration: Duration::from_millis(200),
                focus_duration: Duration::from_millis(150),
                content_transition_duration: Duration::from_millis(300),
                hover_scale: 1.02,
                focus_glow_intensity: 0.6,
            },
        }
    }
}

impl EnhancedConfigCell {
    /// Create a new enhanced config cell
    pub fn new() -> Self {
        Self {
            state: CellState::Empty,
            hover_animation: None,
            focus_animation: None,
            content_animation: None,
            visual_config: CellVisualConfig::default(),
            last_update: Instant::now(),
        }
    }

    /// Create with custom visual configuration
    pub fn with_config(config: CellVisualConfig) -> Self {
        Self {
            state: CellState::Empty,
            hover_animation: None,
            focus_animation: None,
            content_animation: None,
            visual_config: config,
            last_update: Instant::now(),
        }
    }

    /// Update cell state and handle transitions
    pub fn update_state(&mut self, new_state: CellState) {
        if self.state != new_state {
            // Start content transition animation if content changed
            if matches!(
                (&self.state, &new_state),
                (CellState::Empty, CellState::Filled { .. })
                    | (CellState::Filled { .. }, CellState::Empty)
                    | (CellState::Filled { .. }, CellState::Filled { .. })
            ) {
                self.content_animation = Some(AnimationState::new(
                    self.visual_config
                        .interaction_config
                        .content_transition_duration,
                    smooth_step,
                ));
                self.content_animation.as_mut().unwrap().start();
            }

            self.state = new_state;
        }
    }

    /// Set hover state with animation
    pub fn set_hovered(&mut self, hovered: bool) {
        if hovered && !matches!(self.state, CellState::Hovered) {
            self.hover_animation = Some(AnimationState::new(
                self.visual_config.interaction_config.hover_duration,
                ease_in_out_cubic,
            ));
            self.hover_animation.as_mut().unwrap().start();

            if !matches!(self.state, CellState::Focused | CellState::Editing { .. }) {
                self.state = CellState::Hovered;
            }
        } else if !hovered && matches!(self.state, CellState::Hovered) {
            self.state = match &self.state {
                CellState::Filled { question, answer } => CellState::Filled {
                    question: question.clone(),
                    answer: answer.clone(),
                },
                _ => CellState::Empty,
            };
        }
    }

    /// Set focus state with animation
    pub fn set_focused(&mut self, focused: bool, field: Option<EditField>) {
        if focused {
            self.focus_animation = Some(AnimationState::new(
                self.visual_config.interaction_config.focus_duration,
                ease_in_out_cubic,
            ));
            self.focus_animation.as_mut().unwrap().start();

            if let Some(field) = field {
                self.state = CellState::Editing { field };
            } else {
                self.state = CellState::Focused;
            }
        } else if matches!(self.state, CellState::Focused | CellState::Editing { .. }) {
            self.state = match &self.state {
                CellState::Editing { .. } | CellState::Focused => {
                    // Determine state based on content
                    CellState::Empty // This should be updated with actual content
                }
                _ => self.state.clone(),
            };
        }
    }

    /// Update animations and return if repaint is needed
    pub fn update_animations(&mut self) -> bool {
        let mut needs_repaint = false;
        let now = Instant::now();

        // Update hover animation
        if let Some(ref mut anim) = self.hover_animation {
            anim.update();
            needs_repaint = true;
            if anim.is_complete() {
                self.hover_animation = None;
            }
        }

        // Update focus animation
        if let Some(ref mut anim) = self.focus_animation {
            anim.update();
            needs_repaint = true;
            if anim.is_complete() {
                self.focus_animation = None;
            }
        }

        // Update content animation
        if let Some(ref mut anim) = self.content_animation {
            anim.update();
            needs_repaint = true;
            if anim.is_complete() {
                self.content_animation = None;
            }
        }

        self.last_update = now;
        needs_repaint
    }

    /// Render the enhanced cell
    pub fn render(
        &self,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        points: u32,
        question: &mut String,
        answer: &mut String,
    ) -> CellResponse {
        let painter = ui.painter_at(rect);

        // Calculate animation values (simplified for now)
        let hover_progress = if matches!(self.state, CellState::Hovered) {
            1.0
        } else {
            0.0
        };
        let focus_progress = if matches!(self.state, CellState::Focused | CellState::Editing { .. })
        {
            1.0
        } else {
            0.0
        };
        let content_progress = 1.0;

        // Apply hover scale
        let scale =
            1.0 + (hover_progress * (self.visual_config.interaction_config.hover_scale - 1.0));
        let scaled_rect = if scale != 1.0 {
            let center = rect.center();
            let scaled_size = rect.size() * scale;
            egui::Rect::from_center_size(center, scaled_size)
        } else {
            rect
        };

        // Render background with state-based colors
        let bg_start = lerp_color(
            lerp_color(
                self.visual_config.background_gradient.start_color,
                self.visual_config.background_gradient.hover_start,
                hover_progress,
            ),
            self.visual_config.background_gradient.focus_start,
            focus_progress,
        );

        let bg_end = lerp_color(
            lerp_color(
                self.visual_config.background_gradient.end_color,
                self.visual_config.background_gradient.hover_end,
                hover_progress,
            ),
            self.visual_config.background_gradient.focus_end,
            focus_progress,
        );

        paint_gradient_rect(
            &painter,
            scaled_rect,
            bg_start,
            bg_end,
            true,
            self.visual_config.border_style.rounding,
        );

        // Render glow effect
        if let Some(glow_config) = &self.visual_config.border_style.glow_config {
            let glow_intensity = glow_config.intensity
                + (focus_progress * self.visual_config.interaction_config.focus_glow_intensity);
            let enhanced_glow = GlowConfig {
                intensity: glow_intensity,
                ..*glow_config
            };
            paint_glow_rect(
                &painter,
                scaled_rect,
                self.visual_config.border_style.rounding,
                enhanced_glow,
            );
        }

        // Render border
        let border_color = lerp_color(
            lerp_color(
                self.visual_config.border_style.color,
                self.visual_config.border_style.hover_color,
                hover_progress,
            ),
            self.visual_config.border_style.focus_color,
            focus_progress,
        );

        let border_width = self.visual_config.border_style.width + (focus_progress * 1.0);
        painter.rect_stroke(
            scaled_rect,
            self.visual_config.border_style.rounding,
            egui::Stroke::new(border_width, border_color),
        );

        // Render content with proper hierarchy
        self.render_content(ui, scaled_rect, points, question, answer, content_progress)
    }

    /// Render cell content with proper visual hierarchy
    fn render_content(
        &self,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        points: u32,
        question: &mut String,
        answer: &mut String,
        content_progress: f32,
    ) -> CellResponse {
        let mut response = CellResponse::default();

        // Content area with padding
        let content_rect = rect.shrink2(egui::vec2(8.0, 6.0));

        // Points display area (left side)
        let points_width = 60.0;
        let points_rect = egui::Rect::from_min_size(
            content_rect.min,
            egui::vec2(points_width, content_rect.height()),
        );

        // Content area (right side)
        let content_area = egui::Rect::from_min_size(
            egui::pos2(content_rect.min.x + points_width + 4.0, content_rect.min.y),
            egui::vec2(
                content_rect.width() - points_width - 4.0,
                content_rect.height(),
            ),
        );

        // Render points with animation
        let points_alpha = (content_progress * 255.0) as u8;
        let points_color = with_alpha(self.visual_config.text_styling.points_color, points_alpha);

        ui.painter().text(
            points_rect.center(),
            egui::Align2::CENTER_CENTER,
            format!("{} pts", points),
            egui::FontId::proportional(self.visual_config.text_styling.points_font_size),
            points_color,
        );

        // Split content area for question and answer
        let question_height = content_area.height() * 0.55;
        let question_rect = egui::Rect::from_min_size(
            content_area.min,
            egui::vec2(content_area.width(), question_height),
        );

        let answer_rect = egui::Rect::from_min_size(
            egui::pos2(
                content_area.min.x,
                content_area.min.y + question_height + 2.0,
            ),
            egui::vec2(
                content_area.width(),
                content_area.height() - question_height - 2.0,
            ),
        );

        // Render question field
        let question_response = ui.put(
            question_rect,
            egui::TextEdit::singleline(question)
                .hint_text("Question")
                .font(egui::FontId::proportional(
                    self.visual_config.text_styling.question_font_size,
                ))
                .text_color(self.visual_config.text_styling.question_color),
        );

        // Render answer field
        let answer_response = ui.put(
            answer_rect,
            egui::TextEdit::singleline(answer)
                .hint_text("Answer")
                .font(egui::FontId::proportional(
                    self.visual_config.text_styling.answer_font_size,
                ))
                .text_color(self.visual_config.text_styling.answer_color),
        );

        // Track which field has focus
        if question_response.has_focus() {
            response.editing_field = Some(EditField::Question);
        } else if answer_response.has_focus() {
            response.editing_field = Some(EditField::Answer);
        }

        response.question_changed = question_response.changed();
        response.answer_changed = answer_response.changed();
        response.hovered = question_response.hovered() || answer_response.hovered();

        response
    }
}

impl Default for EnhancedConfigCell {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from cell rendering
#[derive(Default)]
pub struct CellResponse {
    pub question_changed: bool,
    pub answer_changed: bool,
    pub hovered: bool,
    pub editing_field: Option<EditField>,
}
