# Conway's Game of Life

A Rust implementation of Conway's Game of Life with a graphical user interface built using egui/eframe.

## Features

- **Interactive GUI**: Modern, responsive interface with real-time controls
- **Mouse Interaction**: Click and drag to draw living cells directly on the grid
- **Rich Presets**: Extensive collection of classic patterns including:
  - **Oscillators**: Blinker, Toad, Beacon, Pulsar, Pentadecathlon
  - **Spaceships**: Glider, LWSS, MWSS, HWSS
  - **Guns**: Gosper Glider Gun, Simkin Glider Gun
  - **Miscellaneous**: R-Pentomino, Diehard, Acorn, Block
- **Configurable Parameters**: Adjustable grid size, update speed, and cell density
- **Generation Tracking**: Real-time display of current generation count
- **Game Controls**: Play/pause, step-by-step execution, clear, and randomize

## Screenshots

The game features a clean interface with:
- Left sidebar for controls and presets
- Central grid area for the game visualization
- Real-time generation counter
- Organized preset categories with descriptions

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/yourusername/game_of_life.git
cd game_of_life
```

2. Build and run:
```bash
cargo run --release
```

## Usage

### Basic Controls

- **Start/Pause**: Begin or pause the automatic evolution
- **Step**: Advance the simulation by one generation
- **Clear**: Remove all living cells from the grid
- **Random**: Populate the grid with random living cells

### Mouse Interaction

- **Click**: Toggle individual cells between alive and dead
- **Click and Drag**: Draw continuous patterns by dragging across the grid

### Configuration

- **Update Speed**: Control simulation speed (1-30 FPS)
- **Grid Size**: Adjust grid dimensions (10-200 width, 10-150 height)
- **Random Density**: Set the probability of cells being alive when randomizing

### Presets

Browse organized categories of classic patterns:
- Select any preset to load it centered on the grid
- Each preset includes a description of its behavior
- Patterns are automatically positioned in the center of the current grid

## Project Structure

```
src/
├── main.rs         # Application entry point and core structure
├── game.rs         # Game logic and Conway's Game of Life rules
├── ui.rs           # User interface rendering and interaction
└── patterns.rs     # Preset pattern definitions
```

## Technical Details

- **Framework**: Built with egui/eframe for cross-platform GUI
- **Architecture**: Modular design with clear separation of concerns
- **Performance**: Optimized for smooth real-time simulation
- **Memory**: Efficient grid representation using Vec<Vec<CellState>>

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes following conventional commits
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Standards

- Follow Rust standard formatting with `cargo fmt`
- Ensure code passes `cargo clippy` lints
- Add tests for new functionality
- Update documentation as needed

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- John Conway for creating the Game of Life
- The egui community for the excellent GUI framework
- Classic pattern contributors and the cellular automata community

## Conway's Game of Life Rules

The Game of Life follows simple rules:

1. **Survival**: A living cell with 2 or 3 neighbors survives
2. **Birth**: A dead cell with exactly 3 neighbors becomes alive
3. **Death**: All other living cells die (underpopulation or overpopulation)

These simple rules create surprisingly complex and beautiful patterns!