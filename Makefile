.PHONY: build run clean

build:
	cargo component build --bin main --release

run: build
	wasmtime run -S http target/wasm32-wasip1/release/main.wasm

clean:
	cargo clean