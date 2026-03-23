#!/bin/bash
set -ex

cd "$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")"

# Build frontend
cd web
npm install
npm run build
cd ..

# Build Rust binary (embeds frontend static files)
cargo build --release --features embed-frontend

ls -lh target/release/agent-browser-hub
echo "Binary: target/release/agent-browser-hub"
