#!/bin/bash

set -euxo pipefail

BINARY=yj

if [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
    # Special handling for Linux: build a static binary
    export CC=clang
    export CXX=clang++
    rustup target add x86_64-unknown-linux-musl
    cargo build --release --target x86_64-unknown-linux-musl
    mv -f "target/x86_64-unknown-linux-musl/release/$BINARY" "target/$BINARY.linux"
elif [[ "$TRAVIS_OS_NAME" == "osx" ]]; then
    cargo build --release
    mv -f "target/release/$BINARY" "target/$BINARY.macos"
elif [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
    cargo build --release
    mv -f "target/release/$BINARY.exe" "target"
else
    cargo build --release
    mv -f "target/release/$BINARY" "target/$BINARY.$TRAVIS_OS_NAME"
fi
