# Rusty Krab - Cyberpunk Party Jeopardy

A cyberpunk-themed Party Jeopardy game built with Rust and egui, featuring dynamic game events, stunning visual effects, and multiplayer support.

## 🎮 Game Overview

Rusty Krab is an interactive Jeopardy-style trivia game with a cyberpunk aesthetic. Teams compete by answering questions across different categories, with special events that can dramatically change the game dynamics.

## 🚀 Installation & Setup

### Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Git**: For cloning the repository

### Installation Steps

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd rusty_krab
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run the game:**
   ```bash
   cargo run --release
   ```

### Development Mode

For development with faster compilation:
```bash
cargo run
```

## 🎯 How to Play

### Game Setup

1. **Launch the game** and you'll start in the lobby
2. **Add teams** by clicking "Add Team" - you can customize team names
3. **Start the game** once you have at least one team
4. **Load a board** with questions and categories (or use the default board)

### Basic Gameplay

1. **Team Selection**: The active team selects a clue from the board
2. **Question Display**: The selected clue's question is displayed
3. **Answer Phase**: The active team attempts to answer
   - ✅ **Correct Answer**: Team gains points equal to the clue value
   - ❌ **Incorrect Answer**: Team loses points, other teams can steal
4. **Steal Phase**: If the active team answers incorrectly, other teams can attempt to steal
5. **Next Turn**: Play continues with the next team

### Scoring System

- **Correct Answer**: +Points (equal to clue value)
- **Incorrect Answer**: -Points (equal to clue value)
- **Successful Steal**: +Points for the stealing team
- **Failed Steal**: No penalty for failed steal attempts

## ⚡ Special Events System

Every **4 questions**, a special event is triggered that can dramatically change the game:

### 🔥 Double Points Event
- **Effect**: The next question is worth **double points** for both correct answers and penalties
- **Animation**: Cyan energy bursts with multiplication effects
- **Strategy**: High risk, high reward - choose your clue wisely!

### 💀 Hard Reset Event
- **Effect**: **All team scores are reset to 0**
- **Animation**: Red glitching effects with system reset visuals
- **Strategy**: Great equalizer - gives trailing teams a fresh start

### 🔄 Reverse Question Event
- **Effect**: The next clue shows the **answer first**, teams must provide the question
- **Animation**: Purple data streams with flipping effects
- **Strategy**: Tests knowledge from a different angle

### Event Timing
- Events trigger after every 4th question is fully resolved
- Animations play during the transition between closing one clue and opening the next
- Events are announced with full-screen cyberpunk-themed animations

## 🎨 Visual Features

- **Cyberpunk Aesthetic**: Neon colors, glitch effects, and futuristic UI
- **Dynamic Animations**: Smooth transitions and visual feedback
- **Event Animations**: Spectacular full-screen effects for special events
- **Responsive Design**: Clean, modern interface that scales well

## 🎮 Controls & Interface

### Main Game Screen
- **Left Panel**: Team list with scores
- **Center**: Game board with categories and clues
- **Click clues**: Select questions to answer
- **Answer buttons**: Mark answers as correct or incorrect

### Game Phases
1. **Lobby**: Add/edit teams, start game
2. **Selecting**: Active team chooses a clue
3. **Showing**: Question is displayed, team answers
4. **Steal**: Other teams attempt to steal if answer was wrong
5. **Resolved**: Question complete, move to next team

## 🏆 Winning the Game

- The game continues until all clues are answered
- The team with the **highest score** at the end wins
- Scores can go negative, so strategy matters!
- Special events can completely change the leaderboard

## 🛠️ Technical Details

### Built With
- **Rust**: Systems programming language for performance and safety
- **egui**: Immediate mode GUI framework for cross-platform support
- **Serde**: Serialization for game state persistence
- **Custom Event System**: Dynamic game events with animation support

### Architecture
- **Game Engine**: Core game logic and state management
- **Event System**: Special events with proper timing and animation control
- **UI System**: Responsive interface with cyberpunk theming
- **Persistence**: Save/load game states

## 🎵 Game Rules Summary

### Core Rules
1. Teams take turns selecting clues from the board
2. Correct answers award points, incorrect answers deduct points
3. Wrong answers allow other teams to steal
4. Game ends when all clues are answered

### Special Event Rules
1. Events trigger every 4 questions automatically
2. Event animations play between clue transitions
3. Hard Reset applies immediately, others affect the next clue
4. Events are randomly selected from available types

### Scoring Rules
- Base points = clue value (100, 200, 300, 400, 500)
- Double Points event: 2x points and penalties
- Negative scores are allowed
- Stealing awards full points to the stealing team

## 🐛 Troubleshooting

### Common Issues

**Game won't start:**
- Ensure Rust is properly installed: `rustc --version`
- Try rebuilding: `cargo clean && cargo build --release`

**Performance issues:**
- Run in release mode: `cargo run --release`
- Close other applications to free up resources

**Display issues:**
- Try different window sizes
- Check graphics drivers are up to date

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.

### Development Setup
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📝 License

This project is open source. Please check the license file for details.

## 🎉 Have Fun!

Enjoy playing Rusty Krab with your friends and family! The combination of classic Jeopardy gameplay with dynamic events and cyberpunk aesthetics creates a unique and exciting party game experience.

---

*Built with ❤️ in Rust*