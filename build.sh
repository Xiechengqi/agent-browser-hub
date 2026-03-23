#!/bin/bash
set -ex

cd "$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")"

cargo build --release

ls -lh target/release/agent-browser-hub
echo "Binary: target/release/agent-browser-hub"
