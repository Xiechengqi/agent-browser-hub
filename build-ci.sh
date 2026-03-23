#!/bin/bash

set -ex

BASEPATH=$(dirname "$(readlink -f "${BASH_SOURCE[0]}")") && cd "$BASEPATH"

# Generate build-info.json
echo ""
echo "==> Generating build-info.json..."

VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
[ -z "$VERSION" ] && VERSION="unknown"
COMMIT=$(git rev-parse --short=7 HEAD 2>/dev/null || echo "unknown")
COMMIT_TIMESTAMP=$(git log -1 --format='%ct' 2>/dev/null || echo "0")
COMMIT_DATE=$(date -d "@$COMMIT_TIMESTAMP" '+%Y-%m-%d %H:%M:%S' 2>/dev/null || echo "unknown")
COMMIT_MESSAGE=$(git log -1 --format='%s' 2>/dev/null || echo "unknown")
BUILD_TIME=$(date '+%Y-%m-%d %H:%M:%S')

cat > build-info.json << EOF
{
  "version": "$VERSION",
  "commit": "$COMMIT",
  "commitDate": "$COMMIT_DATE",
  "commitMessage": "$COMMIT_MESSAGE",
  "buildTime": "$BUILD_TIME"
}
EOF

echo "Build info:"
cat build-info.json

# Parse target architecture
TARGET=${1:-amd64}

if [ "$TARGET" != "amd64" ] && [ "$TARGET" != "arm64" ]; then
    echo "Usage: $0 [amd64|arm64]"
    echo "Invalid target: $TARGET"
    exit 1
fi

echo ""
echo "Building for target architecture: $TARGET"

# Build Rust binary with cross-compilation
echo ""
echo "==> Building Rust binary for ${TARGET}..."

if [ "$TARGET" = "arm64" ]; then
    cargo zigbuild --release --target aarch64-unknown-linux-musl --features embed-frontend
    ls -alht target/aarch64-unknown-linux-musl/release/agent-browser-hub
    echo ""
    echo "==> Build completed successfully!"
    echo "Binary location: target/aarch64-unknown-linux-musl/release/agent-browser-hub"
else
    cargo zigbuild --release --target x86_64-unknown-linux-musl --features embed-frontend
    ls -alht target/x86_64-unknown-linux-musl/release/agent-browser-hub
    echo ""
    echo "==> Build completed successfully!"
    echo "Binary location: target/x86_64-unknown-linux-musl/release/agent-browser-hub"
fi

rm -f build-info.json
