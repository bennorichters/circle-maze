#!/bin/bash

set -e

echo "Building WebAssembly module..."

if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack not found. Installing..."
    cargo install wasm-pack
fi

wasm-pack build --target web --out-dir web/pkg

echo "Build complete! WebAssembly module is in web/pkg/"
echo ""
echo "To run the web app locally:"
echo "  cd web"
echo "  python3 -m http.server 8080"
echo ""
echo "Then open http://localhost:8080 in your browser"
