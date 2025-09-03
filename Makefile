.PHONY: build-agent build-wasm build-all run-agent run-wasm run-wasm-sample clean

build-agent:
	cargo build -p trade-agent-1 --release

build-wasm:
	cargo build -p https-test --target wasm32-wasip2 --release

build-all: build-agent build-wasm

run-wasm: build-wasm
	wasmtime run -S http crates/wasm-component/target/wasm32-wasip2/release/main.wasm

run-wasm-sample: build-wasm
	wasmtime run -S http crates/wasm-component/target/wasm32-wasip2/release/main.wasm '["0xba2ae424d960c26247dd6c32edc70b295c744c43", "0x8d010bf9c26881788b4e6bf5fd1bdc358c8f90b8", "0xde4ee8057785a7e8e800db58f9784845a5c2cbd6"]'

run-agent-sepolia: build-agent
	./crates/trade-agent-1/target/release/trade-agent-1 0xb1ad5f82407bc0f19f42b2614fb9083035a36b69

clean:
	cargo clean -p trade-agent-1 -p https-test -p shared
