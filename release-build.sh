#!/bin/bash

set -euxo pipefail

BINARY=yj
OS_NAME="$TRAVIS_OS_NAME"

if [[ "$OS_NAME" == "linux" ]]; then
    # Special handling for Linux: build a static binary
    export CC=clang
    export CXX=clang++
    rustup target add x86_64-unknown-linux-musl
    cargo build --release --target x86_64-unknown-linux-musl
    mv "target/x86_64-unknown-linux-musl/release/$BINARY" "$BINARY.$OS_NAME"
else
    cargo build --release
    mv "target/release/$BINARY" "$BINARY.$OS_NAME"
fi
