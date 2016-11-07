release:
	@cargo build --release && node target/asmjs-unknown-emscripten/release/hello-world.js

debug:
	@cargo build && node target/asmjs-unknown-emscripten/debug/hello-world.js