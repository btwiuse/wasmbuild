all:
	deno task wasmbuild

deno:
	deno run -A main.ts

browser:
	cargo docs -d .

test:
	deno run -A test.js

docs:
	cargo docs -ro

without-wasm-bindgen:
	cargo build --release --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/wasmbuild.wasm  lib/wasmbuild_bg.wasm
	make deno
