# Party Jeopardy!

Fast Rust-powered Jeopardy-style party game built with eframe/egui.

## Installation

Prerequisites:
- Rust (latest stable). Install via https://rustup.rs

Run the game:
1. Clone the repo
2. cargo run --release

(First build will take a few minutes to compile dependencies.)

## Features


- **Enhanced Cyberpunk Theme**: Dark theme with neon colors, dialogue-specific color palette, glow effects, and smooth animations
- **Dynamic Events**: Special gameplay modifiers triggered automatically during play
- **Advanced Visual Effects**: Gradient backgrounds, particle effects, smooth transitions, and performance-optimized rendering
- **Team Management**: Support for multiple teams with comprehensive scoring system
- **Save/Load**: Persistent game state with autosave and manual save functionality

### Event Visual Indicators

Each event has a unique full-screen animation and indicator to help players understand its effect:

- **Double Points**: Cyan/blue overlay with pulsing "×2" symbol, energy bursts, and multiplier icon near scores. Indicates all points (and penalties) are doubled for the next question.
- **Hard Reset**: Matrix-style falling terminal commands with green digital rain effect and "SYSTEM RESET" message. All team scores are instantly reset to zero; no persistent indicator after animation.
- **Reverse Question**: Purple/magenta overlay with flowing data streams, flipping "?"/"!" symbols, and a mirror icon near clue selection. The next clue has its question and answer swapped.
- **Score Steal**: Green/gold overlay with a bag of money moving from the leading team to the lowest team, coin trail, and team names/amounts shown. Indicates the lowest team has stolen 20% of the leader's points (see animation for details).

## Rules

Gameplay loop:
1. Selecting: Current team picks a clue.
2. Showing: Clue dialog opens; team answers.
3. Answer Resolution: Correct (points awarded) or Incorrect (points deducted) animation plays inside the dialog before it closes.
4. Steal: If incorrect, other teams may buzz/attempt (same animation timing applies).
5. Resolved: Scores updated, turn may rotate.
6. Repeat until board cleared; highest score wins.

Events & specials (if enabled) may modify scoring or presentation between clues.

## License

AGPL-3.0. See LICENSE for full text. Provide attribution when using or modifying.
