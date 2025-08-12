# UI Event System Integration

This document describes how the event system is integrated into the game's user interface layer, providing rich visual feedback and animations for game events.

## Overview

The UI event integration provides a seamless visual experience for the game's event system through:
- Full-screen event animations with unique visual styles
- Interaction blocking during event presentations
- Smooth animation lifecycle management
- Memory-efficient animation state persistence

## Architecture

### Event Animation Controller Integration

The UI integrates with the event system through the `EventAnimationController` which manages animation state and timing:

```rust
use crate::game::events::{EventAnimationController, GameEvent, EventAnimationType};

// Animation controller stored in UI memory for persistence across frames
let event_animation_id = ui.id().with("event_animation");
let mut event_animation: Option<EventAnimationController> = ui
    .memory_mut(|m| m.data.get_temp(event_animation_id))
    .unwrap_or(None);
```

### Animation Lifecycle Management

The UI handles the complete animation lifecycle:

1. **Detection**: Check for new events that need animation
2. **Initialization**: Create and configure animation controller
3. **Update**: Progress animation timing and phases
4. **Rendering**: Draw animation overlays
5. **Cleanup**: Remove completed animations

```rust
// Check for new event animations
if event_animation.is_none() {
    if let Some(active_event) = &game_engine.get_state().event_state.active_event {
        let mut controller = EventAnimationController::new();
        let duration = match active_event {
            GameEvent::DoublePoints => Duration::from_millis(3000),
            GameEvent::HardReset => Duration::from_millis(4000),
            GameEvent::ReverseQuestion => Duration::from_millis(2500),
            GameEvent::ScoreSteal => Duration::from_millis(3200),
        };
        controller.start_animation(active_event.clone(), duration);
        event_animation = Some(controller);
    }
}

// Update and render active animations
if let Some(mut controller) = event_animation.take() {
    if controller.update() {
        event_animation = None; // Animation completed
    } else {
        // Render animation overlay
        render_event_animation(ui, &controller);
        event_animation = Some(controller);
    }
}
```

## Event Animation Types

### Double Points Animation (`DoublePointsMultiplication`)

**Duration**: 3 seconds  
**Color Scheme**: Cyan/Blue with white highlights  
**Visual Elements**:
- Large "×2" text in center with scaling effect
- Energy bursts radiating from center
- Pulsing rings expanding outward
- Scaling point value particles
- Smooth fade-in/fade-out overlay

**Implementation**:
```rust
fn draw_double_points_animation(painter: &egui::Painter, rect: egui::Rect, t: f32) {
    // Cyan/blue base overlay
    let base_alpha = ((0.7 - ease_out * 0.5) * 255.0) as u8;
    let base_color = egui::Color32::from_rgba_unmultiplied(0, 200, 255, base_alpha);
    painter.rect_filled(rect, 0.0, base_color);

    // Central "×2" text with scaling
    let text_size = 120.0 + ease_in_out * 40.0;
    // ... energy bursts, pulsing rings, particles
}
```

### Hard Reset Animation (`HardResetGlitch`)

**Duration**: 4 seconds  
**Color Scheme**: Red error colors transitioning to normal  
**Visual Elements**:
- Screen glitching and distortion effects
- "RESET" text with digital artifacts
- Static noise and screen tears
- System reboot sequence with loading lines
- Progressive fade from red to normal colors

**Implementation**:
```rust
fn draw_hard_reset_animation(painter: &egui::Painter, rect: egui::Rect, t: f32) {
    // Red error overlay with fade
    let base_color = egui::Color32::from_rgba_unmultiplied(255, 0, 50, base_alpha);
    painter.rect_filled(rect, 0.0, base_color);

    // Screen glitching effect
    if t < 0.6 {
        let glitch_intensity = (0.6 - t) / 0.6;
        // ... glitch rectangles with offset
    }
    
    // "RESET" text and system reboot sequence
    // ... digital artifacts and loading lines
}
```

### Reverse Question Animation (`ReverseQuestionFlip`)

**Duration**: 2.5 seconds  
**Color Scheme**: Purple/Magenta with flowing effects  
**Visual Elements**:
- Flowing data streams in circular patterns
- Text flipping effect ("?" → "!")
- Holographic distortion effects
- Mirror reflection lines
- Smooth purple overlay with particle streams

**Implementation**:
```rust
fn draw_reverse_question_animation(painter: &egui::Painter, rect: egui::Rect, t: f32) {
    // Purple/magenta base overlay
    let base_color = egui::Color32::from_rgba_unmultiplied(150, 0, 255, base_alpha);
    painter.rect_filled(rect, 0.0, base_color);

    // Flowing data streams
    for i in 0..8 {
        // ... circular particle streams
    }
    
    // Flipping text effect and holographic distortion
    // ... "?" to "!" transition with mirror effects
}
```

## Interaction Management

### Blocking User Input

During event animations, the UI blocks user interactions to ensure proper event presentation:

```rust
let interaction_blocked = flash.is_some() || pending_answer.is_some() || event_animation.is_some();

if enhanced_modal_button(ui, "Correct", ModalButtonType::Correct).clicked() 
    && !interaction_blocked 
{
    // Handle button click only when not blocked
}
```

### Animation Overlay Rendering

Event animations are rendered as full-screen overlays with the highest z-order:

```rust
egui::Area::new("event_animation_overlay".into())
    .order(egui::Order::Foreground)
    .movable(false)
    .interactable(false)
    .fixed_pos(rect.min)
    .show(ctx, |ui| {
        let painter = ui.painter_at(rect);
        match animation_type {
            EventAnimationType::DoublePointsMultiplication => {
                draw_double_points_animation(&painter, rect, t);
            }
            EventAnimationType::HardResetGlitch => {
                draw_hard_reset_animation(&painter, rect, t);
            }
            EventAnimationType::ReverseQuestionFlip => {
                draw_reverse_question_animation(&painter, rect, t);
            }
            EventAnimationType::ScoreStealHeist => {
                // Render heist animation using context stored in GameState.event_state.last_steal
                // The UI pulls names and +/- amounts from this context.
            }
        }
    });
```

### Score Steal Animation (`ScoreStealHeist`)

**Duration**: ~3.2 seconds
**Color Scheme**: Green/Gold highlights
**Visual Elements**:
- Dimmed backdrop, money bag traveling from leader to lowest team
- Coin trail, expanding ripples
- Team names rendered under each side and +/- amount labels

**Implementation Notes**:
- Uses `GameState.event_state.last_steal` for `thief_name`, `victim_name`, and `amount`.
- The event is instantaneous in logic; only the animation plays during the transition.

## Memory Management

### Animation State Persistence

Animation controllers are stored in UI memory to persist across frames:

```rust
// Store active animation controller
if let Some(controller) = event_animation {
    ui.memory_mut(|m| m.data.insert_temp(event_animation_id, Some(controller)));
} else {
    ui.memory_mut(|m| m.data.remove::<Option<EventAnimationController>>(event_animation_id));
}
```

### Cleanup and Resource Management

- Animation controllers are automatically cleaned up when animations complete
- Memory is released immediately after animation completion
- No persistent animation state is maintained beyond the animation duration

## Performance Considerations

### Frame Rate Optimization

- Animations use efficient easing functions for smooth performance
- Particle counts are optimized for 60fps rendering
- Complex effects are time-bounded to prevent performance degradation

### Rendering Efficiency

- Full-screen overlays are only rendered during active animations
- Animation calculations are cached within the frame
- Painter operations are batched for optimal performance

## Integration Points

### Game Engine Integration

The UI integrates with the game engine's event system through:

1. **Event Detection**: Monitoring `game_engine.get_state().event_state.active_event`
2. **Effect Processing**: Handling `GameEffect::EventAnimation` effects
3. **State Synchronization**: Coordinating animation timing with game state

### Theme System Integration

Event animations integrate with the existing theme system:

- Color palettes are consistent with the cyberpunk theme
- Animation styles match the overall visual design
- Easing functions provide smooth, professional transitions

## Testing and Validation

### Animation Testing

- Each animation type has been tested for visual quality and timing
- Performance testing ensures smooth 60fps rendering
- Memory usage is validated to prevent leaks

### Integration Testing

- Event triggering is tested across different game scenarios
- Animation blocking is validated for proper user interaction management
- State persistence is tested across UI frame updates

## Future Enhancements

### Potential Improvements

- **Sound Integration**: Add audio effects synchronized with visual animations
- **Customization**: Allow players to adjust animation intensity or disable effects
- **Additional Events**: Framework supports easy addition of new event types
- **Performance Scaling**: Automatic quality adjustment based on system performance

### Extensibility

The animation system is designed for easy extension:

```rust
// Adding a new event animation type
match animation_type {
    EventAnimationType::DoublePointsMultiplication => { /* existing */ }
    EventAnimationType::HardResetGlitch => { /* existing */ }
    EventAnimationType::ReverseQuestionFlip => { /* existing */ }
    EventAnimationType::NewEventType => {
        draw_new_event_animation(&painter, rect, t);
    }
}
```

This architecture provides a solid foundation for rich visual feedback while maintaining clean separation between game logic and presentation layers.