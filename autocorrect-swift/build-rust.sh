# build-rust.sh

#!/bin/bash

set -e

THISDIR=$(dirname $0)
TARGET=../target
cd $THISDIR

export SWIFT_BRIDGE_OUT_DIR="$(pwd)/dist"
# Build the project for the desired platforms:
cargo build --target x86_64-apple-darwin
cargo build --target aarch64-apple-darwin
mkdir -p ../target/universal-macos/debug

lipo \
    ${TARGET}/aarch64-apple-darwin/debug/libautocorrect_swift.a \
    ${TARGET}/x86_64-apple-darwin/debug/libautocorrect_swift.a -create -output \
    ${TARGET}/universal-macos/debug/libautocorrect_swift.a

# cargo build --target aarch64-apple-ios

# cargo build --target x86_64-apple-ios
# cargo build --target aarch64-apple-ios-sim
# mkdir -p ./target/universal-ios/debug

# lipo \
#     ./target/aarch64-apple-ios-sim/debug/libmy_rust_lib.a \
#     ./target/x86_64-apple-ios/debug/libmy_rust_lib.a -create -output \
#     ./target/universal-ios/debug/libmy_rust_lib.a