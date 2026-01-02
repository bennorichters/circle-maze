# Circle Maze Generator

A circular maze generator that creates mazes with configurable complexity.
Available as both a CLI tool and a WebAssembly web application.

## Features

- Generate random circular mazes with customizable number of circles
- Find and highlight the longest path (tree diameter)
- Export to SVG format
- Save/load mazes as JSON
- WebAssembly-powered web interface

## CLI Usage

### Generate a new maze

```bash
cargo run -- --create 5
```

This creates a maze with 5 circles and outputs:
- `maze.svg` - Visual representation with highlighted path
- `maze.json` - Serialized maze data

### Load and render existing maze

```bash
cargo run -- --parse maze.json
```

This loads a maze from JSON and outputs `maze.svg`.

### Options

- `--no-path` - Generate the maze SVG without highlighting the longest path

```bash
cargo run -- --create 5 --no-path
```

## Web Application

### Build WebAssembly module

```bash
./build-wasm.sh
```

Or manually:

```bash
wasm-pack build --target web --out-dir web/pkg
```

### Run locally

```bash
cd web
python3 -m http.server 8080
```

Then open http://localhost:8080 in your browser.

### Features

- Interactive maze generation
- Adjustable complexity (3-20 circles)
- Download SVG for printing or further editing
- Download JSON for sharing or later use

## Dependencies

- Rust (2021 edition)
- wasm-pack (for building web app)

## Project Structure

```
circle-maze/
├── src/
│   ├── main.rs            - CLI entry point
│   ├── lib.rs             - WebAssembly bindings
│   ├── maze.rs            - Maze generation algorithm
│   ├── svg/
│   │   ├── mod.rs         - SVG rendering module
│   │   ├── geometry.rs    - Geometric calculations
│   │   ├── markers.rs     - SVG marker definitions
│   │   ├── borders.rs     - Border rendering
│   │   └── solution_path.rs - Path highlighting
│   ├── circle_coord.rs    - Coordinate system
│   ├── json.rs            - JSON parsing
│   └── merge.rs           - Path merging utilities
├── web/
│   ├── index.html         - Web UI
│   ├── app.js             - JavaScript loader
│   └── pkg/               - Generated WASM files (after build)
├── build.rs               - Cargo build script
└── build-wasm.sh          - WASM build script
```

## Algorithm

The maze generator uses a randomized spanning tree algorithm to ensure:
- Every cell is reachable from any other cell
- There is exactly one path between any two cells
- No loops or isolated regions

The visualization highlights the tree diameter (longest path in the maze).
