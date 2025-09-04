.PHONY: build-agent build-wasm build-all run-agent run-wasm run-wasm-sample clean

build-agent:
	cargo build -p trade-agent --release

build-wasm:
	cargo build -p https-test --target wasm32-wasip2 --release

build-all: build-agent build-wasm

run-wasm: build-wasm
	wasmtime run -S http target/wasm32-wasip2/release/main.wasm

# client: Address, token: Address, amount: u64, trade: BuyOrSell
run-agent: build-agent
	./target/release/trade-agent --client $(client) --token $(token) --amount $(amount) --trade $(trade)

clean:
	cargo clean -p trade-agent -p https-test -p shared
