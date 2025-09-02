.PHONY: build run clean run-verbose run-json run-json-pretty run-custom run-json-string run-help

build:
	cargo component build --bin main --release

run: build
	wasmtime run -S http target/wasm32-wasip1/release/main.wasm

# Example: Run with verbose output
run-verbose: build
	wasmtime run -S http target/wasm32-wasip1/release/main.wasm --limit 2 --verbose

# Example: Run with JSON output format
run-json: build
	wasmtime run -S http target/wasm32-wasip1/release/main.wasm --limit 2 --format json

# Example: Run with pretty JSON output format
run-json-pretty: build
	wasmtime run -S http target/wasm32-wasip1/release/main.wasm --limit 2 --format json_pretty

# Example: Run with custom parameters
run-coin: build
	wasmtime run -S http target/wasm32-wasip1/release/main.wasm --verbose --format json --coins bitcoin,ethereum,cardano

# Example: Run with JSON string input
run-json-string: build
	wasmtime run -S http target/wasm32-wasip1/release/main.wasm --string '{"coins": ["bitcoin", "ethereum", "cardano"]}' --format json_pretty

# Example: Run with different API server
run-coinmarketcap: build
	wasmtime run -S http target/wasm32-wasip1/release/main.wasm --api-server coingecko --limit 3 --verbose

# Show help
help:
	wasmtime run -S http target/wasm32-wasip1/release/main.wasm --help

clean:
	cargo clean