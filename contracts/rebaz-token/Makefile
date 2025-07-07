.PHONY: build test clean

build:
	cargo build --package rebaz-token --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/rebaz_token.wasm ./optimized_rebaz_token.wasm

test:
	cargo test --package rebaz-token

clean:
	cargo clean --package rebaz-token
	rm -f optimized_rebaz_token.wasm 