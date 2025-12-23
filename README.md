# Flo4 - Interactive Flow Free Puzzle Solver and Player

<img width="808" height="825" alt="image" src="https://github.com/user-attachments/assets/7ae65d5e-35d6-43a2-b64d-36333352b1a8" />


Flow4 is a comprehensive Flow Free puzzle implementation written in Rust, featuring both an interactive game interface and an advanced constraint satisfaction problem (CSP) solver. The project combines game development, algorithmic puzzle solving, and data acquisition through web scraping.

## Overview

Flow4 implements the classic Flow Free puzzle game where players must connect pairs of colored dots by drawing paths that fill the entire grid without crossing. The project includes:

- **Interactive Game Engine**: Real-time puzzle gameplay with mouse controls and visual feedback
- **Advanced CSP Solver**: Sophisticated backtracking algorithm with constraint propagation
- **Automated Data Acquisition**: Python web scraper for gathering puzzle datasets from online sources
- **Responsive Graphics Engine**: Custom pixel-based rendering system with HSV color generation

The solver can handle puzzles ranging from 5x5 to 15x15 grids and includes over 1,500 puzzle instances scraped from online puzzle repositories.

## Controls

Run the interactive game with: `cargo run --release`

- **Mouse Controls**: Click and drag to draw flow paths between colored endpoints
- **Left Click + Drag**: Create or extend flow paths
- **Right Click**: Clear all current flows and reset puzzle
- **Auto-Progression**: Automatically advances to next puzzle upon completion with 3-second delay

## Technical Implementation

### Game Architecture

The game uses a modular architecture with separate concerns:

- **Board System**: Grid-based representation using enum-based cells (Empty, Path, Head) with efficient indexing and neighbor detection
- **Flow Management**: Dynamic flow tracking with path validation, completion detection, and intelligent cutting/extension logic
- **Interactive Controls**: Real-time mouse input handling with grid coordinate translation and drag state management
- **Visual Feedback**: HSV-based color generation ensuring distinct colors for up to 26 different flows

### Advanced Solver Engine

The solver implements a sophisticated constraint satisfaction approach:

- **Backtracking with Constraint Propagation**: Explores solution space using forced move detection and binary branching
- **Heuristic Optimization**: Prioritizes forced moves (cells with only one valid extension) before exploring multiple possibilities
- **Failure Detection**: Early termination on impossible configurations including blocked endpoints, isolated cells, and unreachable pockets
- **Stack-Based Architecture**: Manages solver states with depth-first search and efficient state restoration

### Puzzle Representation

Puzzles use a compact text format where:

- Uppercase letters represent colored endpoints (heads)
- Lowercase letters represent path segments of the same color
- Each color is mapped to consecutive letters (A/a, B/b, C/c, etc.)

Example 5x5 puzzle:

```
DdBCA
Adbca
adBca
adDCa
aaaaa
```

### Data Acquisition Pipeline

The Python scraper (`flow_stealer.py`) implements:

- **Image Processing**: Downloads puzzle images from FlowFreeSolutions.com and crops to grid boundaries
- **Color Extraction**: Samples center pixels of each grid cell for color identification
- **Head Detection**: Distinguishes endpoints from paths using local pixel uniformity analysis
- **Format Conversion**: Converts RGB color data to letter-based puzzle format with automatic color mapping
- **Batch Processing**: Systematically downloads and processes puzzles across multiple grid sizes (5x5 to 15x15)

## Advanced Features

### Constraint Satisfaction Techniques

The solver employs multiple CSP optimization strategies:

- **Forward Checking**: Validates moves against all constraints before committing
- **Arc Consistency**: Ensures neighboring cells maintain valid relationships
- **Minimal Remaining Values**: Prioritizes cells with fewer valid options
- **Pocket Detection**: Identifies unreachable empty cells that would make solutions impossible

### Graphics Implementation

The rendering system uses the `pixels` crate for efficient graphics:

- **Direct Pixel Manipulation**: Bypasses complex graphics APIs for maximum control and performance
- **Scalable Rendering**: Configurable pixel scaling (40x) with cell subdivision for crisp visuals
- **Dynamic Color Generation**: HSV color space utilization for perceptually distinct flow colors
- **Real-time Updates**: Smooth visual feedback during interactive play

### Performance Optimizations

- **Efficient Board Representation**: Box<[Cell]> for contiguous memory layout and cache efficiency
- **Minimal Allocations**: Reuses data structures and avoids unnecessary memory allocation during solving
- **Early Termination**: Multiple failure detection mechanisms prevent unnecessary computation
- **Batch Processing**: Solver can process entire puzzle sets with performance monitoring

## Dependencies

The project leverages several key Rust crates:

- `pixels` and `winit` for hardware-accelerated graphics and cross-platform windowing
- `hsv` for perceptually uniform color space operations
- `colored` for terminal output formatting during development
- `rand` for puzzle shuffling and testing

Python dependencies include:

- `PIL` for image processing and manipulation
- `numpy` for efficient array operations
- `requests` for web scraping and HTTP operations

## Usage Examples

### Interactive Play

```bash
# Start the interactive game
cargo run --release
```

### Solver Benchmarking

```bash
# Uncomment solver loop in main.rs for batch solving
# Reports solve rate and performance metrics
cargo run --release
```

### Data Collection

```python
# Download puzzle sets for specific grid sizes
python src/flow_stealer.py
# Generates puzzle files in flows/ directory
```

## Future Enhancements

Potential improvements include:

- **Machine Learning Integration**: Neural network move prediction for solver guidance
- **Parallel Solving**: Multi-threaded exploration of solution branches
- **Advanced Graphics**: Smooth path animation and visual effects
- **Puzzle Generation**: Procedural puzzle creation with difficulty analysis
- **Tournament Mode**: Competitive timing and scoring system

## Sources/Inspiration

Puzzle data sourced from FlowFreeSolutions.com with automated scraping pipeline. Solver techniques inspired by: https://mzucker.github.io/2016/08/28/flow-solver.html
Granted, my solver techniques were more rudimentary, as I did not study search-based planners in grad school.
