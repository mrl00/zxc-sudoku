# zxc-sudoku
[![CI](https://github.com/mrl00/zxc-sudoku/actions/workflows/ci.yml/badge.svg)](https://github.com/mrl00/zxc-sudoku/actions/workflows/ci.yml)
[![CI](https://github.com/mrl00/zxc-sudoku/actions/workflows/ci.yml/badge.svg)](https://github.com/mrl00/zxc-sudoku/actions/workflows/ci.yml)
[![CI](https://github.com/mrl00/zxc-sudoku/actions/workflows/ci.yml/badge.svg)](https://github.com/mrl00/zxc-sudoku/actions/workflows/ci.yml)
[![CI](https://github.com/mrl00/zxc-sudoku/actions/workflows/ci.yml/badge.svg)](https://github.com/mrl00/zxc-sudoku/actions/workflows/ci.yml)

Terminal-based Sudoku game built with Rust and ratatui.

## Features

- Automatic puzzle generation with backtracking
- 3 difficulty levels (Easy, Medium, Hard)
- Hint system
- Real-time duplicate detection
- Row/column/block highlighting
- Colorful terminal interface

## Prerequisites

- Rust 2024 or later

## Installation and usage

```bash
# Clone the repository
git clone <repository-url>
cd zxc-sudoku

# Build and run (debug)
cargo run

# Build and run (release, optimized)
cargo run --release
```

## Controls

| Key | Action |
|---|---|
| Arrow keys | Move cursor |
| `1`-`9` | Insert number |
| `Backspace`/`Delete` | Clear cell |
| `H` | Hint (reveal answer) |
| `R` | New game |
| `D` | Change difficulty |
| `Q` / `Esc` | Quit |

## Project structure

```
src/
├── main.rs    # Entry point and event loop
├── app.rs     # Application state
├── game.rs    # Sudoku logic
└── ui.rs      # Terminal rendering
```

## License

MIT