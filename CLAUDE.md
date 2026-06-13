# circle-maze

A circular maze generator in Rust, shipping as a CLI and a WebAssembly web app.
It generates random circular mazes with a configurable number of circles, finds and
highlights the longest path (tree diameter), exports to SVG, and saves/loads mazes as JSON.

## Commands

```bash
cargo build                       # build
cargo test                        # run all tests (inline #[cfg(test)] modules)
cargo run -- --create 5           # generate a 5-circle maze -> maze.svg + maze.json
cargo run -- --parse maze.json    # re-render an existing maze -> maze.svg
cargo run -- --create 5 --no-path # generate without highlighting the longest path
./build-wasm.sh                   # build the WASM module into web/pkg/
```

Run the web app: `./build-wasm.sh`, then `cd web && python3 -m http.server 8080`,
open http://localhost:8080. The web UI offers interactive generation, adjustable
complexity (3-20 circles), and SVG/JSON download.

Requires Rust (2021 edition) and, for the web app, wasm-pack (build-wasm.sh installs it
if missing).

## Algorithm

The generator builds a randomized spanning tree, guaranteeing every cell is reachable
from any other, exactly one path exists between any two cells, and there are no loops or
isolated regions. The visualization highlights the tree diameter (longest path).

## Architecture

```
src/
├── main.rs            CLI entry point (clap), reads/writes files in the working dir
├── lib.rs             wasm-bindgen bindings for the web app
├── maze.rs            generation algorithm, Maze, serialize/deserialize, factory
├── circle_coord.rs    CircleCoord polar coordinate system the maze is built on
├── merge.rs           path merging utilities
├── json.rs            JSON parsing of saved mazes
└── svg/
    ├── mod.rs         rendering entry point (render)
    ├── geometry.rs    geometric calculations
    ├── markers.rs     SVG marker definitions
    ├── borders.rs     border rendering
    └── solution_path.rs  path highlighting
web/
├── index.html         web UI
├── app.js             JavaScript loader
└── pkg/               generated WASM files (after build)
build.rs               cargo build script
build-wasm.sh          WASM build script
tests/fixtures/        saved maze JSON used by tests
```

Two build targets share the crate: the native CLI (`main.rs`) and the wasm32 lib
(`lib.rs`). Code reachable from `lib.rs` must stay wasm-compatible — note the
`cfg(target_arch = "wasm32")` getrandom dependency in Cargo.toml.

## Conventions

- Never write comments.
- Keep lines under 100 characters.
- When adding a dependency, use the latest version.
