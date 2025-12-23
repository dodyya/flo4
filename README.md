# Flo4 - Interactive Flow Free Puzzle Solver and Player

<img width="808" height="825" alt="image" src="https://github.com/user-attachments/assets/7ae65d5e-35d6-43a2-b64d-36333352b1a8" />

Flow4 is a Flow Free puzzle implementation in Rust, featuring an interactive game and an advanced constraint satisfaction problem (CSP) solver. It includes a Python web scraper for gathering puzzle datasets.

## Usage

**Interactive Play:**
`cargo run --release`

**Controls:**

- **Draw**: Click and drag to connect matching colors.
- **Reset**: Right click.

**Data Collection:**
`python src/flow_stealer.py`

## Technical Details

- **Language**: Rust (Game/Solver), Python (Scraper)
- **Graphics**: `pixels` crate for direct pixel manipulation.
- **Solver**: Backtracking with constraint propagation, forward checking, and heuristic optimization.
- **Puzzle Format**: Text-based, mapping colors to letters (A-Z).
- **Inspiration**: Solver techniques inspired by [Matt Zucker's Flow Solver](https://mzucker.github.io/2016/08/28/flow-solver.html).
