build: clean
	./build-rust.sh
	swift-bridge-cli create-package \
		--bridges-dir ./dist \
		--out-dir output \
		--macos ../target/universal-macos/debug/libautocorrect_swift.a \
		--name AutoCorrectSwift
clean:
	rm -Rf ./dist ./output
add_target:
	rustup target add x86_64-apple-darwin aarch64-apple-darwin aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
