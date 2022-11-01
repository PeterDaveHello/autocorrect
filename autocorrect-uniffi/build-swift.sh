# build-rust.sh

#!/bin/bash

set -e

THISDIR=$(dirname $0)
TARGET=../target
cd $THISDIR

# Build the project for the desired platforms:
cargo build --target x86_64-apple-darwin --release 
cargo build --target aarch64-apple-darwin --release
mkdir -p ../target/universal-macos/release

lipo \
    ${TARGET}/aarch64-apple-darwin/release/libautocorrect.a \
    ${TARGET}/x86_64-apple-darwin/release/libautocorrect.a -create -output \
    ${TARGET}/universal-macos/release/libAutoCorrectFFI.a

uniffi-bindgen generate src/autocorrect.udl --language swift --out-dir ${TARGET}/universal-macos/release/swift
zip -r -j "libautocorrect-swift.zip" \
	"$TARGET/universal-macos/release/libAutoCorrectFFI.a" \
	"${TARGET}/universal-macos/release/swift"
ls -lha ${TARGET}/universal-macos/release